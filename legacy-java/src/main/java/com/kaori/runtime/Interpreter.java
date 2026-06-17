package com.kaori.runtime;

import java.util.ArrayList;
import java.util.List;

import com.kaori.compiler.semantic.Visitor;
import com.kaori.compiler.syntax.DeclarationAST;
import com.kaori.compiler.syntax.ExpressionAST;
import com.kaori.compiler.syntax.StatementAST;
import com.kaori.error.KaoriError;

public class Interpreter extends Visitor<Object> {
    public final CallStack<Object> callStack;

    public Interpreter(List<DeclarationAST> declarations) {
        super(declarations);
        this.callStack = new CallStack<>();
    }

    @Override
    public Object visitBinaryExpression(ExpressionAST.BinaryExpression expression) {
        Object left = this.visit(expression.left());
        Object right = this.visit(expression.right());
        ExpressionAST.BinaryOperator operator = expression.operator();

        return switch (operator) {
            case PLUS -> (Double) left + (Double) right;
            case MINUS -> (Double) left - (Double) right;
            case MULTIPLY -> (Double) left * (Double) right;
            case DIVIDE -> {
                if ((Double) right == 0) {
                    throw KaoriError.RuntimeError(
                            String.format("invalid %s operation between %s and %s", operator, left, right),
                            this.line);
                }

                yield (Double) left / (Double) right;
            }

            case MODULO -> {
                if ((Double) right == 0) {
                    throw KaoriError.RuntimeError(
                            String.format("invalid %s operation between %s and %s", operator, left, right),
                            this.line);
                }

                yield (Double) left % (Double) right;
            }
            case GREATER -> (Double) left > (Double) right;
            case GREATER_EQUAL -> (Double) left >= (Double) right;
            case LESS -> (Double) left < (Double) right;
            case LESS_EQUAL -> (Double) left <= (Double) right;
            case AND -> (Boolean) left && (Boolean) right;
            case OR -> (Boolean) left || (Boolean) right;
            case EQUAL -> left.equals(right);
            case NOT_EQUAL -> !left.equals(right);
        };
    }

    @Override
    public Object visitUnaryExpression(ExpressionAST.UnaryExpression expression) {
        Object left = this.visit(expression.left());
        ExpressionAST.UnaryOperator operator = expression.operator();

        return switch (operator) {
            case NEGATE -> -(Double) left;
            case NOT -> !(Boolean) left;
        };
    }

    @Override
    public Object visitAssign(ExpressionAST.Assign expression) {
        Object value = this.visit(expression.right());
        ExpressionAST.Identifier identifier = expression.left();

        this.callStack.define(value, identifier.offset(), identifier.local());

        return value;
    }

    @Override
    public Object visitLiteral(ExpressionAST.Literal expression) {
        return expression.value();
    }

    @Override
    public Object visitIdentifier(ExpressionAST.Identifier expression) {
        Object value = this.callStack.get(expression.offset(), expression.local());

        if (value == null) {
            throw KaoriError.RuntimeError(expression.name() + " is not defined", this.line);
        }

        return value;
    }

    @Override
    public Object visitFunctionCall(ExpressionAST.FunctionCall expression) {
        FunctionObject functionObject = (FunctionObject) this.visit(expression.callee());

        List<Object> arguments = new ArrayList<>();

        for (int i = 0; i < functionObject.parameters().size(); i++) {
            if (i < expression.arguments().size()) {
                ExpressionAST argument = expression.arguments().get(i);
                arguments.add(this.visit(argument));
            } else {
                arguments.add(null);
            }

        }

        this.callStack.enterFunction();

        for (int i = 0; i < functionObject.parameters().size(); i++) {
            Object defaultValue = this.visit(functionObject.parameters().get(i).right());
            Object argument = arguments.get(i);

            this.callStack.declare(argument == null ? defaultValue : argument);
        }

        this.visitDeclarations(functionObject.declarations());

        this.callStack.exitFunction();

        return null;
    }

    @Override
    public void visitPrintStatement(StatementAST.Print statement) {
        Object expression = this.visit(statement.expression());

        System.out.println(expression);
    }

    @Override
    public void visitBlockStatement(StatementAST.Block statement) {
        this.callStack.enterScope();
        this.visitDeclarations(statement.declarations());
        this.callStack.exitScope();
    }

    @Override
    public void visitExpressionStatement(StatementAST.Expr statement) {
        this.visit(statement.expression());
    }

    @Override
    public void visitIfStatement(StatementAST.If statement) {
        Object condition = this.visit(statement.condition());

        if ((Boolean) condition == true) {
            this.visit(statement.thenBranch());
        } else {
            this.visit(statement.elseBranch());
        }
    }

    @Override
    public void visitWhileLoopStatement(StatementAST.WhileLoop statement) {
        while (true) {
            Object condition = this.visit(statement.condition());

            if ((Boolean) condition == false)
                break;

            this.visit(statement.block());
        }
    }

    /* Declarations */
    @Override
    public void visitVariableDeclaration(DeclarationAST.Variable declaration) {
        Object right = this.visit(declaration.right());

        this.callStack.declare(right);
    }

    @Override
    public void visitFunctionDeclaration(DeclarationAST.Function declaration) {
        FunctionObject functionObject = new FunctionObject(declaration.parameters(),
                declaration.block().declarations());

        this.callStack.declare(functionObject);
    }

    @Override
    public void visitFunctionDefinition(DeclarationAST.Function declaration) {

    }
}