package com.kaori.compiler.lexer;

import java.util.ArrayList;
import java.util.List;

import com.kaori.error.KaoriError;

public class Lexer {
    private final String source;
    private int index;
    private int line;
    private List<Token> tokens;

    public Lexer(String source) {
        this.source = source;
    }

    private void reset() {
        this.index = 0;
        this.line = 1;
        this.tokens = new ArrayList<>();
    }

    public List<Token> scan() {
        this.reset();
        this.start();

        return tokens;
    }

    private void start() {
        while (!this.atEnd(this.index)) {
            char c = this.source.charAt(this.index);

            if (Character.isWhitespace(c)) {
                this.scanWhiteSpace();
            } else if (Character.isDigit(c)) {
                this.scanNumber();
            } else if (Character.isLetter(c)) {
                this.scanIdentifierOrKeyword();
            } else if (c == '"') {
                this.scanStringLiteral();
            } else if (this.lookAhead("/*", this.index)) {
                this.scanComment();
            } else {
                this.scanSymbol();
            }
        }
    }

    private void advance(int steps) {
        for (int i = 0; i < steps; i++) {
            if (this.source.charAt(this.index) == '\n') {
                this.line++;
            }

            this.index++;
        }
    }

    private boolean atEnd(int current) {
        return current >= this.source.length();
    }

    private boolean lookAhead(String expected, int currentIndex) {
        for (int i = 0; i < expected.length(); i++) {
            int j = currentIndex + i;

            if (j < this.source.length() && expected.charAt(i) == this.source.charAt(j)) {
                continue;
            }

            return false;
        }

        return true;
    }

    private void createToken(TokenKind kind, int size) {
        int position = this.index;

        Token token = new Token(kind, this.line, position, size);

        this.tokens.add(token);

        this.advance(size);
    }

    private void scanComment() {
        int current = this.index + 2;

        while (!this.atEnd(current) && !this.lookAhead("*/", current)) {
            current++;
        }

        current += 2;

        int size = current - this.index;

        this.advance(size);
    }

    private void scanWhiteSpace() {
        int current = this.index;

        while (!this.atEnd(current) && Character.isWhitespace(this.source.charAt(current))) {
            current++;
        }

        int size = current - this.index;

        this.advance(size);
    }

    private void scanNumber() {
        int current = this.index;

        while (!this.atEnd(current) && Character.isDigit(this.source.charAt(current))) {
            current++;
        }

        if (!this.atEnd(current) && this.source.charAt(current) == '.') {
            current++;
        }

        while (!this.atEnd(current) && Character.isDigit(this.source.charAt(current))) {
            current++;
        }

        int size = current - this.index;

        this.createToken(TokenKind.NUMBER_LITERAL, size);
    }

    private void scanIdentifierOrKeyword() {
        int current = this.index;

        while (!this.atEnd(current) && Character.isAlphabetic(this.source.charAt(current))) {
            current++;
        }

        while (!this.atEnd(current)
                && (Character.isLetterOrDigit(this.source.charAt(current)) || this.source.charAt(current) == '_')) {
            current++;
        }

        TokenKind kind = switch (this.source.substring(this.index, current)) {
            case "if" -> TokenKind.IF;
            case "else" -> TokenKind.ELSE;
            case "while" -> TokenKind.WHILE;
            case "for" -> TokenKind.FOR;
            case "break" -> TokenKind.BREAK;
            case "continue" -> TokenKind.CONTINUE;
            case "return" -> TokenKind.RETURN;
            case "def" -> TokenKind.FUNCTION;
            case "print" -> TokenKind.PRINT;
            case "true", "false" -> TokenKind.BOOLEAN_LITERAL;
            default -> TokenKind.IDENTIFIER;
        };

        int size = current - this.index;

        this.createToken(kind, size);
    }

    private void scanStringLiteral() {
        int current = this.index;

        current++;

        while (!this.atEnd(current) && this.source.charAt(current) != '"') {
            current++;
        }

        if (this.atEnd(current) || this.source.charAt(current) != '"') {
            throw KaoriError.SyntaxError("missing closing quotation marks for string literal", this.line);
        }

        current++;

        int size = current - this.index;

        this.createToken(TokenKind.STRING_LITERAL, size);
    }

    private void scanSymbol() {
        TokenKind kind = switch (this.source.charAt(this.index)) {
            case '+' -> this.lookAhead("++", this.index) ? TokenKind.INCREMENT : TokenKind.PLUS;
            case '-' -> this.lookAhead("--", this.index) ? TokenKind.DECREMENT
                    : this.lookAhead("->", this.index) ? TokenKind.THIN_ARROW : TokenKind.MINUS;
            case '&' -> this.lookAhead("&&", this.index) ? TokenKind.AND : TokenKind.INVALID;
            case '|' -> this.lookAhead("||", this.index) ? TokenKind.OR : TokenKind.INVALID;
            case '=' -> this.lookAhead("==", this.index) ? TokenKind.EQUAL : TokenKind.ASSIGN;
            case '!' -> this.lookAhead("!=", this.index) ? TokenKind.NOT_EQUAL : TokenKind.NOT;
            case '>' -> this.lookAhead(">=", this.index) ? TokenKind.GREATER_EQUAL : TokenKind.GREATER;
            case '<' -> this.lookAhead("<=", this.index) ? TokenKind.LESS_EQUAL : TokenKind.LESS;
            case '*' -> TokenKind.MULTIPLY;
            case '/' -> TokenKind.DIVIDE;
            case '%' -> TokenKind.MODULO;
            case '(' -> TokenKind.LEFT_PAREN;
            case ')' -> TokenKind.RIGHT_PAREN;
            case '{' -> TokenKind.LEFT_BRACE;
            case '}' -> TokenKind.RIGHT_BRACE;
            case ',' -> TokenKind.COMMA;
            case ';' -> TokenKind.SEMICOLON;
            case ':' -> TokenKind.COLON;
            default -> TokenKind.INVALID;
        };

        if (kind == TokenKind.INVALID) {
            throw KaoriError.SyntaxError("invalid token " + this.source.charAt(this.index), this.line);
        }

        int size = switch (kind) {
            case INCREMENT,
                    DECREMENT,
                    AND,
                    OR,
                    NOT_EQUAL,
                    EQUAL,
                    GREATER_EQUAL,
                    LESS_EQUAL,
                    THIN_ARROW ->
                2;
            default -> 1;
        };

        this.createToken(kind, size);
    }
}
