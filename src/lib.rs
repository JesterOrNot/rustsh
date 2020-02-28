use crossterm::{
    event::{read, Event, KeyCode},
    terminal::disable_raw_mode,
};
use std::io::{stdout, Write};
use std::process::exit;

pub fn print_events() {
    let mut print_prompt = true;
    loop {
        if print_prompt {
            print!(">>> ");
            print_prompt = false;
        }
        stdout().flush().unwrap();
        let event = read().unwrap();
        match event {
            Event::Key(n) => match n {
                crossterm::event::KeyEvent {
                    code: m,
                    modifiers: _,
                } => match m {
                    KeyCode::Char(v) => parse(lex(v)),
                    KeyCode::Enter => {
                        print_prompt = true;
                        println!("\r");
                    },
                    _ => {}
                },
            },
            _ => {}
        }
        if event == Event::Key(KeyCode::Esc.into()) {
            disable_raw_mode().unwrap();
            println!();
            exit(0);
        }
    }
}

pub fn parse(token: Token) {
    match token {
        Token::Number(n) => print!("\x1b[31m{}\x1b[m",n),
        Token::CloseParenth(n) | Token::OpenParenth(n) | Token::Whitespace(n) | Token::Charater(n) => print!("{}", n),
        Token::Operator(n) => print!("\x1b[35m{}\x1b[m", n)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Number(char),
    Whitespace(char),
    Operator(char),
    OpenParenth(char),
    CloseParenth(char),
    Charater(char),
}

#[allow(dead_code)]
pub fn lex(input: char) -> Token {
    match input {
        '0'..='9' => return Token::Number(input),
        '+' | '*' | '-' | '/' => return Token::Operator(input),
        '(' => return Token::OpenParenth(input),
        ')' => return Token::CloseParenth(input),
        ' ' => return Token::Whitespace(input),
        _ => return Token::Charater(input),
    }
}
