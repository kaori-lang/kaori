use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use logos::Logos;

use crate::{
    codegen::lower_ast::lower_ast,
    diagnostics::error::Error,
    report_error,
    runtime::{function::Function, value::Value, vm::run_vm},
    syntax::{
        ast::Spanned,
        parser::Parser,
        token::{Span, Token},
    },
    util::string_interner::{StringInterner, Symbol},
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

#[derive(Default)]
pub struct Compiler {
    pub functions: Vec<Function>,
    pub compiled_files: HashMap<Symbol, usize>,
    pub visited: HashSet<Symbol>,
    pub path: Symbol,
}

impl Compiler {
    pub fn compile(&mut self, file: &str) -> Result<usize, Error> {
        let path = INTERNER.lock().unwrap().get_or_intern(file);
        let span = Span::default();

        let index = self.compile_file(Spanned::new(path, span))?;

        for function in self.functions.iter() {
            println!("{}", function);
        }

        Ok(index)
    }

    pub fn compile_file(&mut self, path: Spanned<Symbol>) -> Result<usize, Error> {
        if let Some(index) = self.compiled_files.get(&path.value).copied() {
            return Ok(index);
        }

        if self.visited.contains(&path.value) {
            return report_error!(path.span, self.path, "circular import detected");
        }

        let src = match read_to_string(INTERNER.lock().unwrap().resolve(path.value)) {
            Ok(source) => source,
            Err(..) => {
                return report_error!(path.span, self.path, "expected a valid file path");
            }
        };

        self.visited.insert(path.value);

        let previous_path = self.path;
        self.path = path.value;

        let function_index = self.compile_source(&src)?;

        self.compiled_files.insert(path.value, function_index);
        self.path = previous_path;

        Ok(function_index)
    }

    fn compile_source(&mut self, src: &str) -> Result<usize, Error> {
        let mut tokens = Token::lexer(src)
            .spanned()
            .map(|(token, span)| match token {
                Ok(token) => Ok((token, span.into())),
                Err(()) => report_error!(span.into(), self.path, "unexpected token"),
            })
            .collect::<Result<Vec<(Token, Span)>, Error>>()?;
        tokens.push((Token::Eof, Span::from(src.len()..src.len())));

        let parser = Parser::new(src, tokens, self);

        let ast = parser.parse()?;

        let function_index = lower_ast(ast, self)?;

        Ok(function_index)
    }
}

pub fn compile_and_run(file: &str) -> Result<Value, Error> {
    let mut compiler = Compiler::default();
    let index = compiler.compile(file)?;
    let value = run_vm(index, compiler.functions)?;

    Ok(value)
}
