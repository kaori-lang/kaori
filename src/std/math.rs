use crate::runtime::value::Value;

use crate::std::native_function::NativeFunction;

pub static MATH_FUNCTIONS: &[(&str, NativeFunction)] = &[
    (
        "sin",
        NativeFunction::new(|args| Value::number(args[0].as_number().sin())),
    ),
    (
        "cos",
        NativeFunction::new(|args| Value::number(args[0].as_number().cos())),
    ),
    (
        "tan",
        NativeFunction::new(|args| Value::number(args[0].as_number().tan())),
    ),
    (
        "sqrt",
        NativeFunction::new(|args| Value::number(args[0].as_number().sqrt())),
    ),
    (
        "abs",
        NativeFunction::new(|args| Value::number(args[0].as_number().abs())),
    ),
    (
        "floor",
        NativeFunction::new(|args| Value::number(args[0].as_number().floor())),
    ),
    (
        "ceil",
        NativeFunction::new(|args| Value::number(args[0].as_number().ceil())),
    ),
    (
        "round",
        NativeFunction::new(|args| Value::number(args[0].as_number().round())),
    ),
    (
        "pow",
        NativeFunction::new(|args| {
            Value::number(args[0].as_number().powf(args[1].as_number()))
        }),
    ),
    (
        "log",
        NativeFunction::new(|args| Value::number(args[0].as_number().ln())),
    ),
    (
        "log2",
        NativeFunction::new(|args| Value::number(args[0].as_number().log2())),
    ),
    (
        "log10",
        NativeFunction::new(|args| Value::number(args[0].as_number().log10())),
    ),
    (
        "min",
        NativeFunction::new(|args| {
            Value::number(args[0].as_number().min(args[1].as_number()))
        }),
    ),
    (
        "max",
        NativeFunction::new(|args| {
            Value::number(args[0].as_number().max(args[1].as_number()))
        }),
    ),
    (
        "clamp",
        NativeFunction::new(|args| {
            Value::number(
                args[0]
                    .as_number()
                    .clamp(args[1].as_number(), args[2].as_number()),
            )
        }),
    ),
    ("pi", NativeFunction::new(|_args| Value::number(std::f64::consts::PI))),
    ("e", NativeFunction::new(|_args| Value::number(std::f64::consts::E))),
];
