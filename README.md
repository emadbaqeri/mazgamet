# mazgamet 🐒🦀

Welcome to `mazgamet`! This project is a Rust implementation of the interpreter for the **Maz** language, heavily inspired by the **Monkey** language featured in Thorsten Ball's fantastic book, ["Writing An Interpreter In Go"](https://interpreterbook.com/).

While the book uses Go, this project serves as an exercise in applying the same concepts (lexer, parser, Abstract Syntax Tree, evaluator, REPL) using the Rust programming language. Think of it as Monkey's Rust-loving cousin, Maz!

## About the Maz Language 🗣️

Maz aims to mirror the features and simplicity of the Monkey language described in the book. This includes:

*   C-like syntax
*   Variable bindings (`let`)
*   Integers and Booleans
*   Arithmetic expressions
*   Prefix and infix operators
*   Conditionals (`if`/`else`)
*   Function definitions and application (closures)
*   Return statements
*   *(Potentially more features as the project progresses, like strings, arrays, hash maps)*

The primary goal is educational – to understand the mechanics of building an interpreter from scratch.

## Project Status 🛠️

This is a learning project and likely a work-in-progress, following the structure and chapters of the ["Writing An Interpreter In Go"](https://interpreterbook.com/) book. Expect rough edges and incomplete features! It's not production-ready, but hopefully, it's a fun exploration of interpreters and Rust.

## Getting Started 🚀

You can build and run the `mazgamet` interpreter (which likely includes a REPL) using standard Rust tooling.

### Prerequisites

*   **Rust:** Ensure you have the Rust toolchain installed. Get it from [rustup.rs](https://rustup.rs/).

### Building

1.  **Clone the repository:**
    ```bash
    git clone <your-repository-url>
    cd mazgamet
    ```
2.  **Build the project:**
    ```bash
    cargo build
    ```
    For potentially better performance:
    ```bash
    cargo build --release
    ```

### Running the REPL

To start the Read-Eval-Print Loop (REPL) and interact with the Maz language:

*   **Development build:**
    ```bash
    cargo run
    ```
*   **Release build (after building with `--release`):**
    ```bash
    ./target/release/mazgamet
    ```

Once the REPL starts (look for a prompt like `>>`), you can type Maz code:

```maz
>> let add = fn(x, y) { x + y };
>> add(5, 10);
15
>> let message = "Hello, Maz!"; // If strings are implemented
>> message
"Hello, Maz!"
```

## Goals 🎉

*   Learn the fundamentals of interpreter design and implementation.
*   Solidify understanding of Rust by applying it to a non-trivial project.
*   Translate the Go implementation patterns from the book into idiomatic Rust.
*   Have fun building a programming language!

## Contributing 🤝

As this is primarily a learning exercise following the book, contributions aren't the main focus. However, if you spot bugs, have suggestions for more idiomatic Rust, or want to discuss interpreter concepts, feel free to open an issue!

## License 📝

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
