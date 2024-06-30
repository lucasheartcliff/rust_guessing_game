# Rust Guessing Game

Welcome to the Rust Guessing Game! This simple command-line game, written in Rust, will challenge you to guess a randomly generated number.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Game Rules](#game-rules)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Overview

The Rust Guessing Game is a fun and interactive command-line application where the computer generates a random number, and the player tries to guess it. After each guess, the game will provide feedback on whether the guess was too high, too low, or correct. The game continues until the player guesses the correct number.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/): Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

### Steps

1. **Clone the Repository**

   ```sh
   git clone https://github.com/lucasheartcliff/rust_guessing_game
   cd rust_guessing_game
   ```

2. **Build the Project**

   ```sh
   cargo build --release
   ```

3. **Run the Game**
   ```sh
   cargo run
   ```

## Usage

To play the game, follow the steps below:

1. **Run the Game**: Execute the game using `cargo run`.
2. **Input Your Guess**: Enter your guess when prompted.
3. **Receive Feedback**: The game will tell you if your guess is too high, too low, or correct.
4. **Continue**: Keep guessing until you find the correct number.

Here's an example session:

```sh
$ cargo run
Guess the number!
Please input your guess.
20
Too small!
Please input your guess.
40
Too big!
Please input your guess.
30
You win!
```

## Game Rules

- The game randomly selects a number within a specified range (default is 1 to 100).
- The player has unlimited attempts to guess the number.
- Feedback is provided after each guess to help the player narrow down their guesses.

## Development

### Prerequisites

- Ensure you have Rust and Cargo installed on your machine.

### Building from Source

To build the game from source, follow the installation steps mentioned above. For development builds, use:

```sh
cargo build
```

### Running Tests

To run tests, use the command:

```sh
cargo test
```

### Changing Game Parameters

You can modify the range of numbers by editing the constants in the `main.rs` file.

```rust
const MIN: u8= 1;
const MAX: u8 = 100;
```

# Acknowledgments

- [The Rust Programming Language](https://www.rust-lang.org/)
- [Rust Guessing Game Tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

Feel free to customize the content to better fit your project or preferences!
