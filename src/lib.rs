use logos::Lexer;
use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use logos::Logos;
use std::env::{current_dir, set_var, var, var_os};
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
        print!(
            "\x1b[1000D\x1b[0K\x1b[36m({}) \x1b[32mrustsh\x1b[33m> \x1b[m",
            current_dir().unwrap().display()
        );
        // Print buffer
        print_buffer(&buffer);
        // Move to the left and move to the right cursor position
        print!(
            "\x1b[1000D\x1b[{}C",
            cursor_position + 11 + current_dir().unwrap().display().to_string().len()
        );
        stdout().flush().unwrap();
        let event = read().unwrap();
        if let Event::Key(n) = event {
            match n {
                crossterm::event::KeyEvent {
                    code: m,
                    modifiers: z,
                } => match m {
                    KeyCode::Char(v) => match z {
                        KeyModifiers::CONTROL => {
                            buffer.clear();
                            cursor_position = 0;
                            match v {
                                'd' => {
                                    disable_raw_mode().unwrap();
                                    println!();
                                    exit(0);
                                }
                                _ => {
                                    println!("^{}", v.to_uppercase());
                                    continue;
                                }
                            }
                        }
                        _ => {
                            buffer.insert(cursor_position, v);
                            cursor_position += 1;
                        }
                    },
                    KeyCode::Backspace => {
                        if cursor_position > 0 {
                            cursor_position -= 1;
                            buffer.remove(cursor_position);
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
                        "help" => {
                            disable_raw_mode().unwrap();
                            println!("\nCommands\n-------\nhelp --- Displays this help message\nexit --- exits the terminal");
                            enable_raw_mode().unwrap();
                            cursor_position = 0;
                            buffer.clear();
                        }
                        "" => {
                            println!("\r");
                        }
                        _ => {
                            println!("\r");
                            file.write_all(format!("{}\n", buffer).as_bytes()).unwrap();
                            positon += 1;
                            disable_raw_mode().unwrap();
                            let output = execute_command(&buffer);
                            print!("{}\r", output);
                            enable_raw_mode().unwrap();
                            print!("\r");
                            cursor_position = 0;
                            buffer.clear();
                        }
                    },
                    _ => {}
                },
            }
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

pub fn execute_command(cmd: &str) -> String {
    subprocess::Exec::shell(cmd.to_string())
        .stdout(subprocess::Redirection::Merge)
        .capture()
        .unwrap()
        .stdout_str()
}

fn parse(mut tokens: Lexer<Token, &str>) {
    while tokens.token != Token::End {
        match tokens.token {
            Token::Number => print!("\x1b[1;36m{}\x1b[m", tokens.slice()),
            Token::StringLiteral => print!("\x1b[36m{}\x1b[m", tokens.slice()),
            Token::CloseParenth | Token::OpenParenth => print!("\x1b[1;35m{}\x1b[m", tokens.slice()),
            Token::Operator => print!("\x1b[1;32m{}\x1b[m", tokens.slice()),
            Token::Exit | Token::Help => print!("\x1b[1;33m{}\x1b[m", tokens.slice()),
            _ => print!("{}", tokens.slice())
        }
        tokens.advance();
    }
}

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
pub enum Token {
    #[end]
    End,
    #[error]
    Error,
    #[regex = "[0-9]+"]
    Number,
    #[token = "exit"]
    Exit,
    #[regex = "[a-zA-Z]+"]
    Word,
    #[token = " "]
    Whitespace,
    #[regex = "[\\+-/\\*]"]
    Operator,
    #[token = "("]
    OpenParenth,
    #[token = "help"]
    Help,
    #[token = ")"]
    CloseParenth,
    #[regex = "\"[^\"]*\"|\'[^\']*\'"]
    StringLiteral
}

fn print_buffer(buf: &str) {
    parse(Token::lexer(buf));
}
