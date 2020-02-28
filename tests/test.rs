use color_lexer::lex;
use color_lexer::Token;

#[test]
pub fn test_lex() {
    assert_eq!(lex('h'), Token::Charater('h'))
}
