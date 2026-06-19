use crate::diagnostics::error::Error;
use crate::runtime::{native_function::NativeFunction, value::Value};
use crate::syntax::token::Span;
use crate::util::string_interner::Symbol;

fn arity_error(name: &str, expected: usize, got: usize) -> Error {
    Error::new(
        Span::default(),
        Symbol::default(),
        format!("'{name}' expects {expected} argument(s), got {got}"),
    )
}

fn type_error(name: &str) -> Error {
    Error::new(
        Span::default(),
        Symbol::default(),
        format!("'{name}' expects numeric argument(s)"),
    )
}

pub static MATH_FUNCTIONS: &[(&str, NativeFunction)] = &[
    (
        "sin",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("sin", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("sin"));
            }
            Ok(Value::number(args[0].as_number().sin()))
        }),
    ),
    (
        "cos",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("cos", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("cos"));
            }
            Ok(Value::number(args[0].as_number().cos()))
        }),
    ),
    (
        "tan",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("tan", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("tan"));
            }
            Ok(Value::number(args[0].as_number().tan()))
        }),
    ),
    (
        "sqrt",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("sqrt", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("sqrt"));
            }
            Ok(Value::number(args[0].as_number().sqrt()))
        }),
    ),
    (
        "abs",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("abs", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("abs"));
            }
            Ok(Value::number(args[0].as_number().abs()))
        }),
    ),
    (
        "floor",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("floor", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("floor"));
            }
            Ok(Value::number(args[0].as_number().floor()))
        }),
    ),
    (
        "ceil",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("ceil", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("ceil"));
            }
            Ok(Value::number(args[0].as_number().ceil()))
        }),
    ),
    (
        "round",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("round", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("round"));
            }
            Ok(Value::number(args[0].as_number().round()))
        }),
    ),
    (
        "pow",
        NativeFunction::new(|args| {
            if args.len() < 2 {
                return Err(arity_error("pow", 2, args.len()));
            }
            if !args[0].is_number() || !args[1].is_number() {
                return Err(type_error("pow"));
            }
            Ok(Value::number(args[0].as_number().powf(args[1].as_number())))
        }),
    ),
    (
        "log",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("log", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("log"));
            }
            Ok(Value::number(args[0].as_number().ln()))
        }),
    ),
    (
        "log2",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("log2", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("log2"));
            }
            Ok(Value::number(args[0].as_number().log2()))
        }),
    ),
    (
        "log10",
        NativeFunction::new(|args| {
            if args.is_empty() {
                return Err(arity_error("log10", 1, args.len()));
            }
            if !args[0].is_number() {
                return Err(type_error("log10"));
            }
            Ok(Value::number(args[0].as_number().log10()))
        }),
    ),
    (
        "min",
        NativeFunction::new(|args| {
            if args.len() < 2 {
                return Err(arity_error("min", 2, args.len()));
            }
            if !args[0].is_number() || !args[1].is_number() {
                return Err(type_error("min"));
            }
            Ok(Value::number(args[0].as_number().min(args[1].as_number())))
        }),
    ),
    (
        "max",
        NativeFunction::new(|args| {
            if args.len() < 2 {
                return Err(arity_error("max", 2, args.len()));
            }
            if !args[0].is_number() || !args[1].is_number() {
                return Err(type_error("max"));
            }
            Ok(Value::number(args[0].as_number().max(args[1].as_number())))
        }),
    ),
    (
        "clamp",
        NativeFunction::new(|args| {
            if args.len() < 3 {
                return Err(arity_error("clamp", 3, args.len()));
            }
            if !args[0].is_number()
                || !args[1].is_number()
                || !args[2].is_number()
            {
                return Err(type_error("clamp"));
            }
            Ok(Value::number(
                args[0]
                    .as_number()
                    .clamp(args[1].as_number(), args[2].as_number()),
            ))
        }),
    ),
    (
        "pi",
        NativeFunction::new(|_args| Ok(Value::number(std::f64::consts::PI))),
    ),
    ("e", NativeFunction::new(|_args| Ok(Value::number(std::f64::consts::E)))),
];
