use rustsh::lex;
use rustsh::Token;

#[test]
pub fn test_lex() {
    assert_eq!(lex(&String::from("h4")), vec![Token::Charater('h'), Token::Number('4')])
}
