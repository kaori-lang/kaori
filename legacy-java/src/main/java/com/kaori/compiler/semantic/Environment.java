package com.kaori.compiler.semantic;

import java.util.Stack;

public class Environment<T> {
    private final Stack<T> declarations;
    private int index;
    private Stack<Integer> scopes;
    private int framePointer;

    public Environment() {
        this.declarations = new Stack<>();
        this.index = 0;
        this.scopes = new Stack<>();
        this.framePointer = 0;

        this.scopes.push(0);
        this.declarations.setSize(1_000);

    }

    public void declare(T value) {
        this.declarations.set(this.index, value);
        this.index++;
    }

    public void define(T value, int offset, boolean local) {
        if (local) {
            offset = this.framePointer + offset;
        }

        this.declarations.set(offset, value);
    }

    public T get(int offset, boolean local) {
        if (local) {
            offset = this.framePointer + offset;
        }

        return this.declarations.get(offset);
    }

    public Resolution searchInner(String identifier) {
        int top = this.index - 1;

        for (int i = top; i >= this.scopes.peek(); i--) {
            T declaration = this.declarations.get(i);

            if (declaration.equals(identifier)) {
                boolean local = i >= this.framePointer;
                int offset = i;

                if (local) {
                    offset = offset - this.framePointer;
                }

                Resolution resolution = new Resolution(offset, local);

                return resolution;
            }
        }

        return null;
    }

    public Resolution search(String identifier) {
        int top = this.index - 1;

        for (int i = top; i >= 0; i--) {
            T declaration = this.declarations.get(i);

            if (declaration.equals(identifier)) {
                boolean local = i >= this.framePointer;
                int offset = i;

                if (local) {
                    offset = offset - this.framePointer;
                }

                Resolution resolution = new Resolution(offset, local);

                return resolution;
            }
        }

        return null;
    }

    public void enterScope() {
        this.scopes.add(this.index);
    }

    public void exitScope() {
        this.index = this.scopes.pop();
    }

    public void enterFunction() {
        this.framePointer = this.index;

        this.enterScope();
    }

    public void exitFunction() {
        this.framePointer = 0;

        this.exitScope();
    }

    public boolean insideFunction() {
        return this.framePointer > 0;
    }
}
