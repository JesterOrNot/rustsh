use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::env::{set_var, var, var_os};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::Path;
use std::process::exit;

pub fn print_events() {
    let mut cursor_position = 0;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(var("HOME").unwrap().as_str().to_owned() + "/.rustsh/history.txt")
        .unwrap();
    let mut positon =
        lines_from_file(var("HOME").unwrap().as_str().to_owned() + "/.rustsh/history.txt").count();
    let mut buffer = String::new();
    loop {
        // Move to the left, clear line, print prompt
        print!("\x1b[1000D\x1b[0K\x1b[32mrustsh\x1b[33m> \x1b[m");
        // Print buffer
        print_buffer(&buffer);
        // Move to the left and move to the right cursor position
        print!("\x1b[1000D\x1b[{}C", cursor_position + 8);
        stdout().flush().unwrap();
        let event = read().unwrap();
        match event {
            Event::Key(n) => match n {
                crossterm::event::KeyEvent {
                    code: m,
                    modifiers: _,
                } => match m {
                    KeyCode::Char(v) => {
                        &buffer.insert(cursor_position, v);
                        cursor_position += 1;
                    }
                    KeyCode::Backspace => {
                        if cursor_position > 0 {
                            cursor_position -= 1;
                            &buffer.remove(cursor_position);
                        }
                    }
                    KeyCode::Left => {
                        if cursor_position > 0 {
                            cursor_position -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if cursor_position < buffer.len() {
                            cursor_position += 1;
                        }
                    }
                    KeyCode::Up => {
                        if positon > 0 {
                            positon -= 1;
                        }
                        print!("\x1b[1000D\x1b[0K\x1b[32mrustsh\x1b[33m> \x1b[m");
                        buffer = get_command(positon);
                        print!("\x1b[1000D");
                        cursor_position = buffer.len();
                    }
                    KeyCode::Down => {
                        if positon
                            < lines_from_file(
                                var("HOME").unwrap().as_str().to_owned() + "/.rustsh/history.txt",
                            )
                            .count()
                        {
                            positon += 1;
                        }
                        buffer = get_command(positon);
                        cursor_position = buffer.len();
                    }
                    KeyCode::Enter => match buffer.as_str() {
                        "exit" => {
                            disable_raw_mode().unwrap();
                            println!();
                            exit(0);
                        }
                        "" => {
                            println!("\r");
                        }
                        _ => {
                            println!("\r");
                            &file.write(format!("{}\n", buffer).as_bytes()).unwrap();
                            positon += 1;
                            disable_raw_mode().unwrap();
                            let output = execute_command(&buffer);
                            print!("{}\r", output);
                            enable_raw_mode().unwrap();
                            print!("\r");
                            cursor_position = 0;
                            &buffer.clear();
                        }
                    },
                    _ => {}
                },
            },
            _ => {}
        }
    }
}

pub fn init() {
    set_var(
        "rustsh_home",
        format!("{}/.rustsh", var("HOME").unwrap().as_str()),
    );
    create_dir_all(var_os("rustsh_home").unwrap().to_str().unwrap()).unwrap();
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(var("HOME").unwrap().as_str().to_owned() + "/.rustsh/history.txt")
        .unwrap();
}

fn lines_from_file<T: AsRef<Path>>(filename: T) -> impl Iterator<Item = String> {
    let file = File::open(filename);
    let file = match file {
        Ok(n) => n,
        Err(_) => {
            println!("Error! File not found!");
            exit(0);
        }
    };
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line"))
}

fn get_command(n: usize) -> String {
    match lines_from_file(var("HOME").unwrap().as_str().to_owned() + "/.rustsh/history.txt").nth(n)
    {
        Some(n) => n,
        None => "".to_string(),
    }
}

pub fn execute_command(cmd: &String) -> String {
    return subprocess::Exec::shell(cmd)
        .stdout(subprocess::Redirection::Merge)
        .capture()
        .unwrap()
        .stdout_str();
}

fn parse(tokens: Vec<Token>) {
    for i in tokens {
        match i {
            Token::Number(n) => print!("\x1b[1;36m{}\x1b[m", n),
            Token::CloseParenth(n) | Token::OpenParenth(n) => print!("\x1b[1;35m{}\x1b[m", n),
            Token::Whitespace(n) | Token::Charater(n) => print!("{}", n),
            Token::Word(n) => print!("{}", n),
            Token::Operator(n) => print!("\x1b[1;32m{}\x1b[m", n),
            Token::Builtin(n) => print!("\x1b[1;33m{}\x1b[m", n),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Number(char),
    Builtin(String),
    Word(String),
    Whitespace(char),
    Operator(char),
    OpenParenth(char),
    CloseParenth(char),
    Charater(char),
}

pub fn lex(input: &String) -> Vec<Token> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                result.push(Token::Number(c));
                it.next();
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
            'e' => {
                let mut tmp = String::new();
                let mut do_push = false;
                tmp.push(*it.peek().unwrap());
                it.next();
                let target = "xit".chars().collect::<Vec<char>>();
                let mut pos: usize = 0;
                while !it.peek().is_none() && pos < 3 {
                    if target.get(pos).unwrap() != it.peek().unwrap() {
                        break;
                    }
                    if pos == 2 && *target.get(pos).unwrap() == 't' {
                        do_push = true;
                    }
                    tmp.push(*it.peek().unwrap());
                    it.next();
                    pos += 1;
                }
                if do_push {
                    result.push(Token::Builtin(tmp));
                } else {
                    result.push(Token::Word(tmp));
                }
            }
            _ => {
                result.push(Token::Charater(c));
                it.next();
            }
        }
    }
    return result;
}

fn print_buffer(buf: &String) {
    parse(lex(buf));
}
