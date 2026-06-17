package com.kaori.runtime;

import java.util.Stack;

public class CallStack<T> {
    public final Stack<T> stack;
    private final Stack<Integer> scopes;
    private final Stack<Integer> framePointers;
    private int index;

    public CallStack() {
        final int stackMaxSize = 1_000;

        this.stack = new Stack<>();
        this.scopes = new Stack<>();
        this.framePointers = new Stack<>();
        this.index = 0;

        this.stack.setSize(stackMaxSize);
        this.framePointers.push(0);
    }

    private void updateIndex() {
        this.index++;
    }

    public void declare(T value) {
        this.stack.set(index, value);

        this.updateIndex();
    }

    public void define(T value, int offset, boolean local) {
        if (local) {
            offset = framePointers.peek() + offset;
        }

        this.stack.set(offset, value);
    }

    public T get(int offset, boolean local) {
        if (local) {
            offset = framePointers.peek() + offset;
        }

        return this.stack.get(offset);
    }

    public T loadLocal(int offset) {
        offset = framePointers.peek() + offset;

        return this.stack.get(offset);
    }

    public T loadGlobal(int offset) {
        return this.stack.get(offset);
    }

    public void storeLocal(T value, int offset) {
        offset = framePointers.peek() + offset;

        this.stack.set(offset, value);
    }

    public void storeGlobal(T value, int offset) {
        this.stack.set(offset, value);
    }

    public void enterFunction() {
        this.framePointers.push(this.index);
    }

    public void exitFunction() {
        this.index = this.framePointers.pop();
    }

    public void enterScope() {
        this.scopes.add(this.index);
    }

    public void exitScope() {
        this.index = this.scopes.pop();
    }
}
