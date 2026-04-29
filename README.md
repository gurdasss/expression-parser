# Expression Parser

A modular Rust math expression parser using a Pratt Parser to evaluate single-digit infix arithmetic via a Lexer-AST-Evaluator pipeline. Supports basic operators with precedence and robust error handling in a simple REPL. Focused on clean Rust patterns including enums, match expressions, and comprehensive unit testing.

## Features

- **Lexical Analysis**: Tokenizes input expressions into meaningful tokens (numbers, operators, etc.)
- **Pratt Parser**: Implements top-down operator precedence parsing for correct operator precedence
- **AST Representation**: Builds an abstract syntax tree for expression evaluation
- **Interactive REPL**: Command-line interface with line editing and history
- **Error Handling**: Comprehensive error reporting for invalid input
- **Modular Design**: Clean separation of concerns across lexer, parser, and evaluator components

## Architecture

The parser follows a traditional compiler pipeline:

1. **Lexer** (`src/lexer.rs`): Converts input string to tokens
2. **Parser** (`src/parser.rs`): Builds AST from tokens using Pratt parsing
3. **Evaluator** (planned): Evaluates AST to compute results
4. **REPL** (`src/main.rs`): Interactive interface

## Supported Operations

- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`)
- Single-digit integers (0-9)

## Installation

Ensure you have Rust installed (see [rustup.rs](https://rustup.rs/)).

Clone the repository and build:

```bash
git clone https://github.com/gurdasss/expression-parser.git
cd expression-parser
cargo build --release
```

## Usage

Run the REPL:

```bash
cargo run
```

Enter expressions at the `>> ` prompt:

```
>> 3 + 4 * 2
```

Type `exit` to quit, `help` for assistance.

## Examples

Basic arithmetic:

```
>> 3 + 4
7
>> 5 * 2 - 1
9
```

Operator precedence:

```
>> 3 + 4 * 2
11  # (4 * 2) + 3
```

## Development

### Running Tests

```bash
cargo test
```

### Project Structure

- `src/main.rs`: Entry point and REPL implementation
- `src/lexer.rs`: Lexical analyzer
- `src/parser.rs`: Pratt parser implementation
- `src/expr.rs`: AST expression definitions
- `src/token.rs`: Token type definitions
- `src/error.rs`: Error type definitions

### Contributing

Contributions welcome! Please ensure code follows Rust best practices and includes appropriate tests.

## License

This project is open source. See LICENSE file for details.
