package com.kaori.compiler.syntax;

import java.util.ArrayList;
import java.util.List;

public interface StatementAST extends DeclarationAST {
	record Print(int line, ExpressionAST expression) implements StatementAST {
	}

	record Expr(int line, ExpressionAST expression) implements StatementAST {
	}

	record Block(int line, List<DeclarationAST> declarations) implements StatementAST {
		public Block(int line) {
			this(line, new ArrayList<>());
		}
	}

	record If(
			int line,
			ExpressionAST condition,
			Block thenBranch,
			StatementAST elseBranch) implements StatementAST {
	}

	record WhileLoop(
			int line,
			ExpressionAST condition,
			Block block) implements StatementAST {
	}
}
