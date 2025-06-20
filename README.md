# Yellow Flash ⚡ - Programming Language

Yellow Flash is an interpreted and statically typed small programming language built with Rust


## ✨ Features ✨

- **Statically Typed 🔒:**  
  Catch errors early with a strict type system that prevents bugs and ensures reliability.
- **Interpreted Execution 🚀:**  
  Write and run code instantly without waiting for compilation.
- **Modern, Clean Syntax ✍️:**  
  Intuitive syntax designed for productivity and readability.
- **Error Reporting 🎯:**  
  Clear and friendly runtime error messages.


## 🛠️ Technologies Used 🛠️

- **Rust 🦀** — Core engine for speed and safety.


## 📜 Grammar

```text
program                   -> statement*
statement                 -> block_stmt
                           | expression_stmt
                           | print_stmt
                           | variable_decl_stmt
                           | if_stmt
                           | while_loop_stmt
                           | for_loop_stmt

type                      -> "float" | "str" | "bool"
else                      -> "else" (if_stmt | block_stmt)

block_stmt                -> "{" statement* "}"
expression_stmt           -> expression ";"
print_stmt                -> "print" "(" expression ")" ";"
variable_decl_stmt        -> type identifier "=" expression ";"
if_stmt                   -> "if" "(" expression ")" block_stmt else?
while_loop_stmt           -> "while" "(" expression ")" block_stmt
for_loop_stmt             -> "for" "(" variable_decl_stmt expression ";" expression ")" block_stmt

expression                -> assign
assign                    -> (identifier "=" assign) | logic_or
logic_or                  -> logic_and ( "or" logic_and )*
logic_and                 -> equality ( "and" equality )*
equality                  -> comparison ( ( "!=" | "==" ) comparison )*
comparison                -> term ( ( ">" | ">=" | "<=" | "<" ) term )*
term                      -> factor ( ( "-" | "+" ) factor )*
factor                    -> unary ( ( "/" | "*" ) unary )*
unary                     -> ("-" | "!") unary | primary
primary                   -> "(" expression ")" 
                           | literal 
                           | identifier
```

## 🚀 Getting Started 🚀

### 📋 Prerequisites

- [Rust](https://www.rust-lang.org/) (>= 1.70)
- Cargo (comes bundled with Rust)


### ⬇️ Installation

1. Clone the repository:

    ```bash
    git clone <repository_url>
    cd yellow-flash
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. (Optional) Install globally:

    ```bash
    cargo install --path .
    ```

---

### 🏃 Running Yellow Flash

- Run a script:

    ```bash
    yellow-flash run path/to/your_script.yf
    ```

- Open the interactive REPL:

    ```bash
    yellow-flash repl
    ```

---

### 📦 Building for Production

Build an optimized release binary:

```bash
cargo build --release
```

## 🤝 Contributing 🤝

Contributions are what make the open-source community such an amazing place to learn, inspire, and create.  
Any contributions you make are **greatly appreciated**!

### How to Contribute:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

You can also open issues for bugs, improvements, or feature requests.


## 📄 License 📄

Distributed under the **MIT License**.  
See [`LICENSE`](LICENSE) for more information.
