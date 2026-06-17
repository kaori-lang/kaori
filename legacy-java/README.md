# 🎻 Kaori - Programming Language

⚠️ **Note:** This Java implementation is **discontinued**.  
The new version is being rewritten in Rust: [Kaori](https://github.com/Jenseits1/kaori)

## Technologies Used

-   **Java 17** — The entire interpreter is written in modern Java
-   **Maven / Gradle** — Dependency and build management

## Getting Started

### Prerequisites

-   Java 17+
-   Maven or Gradle (recommended for building)

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/kaori-lang.git
    cd kaori-lang
    ```

2. Build the project with Maven:

    ```bash
    mvn package
    ```

    Or with Gradle:

    ```bash
    ./gradlew build
    ```

3. Run the REPL:

    ```bash
    java -jar target/kaori.jar
    ```

## Running Scripts

To run a `.kaori` script:

```bash
java -jar target/kaori.jar path/to/script.kaori
```

To enter the interactive mode (REPL):

```bash
java -jar target/kaori.jar
```
