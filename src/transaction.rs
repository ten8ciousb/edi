use crate::edi_parse_error::EdiParseError;
use crate::generic_segment::GenericSegment;
use crate::tokenizer::SegmentTokens;
use csv::ReaderBuilder;
use lazy_static::lazy_static;
use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};

/// Represents a transaction in an EDI document. A transaction is initialized with an ST segment
/// and ended with an SE segment.
#[derive(PartialEq, Debug)]
pub struct Transaction<'a> {
    transaction_code: Cow<'a, str>,
    transaction_name: &'static str, // not a Cow because it is a reference to a HashMap value
    transaction_set_control_number: Cow<'a, str>,
    implementation_convention_reference: Option<Cow<'a, str>>,
    segments: VecDeque<GenericSegment<'a>>,
}

// Load the potential transaction schema names from a csv
// source: scraped from https://www.arcesb.com/edi/standards/x12/
lazy_static! {
    static ref SCHEMAS: HashMap<String, String> = {
        let mut map = HashMap::new();
        let schemas_path = format!("{}/resources/schemas.csv", env!("CARGO_MANIFEST_DIR"));
        let mut schemas_csv = ReaderBuilder::new()
            .has_headers(false)
            .from_path(schemas_path)
            .expect("failed to open schemas.csv");
        for record in schemas_csv.records() {
            let record = record.unwrap();
            map.insert(record[0].to_string(), record[1].to_string());
        }
        map
    };
}

impl<'a> Transaction<'a> {
    pub fn parse_from_tokens(input: SegmentTokens<'a>) -> Result<Transaction, EdiParseError> {
        let elements: Vec<&str> = input.iter().map(|x| x.trim()).collect();
        // I always inject invariants wherever I can to ensure debugging is quick and painless,
        // and to check my assumptions.
        edi_assert!(
            elements[0] == "ST",
            "attempted to parse ST from non-ST segment"
        );
        edi_assert!(
            elements.len() >= 3,
            "ST segment does not contain enough elements",
            elements.len()
        );

        let (transaction_code, transaction_set_control_number) =
            (Cow::from(elements[1]), Cow::from(elements[2]));
        let implementation_convention_reference = if elements.len() >= 4 {
            Some(Cow::from(elements[3]))
        } else {
            None
        };
        let transaction_name = if let Some(name) = SCHEMAS.get(&transaction_code.to_string()) {
            name
        } else {
            "unidentified"
        };

        Ok(Transaction {
            transaction_code,
            transaction_name,
            transaction_set_control_number,
            implementation_convention_reference,
            segments: VecDeque::new(),
        })
    }

    /// Enqueue a [GenericSegment] into the transaction.
    pub fn add_generic_segment(&mut self, tokens: SegmentTokens<'a>) {
        self.segments.push_back(
            GenericSegment::parse_from_tokens(tokens)
                .expect("unable to parse generic segment from given tokens"),
        );
    }
}

#[test]
fn construct_transaction() {
    let expected_result = Transaction {
        transaction_code: Cow::from("850"),
        transaction_name: SCHEMAS.get(&"850".to_string()).unwrap(), // should be "Purchase Order"
        transaction_set_control_number: Cow::from("000000001"),
        implementation_convention_reference: None,
        segments: VecDeque::new(),
    };
    let test_input = vec!["ST", "850", "000000001"];

    assert_eq!(
        Transaction::parse_from_tokens(test_input).unwrap(),
        expected_result
    );
}

#[test]
fn spot_check_schemas() {
    assert_eq!(SCHEMAS.get(&"850".to_string()).unwrap(), "Purchase Order");
    assert_eq!(
        SCHEMAS.get(&"100".to_string()).unwrap(),
        "Insurance Plan Description"
    );
    assert_eq!(
        SCHEMAS.get(&"999".to_string()).unwrap(),
        "Implementation Acknowledgment"
    );
}
