use regex::{Captures, Regex};

use crate::{interpreter::environment::Environment, lexer::data::Data, yf_error::ErrorType};

pub struct StringFormatter {
    format_re: Regex,
}

impl StringFormatter {
    pub fn new() -> Self {
        Self {
            format_re: Regex::new(r"\{([0-9a-zA-Z_]*)\}").unwrap(),
        }
    }

    pub fn format(&self, string_literal: &str, env: &Environment) -> Result<String, ErrorType> {
        let mut has_error = false;

        let call_back_function = |caps: &Captures| -> String {
            let identifier = &caps[1];

            let data = match env.get_symbol(identifier) {
                Ok(Data::String(value)) => value,
                Ok(Data::Boolean(value)) => value.to_string(),
                Ok(Data::Float(value)) => value.to_string(),
                _ => {
                    has_error = true;
                    "".to_string()
                }
            };

            return data;
        };

        let formatted_string_literal = self
            .format_re
            .replace_all(string_literal, call_back_function);

        if has_error {
            return Err(ErrorType::SyntaxError);
        }

        return Ok(formatted_string_literal.to_string());
    }
}
