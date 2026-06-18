use crate::{
    compiler::{Compiler, INTERNER},
    runtime::{
        function::Function,
        instruction::Instruction,
        operands::{Const, Reg},
    },
    std::native_function::NativeFunction,
};

impl Compiler {
    pub fn compile_native_functions(
        &mut self,
        functions: &[(&'static str, NativeFunction)],
    ) -> usize {
        let mut file = Function::default();

        file.emit_instruction(Instruction::CreateMap { dest: Reg(0) });

        for (name, function) in functions.iter().copied() {
            let index = self.native_functions.len();

            self.native_functions.push(function);

            let key = {
                let symbol = INTERNER.lock().unwrap().get_or_intern(name);
                let index = file.store_string_const(symbol);
                Const::from(index)
            };
            let value = {
                let index = file.store_native_function_const(index);

                Const::from(index)
            };

            file.emit_instruction(Instruction::LoadConst {
                dest: Reg(1),
                src: value,
            });

            file.emit_instruction(Instruction::SetProperty {
                object: Reg(0),
                key,
                value: Reg(1),
            });
        }

        file.emit_instruction(Instruction::Return { src: Reg(0) });

        file.frame_size = 2;

        let index = self.functions.len();
        self.functions.push(file);

        index
    }
}
