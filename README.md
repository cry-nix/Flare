![image](https://i.ibb.co/gLmRhxwH/flare-text.png)

**Flare** is a simple but powerful programming language, it has the syntax of python but the power of c++, btw it's written in rust.

---

## Features

* Minimal syntax inspired by modern languages
* Variables: `let` and `var`
* Functions and function calls
* Built-in `print` function
* Abstract Syntax Tree (AST) and interpreter written in Rust
* Extensible: can add GUI, loops, operators, and more in the future

---

## Example Program

```flare
fn main() {
    let x = 42
    print("Hello Flare! Value of x is ", x)
}
```

**Output:**

```
Hello Flare! Value of x is 42
```

---

## Getting Started

### Requirements

* Rust 1.72+
* Cargo (comes with Rust)

### Build & Run

```bash
git clone https://github.com/YOUR_USERNAME/flare.git
cd flare
cargo run
```

You should see the tokens, AST, and program output printed in your terminal.

---

## Project Structure

```
flare/
├── src/
│   ├── ast.rs       # AST definitions
│   ├── lexer.rs     # Lexer / tokenizer
│   ├── parser.rs    # Parser
│   ├── eval.rs      # Interpreter / evaluator
│   └── main.rs      # Entry point
└── Cargo.toml       # Rust project config
```

---

## Future Plans

* Function parameters and return values
* Operators: `+`, `-`, `*`, `/`
* Control flow: `if`, `while`, `for`
* Standard library: strings, math, lists
* GUI package for Flare
* Graphics Library like (OpenGL but easier)

---

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

All contributions are welcome!

---

## License

MIT License © 2026 cry-nix
