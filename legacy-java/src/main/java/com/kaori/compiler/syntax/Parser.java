package com.kaori.compiler.syntax;

import java.util.ArrayList;
import java.util.List;

import com.kaori.compiler.lexer.TokenKind;
import com.kaori.compiler.lexer.TokenStream;
import com.kaori.error.KaoriError;

public class Parser {
    private final TokenStream tokens;

    public Parser(TokenStream tokens) {
        this.tokens = tokens;
    }

    public List<DeclarationAST> declarations() {
        List<DeclarationAST> declarations = new ArrayList<>();

        while (!this.tokens.atEnd()) {
            DeclarationAST declaration = this.declaration();
            declarations.add(declaration);

        }

        return declarations;
    }

    /*
     * ─────────────────────────────
     * Declarations
     * ─────────────────────────────
     */
    private DeclarationAST declaration() {
        DeclarationAST declaration = switch (this.tokens.getCurrent()) {
            case FUNCTION -> this.functionDeclaration();
            default -> this.tokens.lookAhead(TokenKind.IDENTIFIER, TokenKind.COLON) ? this.variableDeclaration()
                    : this.statement();
        };

        if (declaration instanceof StatementAST.Print || declaration instanceof DeclarationAST.Variable
                || declaration instanceof StatementAST.Expr)

        {
            this.tokens.consume(TokenKind.SEMICOLON);
        }

        return declaration;
    }

    private DeclarationAST.Variable variableDeclaration() {
        int line = this.tokens.getLine();

        ExpressionAST.Identifier left = this.identifier();
        this.tokens.consume(TokenKind.COLON);

        TypeAST type = this.type();

        if (this.tokens.getCurrent() != TokenKind.ASSIGN) {
            ExpressionAST right = new ExpressionAST.Literal(type, null);
            return new DeclarationAST.Variable(line, left, right, type);
        }

        this.tokens.consume(TokenKind.ASSIGN);

        ExpressionAST right = this.expression();

        return new DeclarationAST.Variable(line, left, right, type);
    }

    private DeclarationAST functionDeclaration() {
        int line = this.tokens.getLine();

        this.tokens.consume(TokenKind.FUNCTION);

        ExpressionAST.Identifier name = this.identifier();

        this.tokens.consume(TokenKind.LEFT_PAREN);

        List<DeclarationAST.Variable> parameters = new ArrayList<>();

        while (!this.tokens.atEnd() && this.tokens.getCurrent() != TokenKind.RIGHT_PAREN) {
            DeclarationAST.Variable parameter = this.variableDeclaration();
            parameters.add(parameter);

            if (this.tokens.getCurrent() == TokenKind.RIGHT_PAREN) {
                break;
            }

            this.tokens.consume(TokenKind.COMMA);
        }

        this.tokens.consume(TokenKind.RIGHT_PAREN);

        this.tokens.consume(TokenKind.THIN_ARROW);

        TypeAST returnType = this.type();

        TypeAST.Function type = new TypeAST.Function(parameters.stream().map(parameter -> parameter.type()).toList(),
                returnType);

        StatementAST.Block block = this.blockStatement();

        return new DeclarationAST.Function(line, name, parameters, type, block);
    }

    /*
     * ─────────────────────────────
     * Statements
     * ─────────────────────────────
     */
    private StatementAST.Expr expressionStatement() {
        int line = this.tokens.getLine();

        ExpressionAST expression = this.expression();

        return new StatementAST.Expr(line, expression);
    }

    private StatementAST.Print printStatement() {
        int line = this.tokens.getLine();

        this.tokens.consume(TokenKind.PRINT);

        this.tokens.consume(TokenKind.LEFT_PAREN);

        ExpressionAST expression = this.expression();

        this.tokens.consume(TokenKind.RIGHT_PAREN);

        return new StatementAST.Print(line, expression);
    }

    private StatementAST.Block blockStatement() {
        int line = this.tokens.getLine();

        this.tokens.consume(TokenKind.LEFT_BRACE);

        List<DeclarationAST> declarations = new ArrayList<>();

        while (!this.tokens.atEnd() && this.tokens.getCurrent() != TokenKind.RIGHT_BRACE) {
            DeclarationAST declaration = this.declaration();
            declarations.add(declaration);
        }

        this.tokens.consume(TokenKind.RIGHT_BRACE);

        return new StatementAST.Block(line, declarations);
    }

    private StatementAST.If ifStatement() {
        int line = this.tokens.getLine();

        this.tokens.consume(TokenKind.IF);

        ExpressionAST condition = this.expression();

        StatementAST.Block thenBranch = this.blockStatement();

        if (this.tokens.getCurrent() != TokenKind.ELSE) {

            StatementAST.Block elseBranch = new StatementAST.Block(line);
            return new StatementAST.If(line, condition, thenBranch, elseBranch);
        }

        this.tokens.consume(TokenKind.ELSE);

        StatementAST elseBranch = switch (this.tokens.getCurrent()) {
            case IF -> this.ifStatement();
            default -> this.blockStatement();
        };

        return new StatementAST.If(line, condition, thenBranch, elseBranch);
    }

    private StatementAST.WhileLoop whileLoopStatement() {
        int line = this.tokens.getLine();

        this.tokens.consume(TokenKind.WHILE);

        ExpressionAST condition = this.expression();

        StatementAST.Block block = this.blockStatement();

        return new StatementAST.WhileLoop(line, condition, block);
    }

    private StatementAST.Block forLoopStatement() {
        int line = this.tokens.getLine();

        this.tokens.consume(TokenKind.FOR);

        DeclarationAST.Variable variable = this.variableDeclaration();

        this.tokens.consume(TokenKind.SEMICOLON);

        ExpressionAST condition = this.expression();

        this.tokens.consume(TokenKind.SEMICOLON);

        StatementAST.Expr increment = this.expressionStatement();

        StatementAST.Block block = this.blockStatement();

        StatementAST.WhileLoop whileLoop = new StatementAST.WhileLoop(line, condition, block);

        List<DeclarationAST> declarations = new ArrayList<>();

        declarations.add(variable);
        declarations.add(whileLoop);

        whileLoop.block().declarations().add(increment);

        return new StatementAST.Block(line, declarations);
    }

    private StatementAST statement() {
        return switch (this.tokens.getCurrent()) {
            case PRINT -> this.printStatement();
            case IF -> this.ifStatement();
            case WHILE -> this.whileLoopStatement();
            case FOR -> this.forLoopStatement();
            default -> this.expressionStatement();
        };
    }

    /*
     * ─────────────────────────────
     * Expressions
     * ─────────────────────────────
     */
    private ExpressionAST expression() {
        if (this.tokens.lookAhead(TokenKind.IDENTIFIER, TokenKind.ASSIGN)) {
            return this.assign();
        }

        return this.or();
    }

    private ExpressionAST assign() {
        ExpressionAST.Identifier left = this.identifier();
        this.tokens.consume(TokenKind.ASSIGN);
        ExpressionAST right = this.expression();

        return new ExpressionAST.Assign(left, right);
    }

    private ExpressionAST or() {
        ExpressionAST left = this.and();

        while (!this.tokens.atEnd()) {
            TokenKind operator = this.tokens.getCurrent();

            switch (operator) {
                case OR -> {
                    this.tokens.consume();
                    ExpressionAST right = this.and();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.OR);
                }
                default -> {
                    return left;
                }
            }
        }

        return left;
    }

    private ExpressionAST and() {
        ExpressionAST left = this.equality();

        while (!this.tokens.atEnd()) {
            TokenKind operator = this.tokens.getCurrent();

            switch (operator) {
                case AND -> {
                    this.tokens.consume();
                    ExpressionAST right = this.equality();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.AND);
                }
                default -> {
                    return left;
                }
            }
        }

        return left;
    }

    private ExpressionAST equality() {
        ExpressionAST left = this.comparison();

        while (!this.tokens.atEnd()) {
            TokenKind operator = this.tokens.getCurrent();

            switch (operator) {
                case EQUAL -> {
                    this.tokens.consume();
                    ExpressionAST right = this.comparison();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.EQUAL);
                }
                case NOT_EQUAL -> {
                    this.tokens.consume();
                    ExpressionAST right = this.comparison();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.NOT_EQUAL);
                }
                default -> {
                    return left;
                }
            }
        }

        return left;
    }

    private ExpressionAST comparison() {
        ExpressionAST left = this.term();

        while (!this.tokens.atEnd()) {
            TokenKind operator = this.tokens.getCurrent();

            switch (operator) {
                case GREATER -> {
                    this.tokens.consume();
                    ExpressionAST right = this.term();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.GREATER);
                }
                case GREATER_EQUAL -> {
                    this.tokens.consume();
                    ExpressionAST right = this.term();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.GREATER_EQUAL);
                }
                case LESS -> {
                    this.tokens.consume();
                    ExpressionAST right = this.term();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.LESS);
                }
                case LESS_EQUAL -> {
                    this.tokens.consume();
                    ExpressionAST right = this.term();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.LESS_EQUAL);
                }
                default -> {
                    return left;
                }
            }
        }

        return left;
    }

    private ExpressionAST term() {
        ExpressionAST left = this.factor();

        while (!this.tokens.atEnd()) {
            TokenKind operator = this.tokens.getCurrent();

            switch (operator) {
                case PLUS -> {
                    this.tokens.consume();
                    ExpressionAST right = this.factor();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.PLUS);
                }
                case MINUS -> {
                    this.tokens.consume();
                    ExpressionAST right = this.factor();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.MINUS);
                }
                default -> {
                    return left;
                }
            }
        }

        return left;
    }

    private ExpressionAST factor() {
        ExpressionAST left = this.prefixUnary();

        while (!this.tokens.atEnd()) {
            TokenKind operator = this.tokens.getCurrent();

            switch (operator) {
                case MULTIPLY -> {
                    this.tokens.consume();
                    ExpressionAST right = this.prefixUnary();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.MULTIPLY);
                }
                case DIVIDE -> {
                    this.tokens.consume();
                    ExpressionAST right = this.prefixUnary();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.DIVIDE);
                }
                case MODULO -> {
                    this.tokens.consume();
                    ExpressionAST right = this.prefixUnary();
                    left = new ExpressionAST.BinaryExpression(left, right, ExpressionAST.BinaryOperator.MODULO);
                }
                default -> {
                    return left;
                }
            }
        }

        return left;
    }

    private ExpressionAST prefixUnary() {
        return switch (this.tokens.getCurrent()) {
            case MINUS -> {
                this.tokens.consume();
                yield new ExpressionAST.UnaryExpression(this.prefixUnary(), ExpressionAST.UnaryOperator.NEGATE);
            }
            case NOT -> {
                this.tokens.consume();
                yield new ExpressionAST.UnaryExpression(this.prefixUnary(), ExpressionAST.UnaryOperator.NOT);
            }
            case PLUS -> {
                this.tokens.consume();
                yield this.prefixUnary();

            }
            default -> this.primary();
        };
    }

    private ExpressionAST primary() {
        return switch (this.tokens.getCurrent()) {
            case BOOLEAN_LITERAL -> {
                String lexeme = this.tokens.getLexeme();
                boolean value = Boolean.parseBoolean(lexeme);
                ExpressionAST literal = new ExpressionAST.Literal(TypeAST.Primitive.BOOLEAN, value);
                this.tokens.consume();

                yield literal;
            }
            case STRING_LITERAL -> {
                String lexeme = this.tokens.getLexeme();
                String value = lexeme.substring(1, lexeme.length() - 1);
                TypeAST type = TypeAST.Primitive.STRING;
                ExpressionAST literal = new ExpressionAST.Literal(type, value);
                this.tokens.consume();

                yield literal;
            }
            case NUMBER_LITERAL -> {
                String lexeme = this.tokens.getLexeme();
                Double value = Double.parseDouble(lexeme);
                TypeAST type = TypeAST.Primitive.NUMBER;
                ExpressionAST literal = new ExpressionAST.Literal(type, value);
                this.tokens.consume();

                yield literal;
            }
            case IDENTIFIER -> {
                ExpressionAST.Identifier identifier = this.identifier();
                yield this.postfixUnary(identifier);

            }
            case LEFT_PAREN -> this.parenthesis();
            default -> throw KaoriError.SyntaxError("expected a valid operand", this.tokens.getLine());
        };
    }

    private ExpressionAST.Identifier identifier() {
        String lexeme = this.tokens.getLexeme();
        this.tokens.consume(TokenKind.IDENTIFIER);

        return new ExpressionAST.Identifier(lexeme);
    }

    private ExpressionAST parenthesis() {
        this.tokens.consume(TokenKind.LEFT_PAREN);

        ExpressionAST expression = this.expression();

        this.tokens.consume(TokenKind.RIGHT_PAREN);

        return expression;
    }

    private ExpressionAST postfixUnary(ExpressionAST.Identifier identifier) {
        return switch (this.tokens.getCurrent()) {
            case INCREMENT -> {
                ExpressionAST literal = new ExpressionAST.Literal(TypeAST.Primitive.NUMBER, 1.0);
                ExpressionAST add = new ExpressionAST.BinaryExpression(identifier, literal,
                        ExpressionAST.BinaryOperator.PLUS);
                this.tokens.consume();
                yield new ExpressionAST.Assign(identifier, add);
            }
            case DECREMENT -> {
                ExpressionAST literal = new ExpressionAST.Literal(TypeAST.Primitive.NUMBER, 1.0);
                ExpressionAST subtract = new ExpressionAST.BinaryExpression(identifier, literal,
                        ExpressionAST.BinaryOperator.MINUS);
                this.tokens.consume();
                yield new ExpressionAST.Assign(identifier, subtract);
            }
            case LEFT_PAREN -> this.functionCall(identifier);
            default -> identifier;
        };
    }

    private ExpressionAST functionCall(ExpressionAST callee) {
        if (this.tokens.getCurrent() != TokenKind.LEFT_PAREN) {
            return callee;
        }

        this.tokens.consume(TokenKind.LEFT_PAREN);

        List<ExpressionAST> arguments = new ArrayList<>();

        while (!this.tokens.atEnd() && this.tokens.getCurrent() != TokenKind.RIGHT_PAREN) {
            ExpressionAST argument = this.expression();
            arguments.add(argument);

            if (this.tokens.getCurrent() == TokenKind.RIGHT_PAREN) {
                break;
            }

            this.tokens.consume(TokenKind.COMMA);
        }

        this.tokens.consume(TokenKind.RIGHT_PAREN);

        ExpressionAST call = new ExpressionAST.FunctionCall(callee, arguments);

        return this.functionCall(call);
    }

    /*
     * ─────────────────────────────
     * Types
     * ─────────────────────────────
     */
    private TypeAST type() {
        TypeAST type = switch (this.tokens.getCurrent()) {
            case IDENTIFIER -> this.primitiveType();
            default ->
                throw KaoriError.SyntaxError(this.tokens.getCurrent() + " is not a valid type", this.tokens.getLine());
        };

        return type;
    }

    private TypeAST primitiveType() {
        String lexeme = this.tokens.getLexeme();
        this.tokens.consume(TokenKind.IDENTIFIER);

        TypeAST type = switch (lexeme) {
            case "str" -> TypeAST.Primitive.STRING;
            case "bool" -> TypeAST.Primitive.BOOLEAN;
            case "number" -> TypeAST.Primitive.NUMBER;
            case "void" -> TypeAST.Primitive.VOID;
            default ->
                throw KaoriError.SyntaxError(this.tokens.getCurrent() + " is not a valid type", this.tokens.getLine());
        };

        return type;
    }

}
