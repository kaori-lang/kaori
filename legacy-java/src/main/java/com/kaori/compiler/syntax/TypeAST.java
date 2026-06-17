package com.kaori.compiler.syntax;

import java.util.List;

public interface TypeAST {
    public abstract boolean equals(TypeAST other);

    public enum Primitive implements TypeAST {
        STRING("string"),
        NUMBER("number"),
        BOOLEAN("boolean"),
        VOID("void");

        private final String label;

        private Primitive(String label) {
            this.label = label;
        }

        @Override
        public String toString() {
            return this.label;
        }

        @Override
        public boolean equals(TypeAST type) {
            if (type instanceof Primitive other) {
                return this == other;
            }

            return false;
        }

    }

    public record Function(List<TypeAST> parameters, TypeAST returnType) implements TypeAST {
        @Override
        public String toString() {
            String parameters = String.join(", ",
                    this.parameters.stream().map(parameter -> parameter.toString()).toList());

            return String.format("(%s) -> %s", parameters, returnType);
        }

        @Override
        public boolean equals(TypeAST type) {
            if (!(type instanceof Function other)) {
                return false;
            }

            if (!this.returnType.equals(other.returnType)) {
                return false;
            }

            if (this.parameters.size() != other.parameters.size()) {
                return false;
            }

            for (int i = 0; i < this.parameters.size(); i++) {
                if (!this.parameters.get(i).equals(other.parameters.get(i))) {
                    return false;
                }
            }

            return true;
        }
    }
}
