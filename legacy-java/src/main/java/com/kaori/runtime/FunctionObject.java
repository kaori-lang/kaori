package com.kaori.runtime;

import java.util.List;

import com.kaori.compiler.syntax.DeclarationAST;

public record FunctionObject(List<DeclarationAST.Variable> parameters, List<DeclarationAST> declarations) {

}
