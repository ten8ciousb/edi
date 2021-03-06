extern crate edi;
use edi::parse;
// For tests that check behavior of private fields or structs, or individual unit tests, I put the tests in
// the same file as the struct/function being tested. This avoids unnecessary `pub` at the cost of messier test organization.
// I think the trade-off is worth it, and the organizational loss is not that bad.
// I reserve this file for E2E and integration tests.
#[test]
fn full_parse_test() {
    let input = "ISA*00*          *00*          *ZZ*SENDERISA      *14*0073268795005  *020226*1534*U*00401*000000001*0*T*>~
GS*PO*SENDERGS*007326879*20020226*1534*1*X*004010~
ST*850*000000001~
BEG*00*SA*A99999-01**19970214~
REF*VR*54321~
ITD*01*3*1**15**16~
DTM*002*19971219~
DTM*002*19971219~
N1*BT*BUYSNACKS INC.*9*1223334444~
N3*P.O. BOX 0000~
N4*TEMPLE*TX*76503~
N1*ST*BUYSNACKS PORT*9*1223334445~
N3*1000 N. SAMPLE HIGHWAY~
N4*ATHENS*GA*30603~
PO1**16*CA*12.34**CB*000111111*UA*002840022222~
PID*F****CRUNCHY CHIPS LSS~
PO4*48*7.89*LB~
PO1**13*CA*12.34**CB*000555555*UA*002840033333~
PID*F****NACHO CHIPS LSS~
PO4*48*8.9*LB~
PO1**32*CA*12.34**CB*000666666*UA*002840044444~
PID*F****POTATO CHIPS~
PO4*72*6.78*LB~
PO1**51*CA*12.34**CB*000874917*UA*002840055555~
PID*F****CORN CHIPS~
PO4*48*8.9*LB~
PO1**9*CA*12.34**CB*000874958*UA*002840066666~
PID*F****BBQ CHIPS~
PO4*48*4.5*LB~
PO1**85*CA*12.34**CB*000874990*UA*002840077777~
PID*F****GREAT BIG CHIPS LSS~
PO4*48*4.56*LB~
PO1**1*CA*12.34**CB*000875088*UA*002840088888~
PID*F****MINI CHIPS LSS~
PO4*48*4.56*LB~
CTT*7~
SE*35*000000001~
GE*1*1~
IEA*1*000000001~";

    let edi_document = parse(input).unwrap();
    // Spot check some properties in the document
    assert_eq!(edi_document.interchanges.len(), 1);
    assert_eq!(edi_document.interchanges[0].functional_groups.len(), 1);
    assert_eq!(
        edi_document.interchanges[0].interchange_control_number,
        "000000001"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].date,
        "20020226"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].transaction_name,
        "Purchase Order"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[0]
            .segment_abbreviation,
        "BEG"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[1]
            .segment_abbreviation,
        "REF"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[6]
            .segment_abbreviation,
        "N3"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[32]
            .segment_abbreviation,
        "CTT"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[32]
            .elements
            .len(),
        1
    );
}
#[test]
fn empty_segments() {
    let input = "ISA*  *          *  *          *ZZ*SENDERISA      *14*0073268795005  *020226*1534*U*00401*000000001*0*T*>~
GS*PO*SENDERGS*007326879*20020226*1534*1*X*004010~
ST*850*000000001~
BEG*****~
TEST*****~
ANY_AMOUNT_OF_CHARS_IS_OKAY_ALTHOUGH_ATYPICAL*****~
lowercase is chill***********************~
BEG*****~
BEG*****~
BEG*****~
BEG*****~
SE*10*000000001~
GE*1*1~
IEA*1*000000001~";

    let edi_document = parse(input).unwrap();
    // Spot check some properties in the document
    assert_eq!(edi_document.interchanges.len(), 1);
    assert_eq!(edi_document.interchanges[0].functional_groups.len(), 1);
    assert_eq!(
        edi_document.interchanges[0].interchange_control_number,
        "000000001"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].date,
        "20020226"
    );

    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[0]
            .segment_abbreviation,
        "BEG"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[0]
            .elements
            .len(),
        5
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[3]
            .segment_abbreviation,
        "lowercase is chill"
    );
    assert_eq!(
        edi_document.interchanges[0].functional_groups[0].transactions[0].segments[3]
            .elements
            .len(),
        23
    );
}
