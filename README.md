# 🎻 Kaori - Programming Language

**Kaori** is a statically typed programming language, now implemented fully in **Rust**, previously implemented in Java 17

## Features

-   **Statically Typed**  
    Enforces type safety for predictable, and faster runtimes

-   **Detailed Error Messages**  
    Developer-friendly diagnostics for syntax and runtime issues

-   **Implemented Language Features**

    -   [x] Variable declaration (`x: num = 10;`, `x: bool = true;`, `x: str = "Hello world";`)
    -   [x] Assign operators (`x = 5;`)
    -   [x] Logical operators (`&&`, `||`, `!`)
    -   [x] Arithmetic operators (`+`, `-`, `*`, `/`)
    -   [x] Comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)
    -   [ ] Postfix operators (`++`, `--`)
    -   [x] `if / else` statements
    -   [x] `for` loops
    -   [x] `while` loops
    -   [x] block statements for scope (`{ ... }`)
    -   [x] Output with `print` statements
    -   [x] Code comments (`/* this is a comment */`)
    -   [x] Bytecode generation
    -   [x] Virtual machine to interpret bytecode
    -   [ ] Functions
    -   [ ] Function and loop control flow (`break`, `continue`, `return`)
    -   [ ] Native data structures (e.g., lists, maps)
    -   [ ] Constant folding
    -   [ ] Classes and inheritance

## 🛠️ Technologies Used 🛠️

-   **Rust 🦀** — Core engine for speed and safety.

## Grammar

```text
program                  -> declaration* EOF

type                     -> function_type | primitive_type
primitive_type           -> bool | num | str
function_type            -> ( [type [, type]*] ) -> type

declaration              -> variable_declaration
                         | function_declaration | statement

variable_declaration     -> identifier : type (= expression)? ;

function_declaration     -> def identifier ( [variable_declaration*] ) -> type block_statement

statement                -> expression_statement
                         | print_statement
                         | if_statement
                         | while_statement
                         | for_statement
                         | block_statement

expression_statement     -> expression ;

print_statement          -> print ( expression ) ;

if_statement             -> if expression block_statement [else [if_statement | block_statement]]?

while_statement          -> while expression block_statement

for_statement            -> for variable_declaration ; expression ; expression block_statement

block_statement          -> { declaration* }

expression               -> assignment | logic_or

assignment               -> identifier = expression

logic_or                 -> logic_and [|| logic_and]*

logic_and                -> equality [&& equality]*

equality                 -> comparison [[!= | ==] comparison]*

comparison               -> term [[> | >= | < | <=] term]*

term                     -> factor [[+ | -] factor]*

factor                   -> prefix_unary [[* | /] prefix_unary]*

prefix_unary             -> [! | -] unary | primary

primary                  -> number_literal
                         | string_literal
                         | boolean_literal
                         | postfix_unary
                         | ( expression )

postfix_unary            -> [identifier [++ | --]? | function_call]

function_call             -> callee [(expression [, expression]*)]*
```

## Getting Started

### 📋 Prerequisites

-   [Rust](https://www.rust-lang.org/) (>= 1.70)
-   Cargo (comes bundled with Rust)

### ⬇️ Installation

1. Clone the repository:

    ```bash
    git clone <repository_url>
    cd kaori
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. (Optional) Install globally:

    ```bash
    cargo install --path .
    ```

### Building for Production

Build an optimized release binary:

```bash
cargo build --release
```

## Contributing

Contributions are warmly welcome!

### Ways to Contribute:

-   🚨 Report bugs
-   ✨ Propose new features or syntax ideas
-   🧪 Add new test cases
-   📚 Improve the documentation

### Steps:

1. Fork the repo
2. Create a new branch:

    ```bash
    git checkout -b feature/my-feature
    ```

3. Make your changes and commit:

    ```bash
    git commit -m 'feat: add my feature'
    ```

4. Push and open a PR

## 💖 Name Inspiration

The name Kaori is inspired by the character Kaori Miyazono from the anime "Your Lie in April". She represents inspiration, motivation, and the desire to create something different from the standard — the same spirit behind creating this language

## License

Kaori is released under the **MIT License**.  
See the [`LICENSE`](LICENSE) file for more details
