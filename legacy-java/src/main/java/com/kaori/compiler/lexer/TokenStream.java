package com.kaori.compiler.lexer;

import java.util.List;

import com.kaori.error.KaoriError;

public class TokenStream {
    private final List<Token> tokens;
    private String source;
    private int index;
    private int line;

    public TokenStream(List<Token> tokens, String source) {
        this.tokens = tokens;
        this.source = source;
        this.index = 0;
        this.line = 1;
    }

    public TokenKind getCurrent() {
        if (this.atEnd()) {
            return TokenKind.EOF;
        }

        Token token = this.tokens.get(this.index);

        return token.kind();
    }

    public int getLine() {
        Token token = this.tokens.get(this.index);

        return token.line();
    }

    private int getPosition() {
        Token token = this.tokens.get(this.index);

        return token.position();
    }

    private int getSize() {
        Token token = this.tokens.get(this.index);

        return token.size();
    }

    public String getLexeme() {
        return this.source.substring(this.getPosition(), this.getPosition() + this.getSize());
    }

    public boolean atEnd() {
        return this.index >= this.tokens.size();
    }

    private void advance() {
        this.index++;

        if (!this.atEnd()) {
            this.line = this.getLine();
        }
    }

    public void consume(TokenKind expected) {
        if (this.getCurrent() == TokenKind.EOF) {
            throw KaoriError.SyntaxError(
                    String.format("expected %s, but had an unexpected end of file", expected),
                    this.line);
        }

        if (this.getCurrent() != expected) {
            throw KaoriError.SyntaxError(String.format("expected %s instead of %s", expected, this.getCurrent()),
                    this.line);
        }

        this.advance();
    }

    public void consume() {
        this.advance();
    }

    public boolean lookAhead(TokenKind... expected) {
        for (int i = 0; i < expected.length; i++) {
            int j = this.index + i;

            if (j >= this.tokens.size()) {
                return false;
            }

            Token token = this.tokens.get(j);

            if (token.kind() != expected[i])
                return false;
        }

        return true;
    }
}
