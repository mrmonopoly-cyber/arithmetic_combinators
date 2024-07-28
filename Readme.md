# Arithmetic Combinators

An interpreter written in Rust that uses Interaction Combinator theory to compute basic arithmetic expressions.

## Table of Contents
- [Features](#features)
- [Theory](#theory)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)

## Features

- Supports basic arithmetic operations: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`).
- Handles integer numbers, both positive and negative.
- Includes a Command Line Interface (CLI) that starts automatically when the program is run.

## Theory

This project is based on the Interaction Combinator theory. For more information, refer to the [Interaction Combinator theory paper](https://core.ac.uk/download/pdf/81113716.pdf).

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/mrmonopoly-cyber/arithmetic_combinators.git
    cd arithmetic_combinators
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

Run the interpreter:
```sh
cargo run --release
```

## Examples
```sh
CLI>: 6 + 7
13
CLI>: 7 / 2
3
CLI>: 6 + (-2)
4
CLI>: (-9) * (-9)
81
```
