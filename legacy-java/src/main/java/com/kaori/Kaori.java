package com.kaori;

import java.io.FileWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import com.kaori.compiler.lexer.Lexer;
import com.kaori.compiler.lexer.Token;
import com.kaori.compiler.lexer.TokenStream;
import com.kaori.compiler.semantic.Resolver;
import com.kaori.compiler.semantic.TypeChecker;
import com.kaori.compiler.syntax.DeclarationAST;
import com.kaori.compiler.syntax.Parser;
import com.kaori.error.KaoriError;
import com.kaori.runtime.Interpreter;

public class Kaori {
    public void start() {
        try {

            Path path = Path.of("src/main/java/com/kaori/source/main.kaori");
            String source = Files.readString(path);

            TokenStream tokens = this.tokens(source);
            List<DeclarationAST> declarations = this.declarations(tokens);

            Resolver resolver = new Resolver(declarations);
            resolver.run();

            TypeChecker typeChecker = new TypeChecker(declarations);
            typeChecker.run();

            Interpreter interpreter = new Interpreter(declarations);

            long startTime = System.nanoTime();

            interpreter.run();

            long endTime = System.nanoTime();

            long durationNs = endTime - startTime;
            double durationMs = durationNs / 1_000_000.0;

            System.out.printf("Ast walker executed in %.3f ms%n", durationMs);

        } catch (KaoriError error) {
            System.out.println(error);
        } catch (IOException error) {
            System.out.println(error);
        }
    }

    private List<DeclarationAST> declarations(TokenStream tokens) {
        Parser parser = new Parser(tokens);

        return parser.declarations();
    }

    private TokenStream tokens(String source) {
        Lexer lexer = new Lexer(source);
        List<Token> tokens = lexer.scan();

        return new TokenStream(tokens, source);
    }
}
