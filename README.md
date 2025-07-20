Here’s a clean and professional `README.md` for your `minigrep` Rust project hosted at [github.com/sankeerth125/minigrep](https://github.com/sankeerth125/minigrep).

---

````markdown
# MiniGrep 🔍

A simple command-line tool written in Rust to search for a string in a file — inspired by the classic Unix `grep`.

## 📦 Features

- Search for a string in a text file
- Case-sensitive and case-insensitive search
- Simple and fast, powered by Rust
- Learn-by-building style — part of [The Rust Programming Language Book](https://doc.rust-lang.org/book/)

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (with `cargo`)

### Installation

Clone the repo:

```bash
git clone https://github.com/sankeerth125/minigrep.git
cd minigrep
````

Build the project:

```bash
cargo build --release
```

Run the binary:

```bash
cargo run -- <query> <filename>
```

Example:

```bash
cargo run -- to poem.txt
```

To ignore case:

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```

## 🧪 Running Tests

```bash
cargo test
```

## 📁 Project Structure

```
minigrep/
├── src/
│   ├── lib.rs      # Main logic for search
│   └── main.rs     # CLI entry point
├── Cargo.toml      # Metadata and dependencies
```

## 🛠️ Built With

* [Rust](https://www.rust-lang.org/) - Systems programming language
* [Cargo](https://doc.rust-lang.org/cargo/) - Rust's package manager

## 🧠 Learning Resource

This project is based on Chapter 12 of [The Rust Programming Language Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html). It's a great way to learn about ownership, modules, error handling, and writing tests in Rust.
