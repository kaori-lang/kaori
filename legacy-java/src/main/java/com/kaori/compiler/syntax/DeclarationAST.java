package com.kaori.compiler.syntax;

import java.util.List;

public interface DeclarationAST {
	int line();

	public static class Variable implements DeclarationAST {
		public final int line;
		public final ExpressionAST.Identifier left;
		public ExpressionAST right;
		public final TypeAST type;

		public Variable(int line, ExpressionAST.Identifier left, ExpressionAST right, TypeAST type) {
			this.line = line;
			this.left = left;
			this.right = right;
			this.type = type;
		}

		@Override
		public int line() {
			return line;
		}

		public ExpressionAST.Identifier left() {
			return left;
		}

		public ExpressionAST right() {
			return right;
		}

		public TypeAST type() {
			return type;
		}

		public void setRight(ExpressionAST right) {
			this.right = right;
		}
	}

	public record Function(
			int line,
			ExpressionAST.Identifier name,
			List<Variable> parameters,
			TypeAST.Function type,
			StatementAST.Block block) implements DeclarationAST {
	}

}
