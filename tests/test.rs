use color_lexer::lex;
use color_lexer::Token;

#[test]
pub fn test_lex() {
    let input = String::from("Hello 123");
    let lexed_out = lex(&input).unwrap();
    assert_eq!(
        lexed_out,
        vec![
            Token::Charater('H'),
            Token::Charater('e'),
            Token::Charater('l'),
            Token::Charater('l'),
            Token::Charater('o'),
            Token::Whitespace(' '),
            Token::Number(123)
        ]
    )
}
