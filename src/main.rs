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

        // Remove the newline character from the input and split it into an iterator
        let mut user_input = input.trim().split_whitespace();
        // Use the next iteration (e.g. the first word) as the command (e.g. "ls" or "echo")
        let command = user_input.next().unwrap();
        // Use the rest of the iteration (the same variable user_input, this is Iterators behavior) as the command arguments (e.g. "-l" or "hello")
        let command_args = user_input;

        #[allow(unused_mut)]
        let mut child = Command::new(command).args(command_args).spawn();

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
