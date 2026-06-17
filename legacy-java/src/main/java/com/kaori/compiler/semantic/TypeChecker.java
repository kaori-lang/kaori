package com.kaori.compiler.semantic;

import java.util.List;

import com.kaori.compiler.syntax.DeclarationAST;
import com.kaori.compiler.syntax.ExpressionAST;
import com.kaori.compiler.syntax.StatementAST;
import com.kaori.compiler.syntax.TypeAST;
import com.kaori.error.KaoriError;

public class TypeChecker extends Visitor<TypeAST> {
    private final Environment<TypeAST> environment;

    public TypeChecker(List<DeclarationAST> declarations) {
        super(declarations);
        this.environment = new Environment<>();

    }

    @Override
    public TypeAST visitBinaryExpression(ExpressionAST.BinaryExpression expression) {
        TypeAST left = this.visit(expression.left());
        TypeAST right = this.visit(expression.right());
        ExpressionAST.BinaryOperator operator = expression.operator();

        TypeAST type = switch (operator) {
            case PLUS, MINUS, MULTIPLY, DIVIDE, MODULO ->
                left.equals(TypeAST.Primitive.NUMBER) &&
                        right.equals(TypeAST.Primitive.NUMBER)
                                ? TypeAST.Primitive.NUMBER
                                : TypeAST.Primitive.VOID;

            case GREATER, GREATER_EQUAL, LESS, LESS_EQUAL ->
                left.equals(TypeAST.Primitive.NUMBER) &&
                        right.equals(TypeAST.Primitive.NUMBER)
                                ? TypeAST.Primitive.BOOLEAN
                                : TypeAST.Primitive.VOID;

            case AND, OR ->
                left.equals(TypeAST.Primitive.BOOLEAN) &&
                        right.equals(TypeAST.Primitive.BOOLEAN)
                                ? TypeAST.Primitive.BOOLEAN
                                : TypeAST.Primitive.VOID;

            case EQUAL, NOT_EQUAL ->
                left.equals(right)
                        ? TypeAST.Primitive.BOOLEAN
                        : TypeAST.Primitive.VOID;
        };

        if (type.equals(TypeAST.Primitive.VOID)) {
            throw KaoriError.TypeError(
                    String.format("invalid %s operation between %s and %s", operator, left, right),
                    this.line);
        }

        return type;
    }

    @Override
    public TypeAST visitUnaryExpression(ExpressionAST.UnaryExpression expression) {
        TypeAST left = this.visit(expression.left());
        ExpressionAST.UnaryOperator operator = expression.operator();

        TypeAST type = switch (operator) {
            case NEGATE -> left.equals(TypeAST.Primitive.NUMBER) ? TypeAST.Primitive.NUMBER : TypeAST.Primitive.VOID;
            case NOT -> left.equals(TypeAST.Primitive.BOOLEAN) ? TypeAST.Primitive.BOOLEAN : TypeAST.Primitive.VOID;

        };

        if (type.equals(TypeAST.Primitive.VOID)) {
            throw KaoriError.TypeError(String.format("invalid %s operation for %s", operator, left),
                    this.line);
        }

        return type;
    }

    @Override
    public TypeAST visitAssign(ExpressionAST.Assign expression) {
        ExpressionAST.Identifier identifier = expression.left();

        TypeAST left = this.visit(identifier);
        TypeAST right = this.visit(expression.right());

        if (!left.equals(right)) {
            throw KaoriError.TypeError(
                    String.format("invalid variable assignment with type %s for type %s", right, left),
                    this.line);
        }

        this.environment.define(right, identifier.offset(), identifier.local());

        return right;
    }

    @Override
    public TypeAST visitLiteral(ExpressionAST.Literal expression) {
        return expression.type();
    }

    @Override
    public TypeAST visitIdentifier(ExpressionAST.Identifier expression) {
        return this.environment.get(expression.offset(), expression.local());
    }

    @Override
    public TypeAST visitFunctionCall(ExpressionAST.FunctionCall expression) {
        TypeAST type = this.visit(expression.callee());

        if (!(type instanceof TypeAST.Function func)) {
            throw KaoriError.TypeError(String.format("invalid %s type is not a function", type),
                    this.line);
        }

        int smallest = Math.min(func.parameters().size(), expression.arguments().size());

        for (int i = 0; i < smallest; i++) {
            TypeAST argument = this.visit(expression.arguments().get(i));
            TypeAST parameter = func.parameters().get(i);

            if (!argument.equals(parameter)) {
                throw KaoriError.TypeError(
                        String.format("invalid argument of type %s for parameter of type %s", argument, parameter),
                        this.line);
            }
        }

        return func.returnType();
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
        TypeAST condition = this.visit(statement.condition());

        if (!condition.equals(TypeAST.Primitive.BOOLEAN)) {
            throw KaoriError.TypeError(String.format("invalid type for condition: %s", condition), this.line);
        }

        this.visit(statement.thenBranch());
        this.visit(statement.elseBranch());
    }

    @Override
    public void visitWhileLoopStatement(StatementAST.WhileLoop statement) {
        TypeAST condition = this.visit(statement.condition());

        if (!condition.equals(TypeAST.Primitive.BOOLEAN)) {
            throw KaoriError.TypeError(String.format("invalid type for condition: %s", condition), this.line);
        }

        this.visit(statement.block());
    }

    /* Declarations */
    @Override
    public void visitVariableDeclaration(DeclarationAST.Variable declaration) {
        TypeAST left = declaration.type();
        TypeAST right = this.visit(declaration.right());

        if (!left.equals(right)) {
            throw KaoriError.TypeError(
                    String.format("invalid variable declaration with type %s for type %s", left, right),
                    this.line);
        }

        this.environment.declare(left);
    }

    @Override
    public void visitFunctionDeclaration(DeclarationAST.Function declaration) {
        this.environment.declare(declaration.type());
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
