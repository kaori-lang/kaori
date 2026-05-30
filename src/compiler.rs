use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    sync::{LazyLock, Mutex},
};

use logos::Logos;

use crate::{
    bytecode::{function::Function, lower_ast::lower_ast},
    diagnostics::error::Error,
    report_error,
    syntax::{
        ast::{Ast, Spanned},
        parser::Parser,
        token::{Span, Token},
    },
    util::string_interner::{StringInterner, Symbol},
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

pub struct Compiler {
    pub functions: Vec<Function>,
    pub compiled_imports: HashMap<Symbol, usize>,
    pub visited: HashSet<Symbol>,
    pub path: Symbol,
}

impl Default for Compiler {
    fn default() -> Self {
        let path = INTERNER.lock().unwrap().get_or_intern("main.kr");

        Self {
            functions: Vec::new(),
            compiled_imports: HashMap::new(),
            visited: HashSet::new(),
            path,
        }
    }
}
impl Compiler {
    pub fn compile(&mut self) {
        let span = Span::default();

        let index = match self.compile_file(Spanned::new(self.path, span)) {
            Ok(index) => index,
            Err(error) => {
                return error.report();
            }
        };

        for function in self.functions.iter() {
            println!("{}", function);
        }
    }

    pub fn compile_file(&mut self, path: Spanned<Symbol>) -> Result<usize, Error> {
        if let Some(index) = self.compiled_imports.get(&path.value).copied() {
            return Ok(index);
        }

        if self.visited.contains(&path.value) {
            return Err(report_error!(
                path.span,
                path.value,
                "circular import detected"
            ));
        }

        let src = match read_to_string(INTERNER.lock().unwrap().resolve(path.value)) {
            Ok(source) => source,
            Err(..) => {
                return Err(report_error!(
                    path.span,
                    self.path,
                    "tried to import an invalid file path"
                ));
            }
        };

        self.visited.insert(path.value);

        let previous_path = self.path;
        self.path = path.value;
        let index = self.compile_source_code(path.value, &src)?;

        self.compiled_imports.insert(path.value, index);
        self.path = previous_path;
        Ok(index)
    }

    pub fn compile_source_code(&mut self, path: Symbol, src: &str) -> Result<usize, Error> {
        let mut tokens = Token::lexer(src)
            .spanned()
            .map(|(token, span)| match token {
                Ok(token) => Ok((token, span.into())),
                Err(()) => Err(report_error!(span.into(), path, "unexpected token")),
            })
            .collect::<Result<Vec<(Token, Span)>, Error>>()?;
        tokens.push((Token::Eof, Span::from(src.len()..src.len())));

        let parser = Parser::new(src, tokens, self);

        let ast = parser.parse()?;

        lower_ast(ast, self)
    }

    pub fn push_function(&mut self, function: Function) -> usize {
        let index = self.functions.len();

        self.functions.push(function);

        index
    }
}

pub fn compile_and_run() {
    //run_vm(functions)?;
}
