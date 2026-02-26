

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

* Custom lexer and parser
* AST with precedence-based expression parsing
* `let` and `return` statements
* First-class functions
* Stack-based environment
* Built-in functions and methods
* Error handling system
* Truthy evaluation logic
* Arrays and hash maps
* Script execution support
* Evaluation tests included

---

## 🧱 Supported Data Types

* Integer
* Float
* Boolean
* String
* Array
* Hash Map
* Function
* Null

---

## 📂 Project Structure

```
src/
├── ast/                # Abstract Syntax Tree definitions
│   ├── expression/
│   ├── statement/
│   ├── program.rs
│   ├── precedence.rs
│   └── error.rs
│
├── object/             # Runtime object system
│   ├── built_in/
│   ├── member/
│   ├── array.rs
│   ├── boolean.rs
│   ├── float_obj.rs
│   ├── integer.rs
│   ├── string_obj.rs
│   ├── function.rs
│   ├── return_value.rs
│   ├── error.rs
│   ├── hashmap.rs
│   ├── null.rs
│   ├── stack_environment.rs
│   └── truthy.rs
│
├── script.rs
├── eval_test.rs
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

### Variables

```aloe
let x = 10;
let name = "Aloe";
```

### Functions

```aloe
let add = fn(a, b) {
    return a + b;
};

add(2, 3);
```

### Conditionals

```aloe
if (x > 5) {
    console("Large");
} else {
    console("Small");
}
```

### Arrays

```aloe
let arr = [1, 2, 3];
len(arr);
```

### Loops

```aloe
let found = for i <- range(10){
    if (i == 3){
        continue;
    }
    if (i == 5){
        break true;
    }
}
```
---

## 🧠 Architecture

Aloe follows a traditional interpreter design:

1. Lexer → Tokenizes input
2. Parser → Builds AST
3. Evaluator → Walks AST
4. Environment → Manages scope
5. Object System → Runtime values

---

## 🧪 Testing

```bash
cargo test
```

---

## 🗺 Roadmap

* Improved diagnostics
* Module system
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
