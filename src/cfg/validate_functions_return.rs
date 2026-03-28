use crate::{error::kaori_error::KaoriError, kaori_error};

use super::{basic_block::Terminator, function::Function, graph_traversal::reversed_postorder};

pub fn validate_functions_return(functions: &mut [Function]) -> Result<(), KaoriError> {
    for function in functions {
        let reversed_postorder = reversed_postorder(&function.basic_blocks);

        for index in reversed_postorder {
            let basic_block = &mut function.basic_blocks[index];

            if basic_block.terminator.is_none() {
                if function.return_required {
                    return Err(kaori_error!(function.span, "return statement is required"));
                } else {
                    basic_block.terminator = Some(Terminator::Return { src: None });
                }
            }
        }
    }

    Ok(())
}
