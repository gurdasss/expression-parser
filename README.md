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
2. **Parser** (`src/parser.rs`): Builds AST from tokens using Pratt parsing (top-down operator precedence)
3. **Evaluator** (`src/evaluator.rs`): Evaluates AST to compute results with error handling
4. **REPL** (`src/main.rs`): Interactive interface with line editing and history

### Pratt Parser

The parser uses the Pratt parsing algorithm (also known as top-down operator precedence parsing) to handle operator precedence and associativity correctly. Each operator has a binding power that determines how tightly it binds to its operands:

- Addition (`+`) and Subtraction (`-`): Binding power 1 (lowest precedence)
- Multiplication (`*`) and Division (`/`): Binding power 2 (highest precedence)

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

Enter expressions at the `>> ` prompt. The program will parse and evaluate your expressions:

```
>> 3 + 4
7
>> 5 * 2 - 1
9
>> 3 + 4 * 2
11
```

Type `exit` to quit, `help` for assistance.

## Examples

Basic arithmetic operations:

```
>> 3 + 4
7
>> 9 - 5
4
>> 6 * 2
12
>> 8 / 2
4
```

Operator precedence (multiplication and division evaluated before addition and subtraction):

```
>> 3 + 4 * 2
11  # Evaluated as 3 + (4 * 2)
>> 10 - 6 / 2
7   # Evaluated as 10 - (6 / 2)
```

Chained operations:

```
>> 1 + 2 + 3
6
>> 9 - 3 - 2
4
>> 2 * 3 * 4
24
```

Error handling (division by zero):

```
>> 5 / 0
error: division by zero
```

## Development

### Running Tests

```bash
cargo test
```

### Project Structure

- `src/main.rs`: Entry point and REPL implementation
- `src/lexer.rs`: Lexical analyzer (tokenizer)
- `src/parser.rs`: Pratt parser implementation
- `src/evaluator.rs`: Expression evaluator
- `src/expr.rs`: AST expression definitions
- `src/token.rs`: Token type definitions
- `src/error.rs`: Error type definitions (ParseError, EvalError)

## Implementation Status

✅ **Complete**:
- Lexer: Full tokenization of arithmetic expressions
- Parser: Complete Pratt parser with operator precedence
- Evaluator: Full AST evaluation with error handling
- REPL: Interactive command-line interface
- Tests: Comprehensive unit tests for all components

### Test Coverage

The project includes extensive unit tests covering:
- Lexer: Token recognition, whitespace handling, error tokens
- Parser: Single expressions, binary operations, operator precedence, chaining
- Evaluator: Arithmetic operations, precedence correctness, error handling

### Contributing

Contributions welcome! Please ensure code follows Rust best practices and includes appropriate tests.

## License

This project is open source. See LICENSE file for details.
