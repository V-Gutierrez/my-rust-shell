use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

#[allow(unused_must_use)]
fn main() {
    loop {
        // TODO: Improve Prompt
        print!("--->>> ");

        stdout().flush();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        // Remove the newline character from the input
        let command = input.trim();

        #[allow(unused_mut)]
        let mut child = Command::new(command).spawn();

        match child {
            Ok(mut child) => {
                child.wait();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
