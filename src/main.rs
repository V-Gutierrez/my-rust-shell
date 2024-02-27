use std::{
    env::{self},
    io::{stdin, stdout, Write},
    path::Path,
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

        match command {
            // cd command must be a built in (https://unix.stackexchange.com/a/38809)
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = command_args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);

                if let Err(e) = env::set_current_dir(root) {
                    eprintln!("{}", e);
                }
            }
            command => {
                #[allow(unused_mut)]
                let mut child = Command::new(command).args(command_args).spawn();

                match child {
                    Ok(mut child) => {
                        child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
