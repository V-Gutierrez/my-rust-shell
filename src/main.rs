use std::{io::{stdin, stdout, Write}, process::Command};

#[allow(unused_must_use)]
fn main() {
    loop {
    // TODO: Improve Prompt
    print!(">>> ");

    stdout().flush();

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .unwrap();

    // Remove the newline character from the input
    let command = input.trim();
    
    let mut child = Command::new(command).spawn().unwrap();

    child.wait();
}}
