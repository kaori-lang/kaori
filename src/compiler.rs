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
                    "file not found".to_string(),
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

    pub fn compile_file(&mut self, path: &[Spanned<Symbol>]) -> Result<usize, Error> {
        let mut path_buffer = PathBuf::new();

        for symbol in path {
            path_buffer.push(INTERNER.lock().unwrap().resolve(symbol.value));
        }

        path_buffer.add_extension("kr");

        if let Some(s) = path_buffer.to_str() {
            println!("{}", s);
        }

        /*  if let Some(compilation) = self.compiled_files.get(&path_buffer).copied() {
                   match compilation {
                       Compilation::Incomplete => {
                           return report_error!(path.span, self.path, "circular import detected");
                       }
                       Compilation::Function(index) => return Ok(index),
                   }
               }

               let src = match read_to_string(path_buffer) {
                   Ok(source) => source,
                   Err(..) => {
                       return report_error!(path.span, self.path, "expected a valid file path");
                   }
               };

               self.compiled_files.insert(path.value);

               let previous_path = self.path;
               self.path = path.value;

               let function_index = self.compile_source(&src)?;

               self.compiled_files.insert(path.value, function_index);
               self.path = previous_path;
        */
        let function_index = 0;
        Ok(function_index)
    }

    fn compile_source(&mut self, src: &str) -> Result<usize, Error> {
        let mut tokens = Token::lexer(src)
            .spanned()
            .map(|(token, span)| match token {
                Ok(token) => Ok((token, span.into())),
                Err(()) => report_error!(span.into(), self.current_file, "unexpected token"),
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

    //let value = run_vm(index, compiler.functions)?;

    let value = Value::nil();

    Ok(value)
}
