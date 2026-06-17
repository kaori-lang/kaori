package com.kaori.compiler.semantic;

import java.util.List;

import com.kaori.compiler.syntax.DeclarationAST;
import com.kaori.compiler.syntax.ExpressionAST;
import com.kaori.compiler.syntax.StatementAST;
import com.kaori.error.KaoriError;

public class Resolver extends Visitor<Object> {
    private final Environment<String> environment;

    public Resolver(List<DeclarationAST> declarations) {
        super(declarations);
        this.environment = new Environment<String>();

    }

    /* Expressions */
    @Override
    public Object visitBinaryExpression(ExpressionAST.BinaryExpression expression) {
        this.visit(expression.left());
        this.visit(expression.right());

        return null;
    }

    @Override
    public Object visitUnaryExpression(ExpressionAST.UnaryExpression expression) {
        this.visit(expression.left());

        return null;
    }

    @Override
    public Object visitAssign(ExpressionAST.Assign expression) {
        this.visit(expression.left());
        this.visit(expression.right());

        return null;
    }

    @Override
    public Object visitLiteral(ExpressionAST.Literal expression) {
        return null;
    }

    @Override
    public Object visitIdentifier(ExpressionAST.Identifier expression) {
        Resolution resolution = this.environment.search(expression.name());

        if (resolution == null) {
            throw KaoriError.ResolveError(expression.name() + " is not declared", this.line);
        }

        expression.setReference(resolution.offset(), resolution.local());

        return null;
    }

    @Override
    public Object visitFunctionCall(ExpressionAST.FunctionCall expression) {
        this.visit(expression.callee());

        for (ExpressionAST argument : expression.arguments()) {
            this.visit(argument);
        }

        return null;
    }

    /* Statements */
    @Override
    public void visitPrintStatement(StatementAST.Print statement) {
        this.visit(statement.expression());
    }

    @Override
    public void visitBlockStatement(StatementAST.Block statement) {
        this.environment.enterScope();
        this.visitDeclarations(statement.declarations());
        this.environment.exitScope();
    }

    @Override
    public void visitExpressionStatement(StatementAST.Expr statement) {
        this.visit(statement.expression());
    }

    @Override
    public void visitIfStatement(StatementAST.If statement) {
        this.visit(statement.condition());
        this.visit(statement.thenBranch());
        this.visit(statement.elseBranch());
    }

    @Override
    public void visitWhileLoopStatement(StatementAST.WhileLoop statement) {
        this.visit(statement.condition());
        this.visit(statement.block());
    }

    /* Declarations */
    @Override
    public void visitVariableDeclaration(DeclarationAST.Variable declaration) {
        ExpressionAST.Identifier identifier = declaration.left();

        this.visit(declaration.right());

        Resolution resolution = this.environment.searchInner(identifier.name());

        if (resolution == null) {
            this.environment.declare(identifier.name());
        } else {
            throw KaoriError.ResolveError(identifier.name() + " is already declared", this.line);
        }

    }

    @Override
    public void visitFunctionDeclaration(DeclarationAST.Function declaration) {
        ExpressionAST.Identifier identifier = declaration.name();

        if (this.environment.insideFunction()) {
            throw KaoriError.ResolveError(identifier.name() + " can't be declared inside another function", this.line);
        }

        Resolution resolution = this.environment.searchInner(identifier.name());

        if (resolution == null) {
            this.environment.declare(identifier.name());
        } else {
            throw KaoriError.ResolveError(identifier.name() + " is already declared", this.line);
        }

    }

    @Override
    public void visitFunctionDefinition(DeclarationAST.Function declaration) {
        this.environment.enterFunction();

        for (DeclarationAST.Variable parameter : declaration.parameters()) {
            this.visit(parameter);
        }

        List<DeclarationAST> declarations = declaration.block().declarations();

        this.visitDeclarations(declarations);

        this.environment.exitFunction();
    }

}
