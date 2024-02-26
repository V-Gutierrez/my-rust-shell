use std::{io::stdin, process::Command};

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    // Remove the newline character from the input
    let command = input.trim();
    
    Command::new(command).spawn().unwrap();
}
