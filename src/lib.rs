use crossterm::terminal::enable_raw_mode;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::disable_raw_mode,
};
use std::io::{stdout, Write};
use std::process::exit;

pub fn print_events() {
    let mut print_prompt = true;
    let mut buffer = String::new();
    loop {
        if print_prompt {
            print!("\x1b[32mrustsh\x1b[33m> \x1b[m");
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
                    KeyCode::Char(v) => {
                        parse(lex(v));
                        &buffer.push(v);
                    }
                    KeyCode::Enter => match buffer.as_str() {
                        "exit" => {
                            disable_raw_mode().unwrap();
                            println!();
                            exit(0);
                        }
                        _ => {
                            println!("\r");
                            disable_raw_mode().unwrap();
                            let output = execute_command(&buffer);
                            print!("{}\r", output);
                            enable_raw_mode().unwrap();
                            &buffer.clear();
                            print_prompt = true;
                            print!("\r");
                        }
                    },
                    KeyCode::Backspace => {
                        if buffer.len() != 0 {
                            print!("\x1b[1D\x1b[0K");
                        }
                        &buffer.pop();
                    }
                    _ => {}
                },
            },
            _ => {}
        }
    }
}

pub fn execute_command(cmd: &String) -> String {
    return subprocess::Exec::shell(cmd)
        .stdout(subprocess::Redirection::Merge)
        .capture()
        .unwrap()
        .stdout_str();
}

pub fn parse(token: Token) {
    match token {
        Token::Number(n) => print!("\x1b[31m{}\x1b[m", n),
        Token::CloseParenth(n)
        | Token::OpenParenth(n)
        | Token::Whitespace(n)
        | Token::Charater(n) => print!("{}", n),
        Token::Operator(n) => print!("\x1b[35m{}\x1b[m", n),
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
