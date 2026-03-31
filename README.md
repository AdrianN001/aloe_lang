

# 🌿 Aloe Language 🌿


<p align="center">
  <strong>A lightweight interpreted programming language written in Rust</strong>
</p>

<p align="center">

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.1.0-green)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)

</p>

---

## 🚀 Overview

**Aloe** is a lightweight interpreted programming language implemented in **Rust**.
It includes a full interpreter pipeline with a lexer, parser (AST-based), evaluator, object system, and built-in functions.

This project is designed to be:

* 🧠 Educational
* 🧩 Modular and extensible
* ⚡ Lightweight
* 🦀 Idiomatic Rust

---

## ✨ Features

*   **Custom Lexer & Parser**: A hand-written lexer and precedence-climbing (Pratt) parser that produces a clean AST.
*   **Core Language Constructs**: Support for `let` bindings, `return`, `if/elif/else`, and `for` loops.
*   **First-Class Functions**: Functions are objects and can be assigned to variables, passed as arguments, and returned from other functions, enabling closures.
*   **Rich Data Types**: Built-in support for integers, floats, booleans, strings, arrays, and hash maps.
*   **Comprehensive Object System**: Includes built-in methods for primary data types (e.g., `string.length`, `array.push()`).
*   **Error Handling**: Robust error and panic handling with stack traces.
*   **Module System**: Support for importing code from other files using `import { ... } from "..."`.
*   **Interactive REPL**: A Read-Eval-Print Loop for interactive coding and experimentation.
*   **Script Execution**: Run `.aloe` files directly from the command line.

---

## 🧱 Supported Data Types

*   Integer
*   Float
*   Boolean
*   String
*   Array
*   Hash Map
*   Iterator
*   Function
*   Structs
*   Null

---

## 📂 Project Structure

```
src/
├── ast/                # Abstract Syntax Tree definitions and parser logic
│   ├── expression/     # Expression node types (Infix, Prefix, Call, etc.)
│   └── statement/      # Statement node types (Let, Return, Struct, etc.)
│
├── evaluator/          # Logic for evaluating AST nodes
│
├── lexer.rs            # Converts source code into a stream of tokens
│
├── object/             # Defines the runtime object system and built-in functionality
│   ├── built_in/       # Implementations for global functions (print, len, etc.)
│   ├── member/         # Implementations for type methods (array.map, string.split)
│   ├── operation/      # Operator overloading for different types
│   └── stack_environment.rs # Handles variable scoping and closures
│
├── main.rs             # Entry point for REPL and script execution
└── repl.rs             # REPL implementation
```

---


## 🛠 Installation

Make sure Rust is installed:

```bash
rustc --version
```

If not:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 🔨 Build

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

---

## ▶ Run

```bash
cargo run
```

Or run a script:

```bash
cargo run -- path/to/script.aloe
```

---

## 📜 Example Syntax

### Variables & Functions

```aloe
# Define a variable
let name = "Aloe";

# Define a function
let add = fn(a, b) {
    return a + b;
};

# Call the function
let result = add(2, 3);
print(result); # Outputs: 5
```

### Conditionals

```aloe
let x = 10;
if (x > 5) {
    print("x is large");
} else {
    print("x is small");
}
```

### Loops

The `for` loop works with any iterable object, such as ranges, arrays, or strings.

```aloe
// Loop over a range
let result = for i <- range(10) {
    if (i == 3) {
        continue; # Skip the rest of this iteration
    }
    if (i == 5) {
        break "Found it!"; # Exit the loop and return a value
    }
}
print(result); # Outputs: Found it!
```

### Arrays & Hash Maps

```aloe
# Array literal and methods
let list = [1, 2, 3];
let squares = list.map(fn(n) { return n * n; });
print(squares); # Outputs: [1, 4, 9]

# Hash map literal
let person = {"name": "James", "age": 30};
print(person["name"]); # Outputs: James
```

### Structs

```aloe
struct Car {
    color;

    fun get_color(this) {
        return this.color;
    }

    fun set_color(this, c) {
        this.color = c;
    }
}

let c = Car("red");
c.set_color("yellow")!;
print(c.get_color()); # Outputs: yellow
```

---

## 🧠 Architecture

Aloe follows a classic interpreter design pattern:

1.  **Lexer (`lexer.rs`)**: The input source code string is fed into the lexer, which tokenizes it into a sequence of tokens (e.g., `KwLet`, `Identifier`, `Assign`).
2.  **Parser (`ast.rs`)**: The stream of tokens is parsed into an Abstract Syntax Tree (AST), which is a hierarchical representation of the code's structure. The parser uses precedence rules to correctly handle complex expressions.
3.  **Evaluator (`evaluator.rs`)**: The evaluator walks the AST, node by node, executing the program's logic.
4.  **Environment (`object/stack_environment.rs`)**: A stack-based environment manages variable scope. Nested scopes (like in function calls) are created by enclosing the parent environment, enabling support for closures.
5.  **Object System (`object/`)**: During evaluation, all runtime values are represented as `Object` variants (e.g., `Object::Int`, `Object::String`). This allows for dynamic typing and a unified way to handle all values.

---

## 🧪 Testing

```bash
cargo test
```

---

## 🗺 Roadmap

* Improved diagnostics
* Expanded standard library
* Optional bytecode backend

---

## 🤝 Contributing

Pull requests are welcome!
If you'd like to contribute, feel free to fork and open a PR.

---

## 📄 License

MIT License

---
