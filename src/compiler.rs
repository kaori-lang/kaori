use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        lexer::{lexer::Lexer, token::Token, token_stream::TokenStream},
        semantic::{resolver::Resolver, type_checker::TypeChecker},
        syntax::parser::Parser,
    },
};

struct Compiler {
    source: String,
}

impl Compiler {
    fn run_lexer(&self) -> Result<TokenStream, KaoriError> {
        let mut tokens = Vec::new();
        let mut lexer = Lexer::new(&self.source, &mut tokens);
        lexer.tokenize()?;

        let token_stream = TokenStream::new(self.source.to_string(), tokens);
        Ok(token_stream)
    }

    pub fn compile(&mut self) -> Result<(), KaoriError> {
        let token_stream = self.run_lexer()?;

        let mut parser = Parser::new(token_stream);

        let mut ast = parser.parse()?;

        let mut resolver = Resolver::default();

        let hir = resolver.resolve(&mut ast)?;

        let mut type_checker = TypeChecker::default();

        type_checker.check(&hir)?;

        Ok(())
    }
}
