#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub charj); // synthesized by LALRPOP

pub mod location;
pub mod token;

#[test]
fn test_charj() {
    assert!(charj::TermParser::new().parse("22").is_ok());
    assert!(charj::TermParser::new().parse("(22)").is_ok());
    assert!(charj::TermParser::new().parse("((((22))))").is_ok());
    assert!(charj::TermParser::new().parse("((22)").is_err());
}
