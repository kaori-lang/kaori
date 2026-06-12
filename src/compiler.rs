use std::{
    collections::HashMap,
    fs::read_to_string,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use logos::Logos;

use crate::{
    codegen::lower_ast::lower_ast,
    diagnostics::error::Error,
    runtime::{function::Function, vm::run_vm},
    syntax::{
        ast::Spanned,
        parser::Parser,
        token::{Span, Token},
    },
    util::string_interner::{StringInterner, Symbol},
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

pub enum Compilation {
    Incomplete,
    Function(usize),
}

#[derive(Default)]
pub struct Compiler {
    pub functions: Vec<Function>,
    pub files: HashMap<Symbol, Compilation>,
    pub current_file: Symbol,
}

impl Compiler {
    pub fn compile(&mut self, file: &str) -> Result<usize, Error> {
        let symbol = INTERNER.lock().unwrap().get_or_intern(file);

        let src = match read_to_string(file) {
            Ok(source) => source,
            Err(..) => {
                return Err(Error::new(
                    Span::default(),
                    self.current_file,
                    format!("{} file not found", file),
                ));
            }
        };

        self.current_file = symbol;

        self.files.insert(symbol, Compilation::Incomplete);

        let index = self.compile_source(&src)?;

        self.files.insert(symbol, Compilation::Function(index));

        for function in self.functions.iter() {
            println!("{}", function);
        }

        Ok(index)
    }

    pub fn compile_file(
        &mut self,
        interned_path: &[Spanned<Symbol>],
    ) -> Result<usize, Error> {
        let mut path = PathBuf::new();

        for symbol in interned_path {
            path.push(INTERNER.lock().unwrap().resolve(symbol.value));
        }

        path.add_extension("kr");

        let interned_file =
            INTERNER.lock().unwrap().get_or_intern(path.to_str().unwrap());

        if let Some(compilation) = self.files.get(&interned_file) {
            match compilation {
                Compilation::Incomplete => {
                    let span = interned_path
                        .last()
                        .map(|s| s.span)
                        .unwrap_or_default();
                    return Err(Error::new(
                        span,
                        interned_file,
                        "cyclic dependency detected".to_string(),
                    ));
                }
                Compilation::Function(index) => return Ok(*index),
            }
        }

        let src = match read_to_string(&path) {
            Ok(source) => source,
            Err(..) => {
                let span =
                    interned_path.last().map(|s| s.span).unwrap_or_default();
                return Err(Error::new(
                    span,
                    self.current_file,
                    "expected a valid file path".to_string(),
                ));
            }
        };

        self.files.insert(interned_file, Compilation::Incomplete);

        let previous_file = self.current_file;

        self.current_file = interned_file;

        let index = self.compile_source(&src)?;

        self.files.insert(interned_file, Compilation::Function(index));

        self.current_file = previous_file;

        Ok(index)
    }

    fn compile_source(&mut self, src: &str) -> Result<usize, Error> {
        let mut tokens = Token::lexer(src)
            .spanned()
            .map(|(token, span)| match token {
                Ok(token) => Ok((token, span.into())),
                Err(()) => Err(Error::new(
                    span.into(),
                    self.current_file,
                    "unexpected token".to_string(),
                )),
            })
            .collect::<Result<Vec<(Token, Span)>, Error>>()?;

        tokens.push((Token::Eof, Span::from(src.len()..src.len())));

        let parser = Parser::new(src, tokens, self);

        let ast = parser.parse()?;

        let function_index = lower_ast(ast, self)?;

        Ok(function_index)
    }
}

pub fn compile_and_run(file: &str) -> Result<(), Error> {
    let mut compiler = Compiler::default();
    let index = compiler.compile(file)?;

    run_vm(index, compiler.functions)?;

    Ok(())
}
