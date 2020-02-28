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
                    KeyCode::Char(v) => print!("{}", v),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Number(i32),
    Whitespace(char),
    Operator(char),
    OpenParenth(char),
    CloseParenth(char),
    Charater(char),
}

#[allow(dead_code)]
pub fn lex(input: &String) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                let mut tmp = String::new();
                tmp.push(*it.peek().unwrap());
                it.next();
                while !it.peek().is_none() && ('0'..='9').contains(it.peek().unwrap()) {
                    tmp.push(*it.peek().unwrap());
                    it.next();
                }
                result.push(Token::Number((tmp).parse::<i32>().unwrap()));
            }
            '+' | '*' | '-' | '/' => {
                result.push(Token::Operator(c));
                it.next();
            }
            '(' => {
                result.push(Token::OpenParenth(c));
                it.next();
            }
            ')' => {
                result.push(Token::CloseParenth(c));
                it.next();
            }
            ' ' => {
                result.push(Token::Whitespace(c));
                it.next();
            }
            _ => {
                result.push(Token::Charater(c));
                it.next();
            }
        }
    }
    Ok(result)
}
