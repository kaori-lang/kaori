package com.kaori.compiler.lexer;

public record Token(TokenKind kind, int line, int position, int size) {
}
