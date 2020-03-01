use rustsh::lex;
use rustsh::Token;

#[test]
pub fn test_lex() {
    assert_eq!(lex('h'), Token::Charater('h'))
}
