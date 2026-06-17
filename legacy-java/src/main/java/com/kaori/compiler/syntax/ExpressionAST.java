package com.kaori.compiler.syntax;

import java.util.List;

public interface ExpressionAST {
    public static enum BinaryOperator {
        PLUS("+"),
        MINUS("-"),
        MULTIPLY("*"),
        DIVIDE("/"),
        MODULO("%"),
        AND("&&"),
        OR("||"),
        NOT_EQUAL("!="),
        EQUAL("=="),
        GREATER(">"),
        GREATER_EQUAL(">="),
        LESS("<"),
        LESS_EQUAL("<=");

        public final String label;

        private BinaryOperator(String label) {
            this.label = label;
        }

        public String toString() {
            return this.label;
        }
    }

    public static enum UnaryOperator {
        NEGATE("-"),
        NOT("!");

        public final String label;

        private UnaryOperator(String label) {
            this.label = label;
        }

        public String toString() {
            return this.label;
        }
    }

    record BinaryExpression(ExpressionAST left, ExpressionAST right, BinaryOperator operator) implements ExpressionAST {
    }

    record UnaryExpression(ExpressionAST left, UnaryOperator operator) implements ExpressionAST {
    }

    record Assign(ExpressionAST.Identifier left, ExpressionAST right) implements ExpressionAST {
    }

    record Literal(TypeAST type, Object value) implements ExpressionAST {
    }

    record FunctionCall(ExpressionAST callee, List<ExpressionAST> arguments) implements ExpressionAST {
    }

    public class Identifier implements ExpressionAST {
        private final String name;
        private int offset;
        private boolean local;

        public Identifier(String name) {
            this.name = name;
            this.offset = -1;
            this.local = false;
        }

        public String name() {
            return this.name;
        }

        public int offset() {
            return this.offset;
        }

        public boolean local() {
            return this.local;
        }

        public void setReference(int offset, boolean local) {
            this.offset = offset;
            this.local = local;
        }

    }
}
