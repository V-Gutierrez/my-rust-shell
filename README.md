<p align="center">
   <video width="600" height="400" controls>
      <source src="https://github.com/V-Gutierrez/my-rust-shell/assets/62355596/7d5f6f23-291b-4774-a84e-94aa301b5b2d" type="video/mp4">
</video>
</p>

# My Rust Shell

A simplistic, yet functional shell implemented in Rust, offering a minimalistic approach to command execution and directory navigation with a visually appealing, colorized prompt.

## Features

- **Execute System Commands**: Allows for executing a wide range of system commands.
- **Built-in `cd` Command**: Native support for changing the current working directory.
- **Colorized Prompt**: A user-friendly prompt that displays the operating system, current user, and working directory in different colors.

## Getting Started

### Prerequisites

You'll need Rust installed on your system. Follow the [official Rust installation guide](https://www.rust-lang.org/tools/install) if necessary.

### Installation

1. Clone this repository:
   ```sh
   git clone <repository-url>
   ```
2. Navigate to the repository's directory.

3. Build the project:
   ```sh
   cargo build
   ```

4. Run the shell:
   ```sh
   cargo run
   ```

## Usage

After launching `my-rust-shell`, you can use it just like any other shell. Here are some basic command examples:

- Change directory:
  ```sh
  cd /path/to/directory
  ```
- List files in the current directory:
  ```sh
  ls
  ```
- Print the current working directory:
  ```sh
  pwd
  ```
- Display environment variables:
  ```sh
  printenv
  ```
- Read the contents of a file:
  ```sh
  cat file.txt
  ```
- To exit the shell, type:
  ```sh
  exit
  ```

## Development

This shell serves as a foundation. There's ample room for expansion, such as adding more built-in commands, improving error handling, and integrating features like command history and auto-completion. Contributions to enhance `my-rust-shell` are highly appreciated.

## License

This project is licensed under the GNU GENERAL PUBLIC LICENSE License - see the `LICENSE` file for details.
