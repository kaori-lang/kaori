use std::collections::HashMap;

use crate::{
    ast::{binary_op::BinaryOpKind, unary_op::UnaryOpKind},
    error::kaori_error::KaoriError,
    hir::{
        decl::{Decl, DeclKind},
        expr::{Expr, ExprKind},
        node::Node,
        node_id::NodeId,
        stmt::{Stmt, StmtKind},
    },
};

use super::{
    active_loops::ActiveLoops,
    basic_block::{BasicBlock, Terminator},
    constant_pool::ConstantPool,
    function::{Function, FunctionId},
    instruction::Instruction,
    operand::Operand,
};

pub fn build_functions_graph(declarations: &[Decl]) -> Result<Vec<Function>, KaoriError> {
    let mut node_to_function = HashMap::new();

    for declaration in declarations {
        if let DeclKind::Function { .. } = &declaration.kind {
            node_to_function.insert(declaration.id, FunctionId::default());
        }
    }

    let mut functions = Vec::new();

    for declaration in declarations {
        if let DeclKind::Function { .. } = &declaration.kind {
            let mut ctx = FunctionContext::new(&node_to_function);

            ctx.visit_declaration(declaration)?;

            let id = *node_to_function.get(&declaration.id).unwrap();

            let function = Function::new(
                id,
                ctx.basic_blocks,
                ctx.constant_pool.constants,
                ctx.variables.len(),
            );

            functions.push(function);
        }
    }

    Ok(functions)
}

pub struct FunctionContext<'a> {
    index: usize,
    variables: HashMap<NodeId, Operand>,
    constant_pool: ConstantPool,
    basic_blocks: Vec<BasicBlock>,
    active_loops: ActiveLoops,
    node_to_function: &'a HashMap<NodeId, FunctionId>,
}

impl<'a> FunctionContext<'a> {
    pub fn new(node_to_function: &'a HashMap<NodeId, FunctionId>) -> Self {
        Self {
            index: 0,
            variables: HashMap::new(),
            constant_pool: ConstantPool::default(),
            basic_blocks: Vec::new(),
            active_loops: ActiveLoops::default(),
            node_to_function,
        }
    }

    pub fn create_variable(&mut self, id: NodeId) -> Operand {
        let variable = Operand::Variable(self.variables.len());

        self.variables.insert(id, variable);

        variable
    }

    fn emit_instruction(&mut self, instruction: Instruction) {
        let basic_block = &mut self.basic_blocks[self.index];

        if basic_block.terminator.is_none() {
            basic_block.instructions.push(instruction);
        };
    }

    fn set_terminator(&mut self, terminator: Terminator) {
        let basic_block = &mut self.basic_blocks[self.index];

        if basic_block.terminator.is_none() {
            basic_block.terminator = Some(terminator);
        }
    }

    fn create_bb(&mut self) -> usize {
        let index = self.basic_blocks.len();

        let basic_block = BasicBlock::new(index);

        self.basic_blocks.push(basic_block);

        index
    }

    fn visit_nodes(&mut self, nodes: &[Node]) -> Result<(), KaoriError> {
        for node in nodes {
            self.visit_ast_node(node)?;
        }

        Ok(())
    }

    fn visit_ast_node(&mut self, node: &Node) -> Result<(), KaoriError> {
        match node {
            Node::Declaration(declaration) => self.visit_declaration(declaration)?,
            Node::Statement(statement) => self.visit_statement(statement)?,
        };

        Ok(())
    }

    fn visit_declaration(&mut self, declaration: &Decl) -> Result<(), KaoriError> {
        match &declaration.kind {
            DeclKind::Variable { right, .. } => {
                let src = self.visit_expression(right);
                let dest = self.create_variable(declaration.id);

                let instruction = Instruction::move_(dest, src);

                self.emit_instruction(instruction);
            }
            DeclKind::Function {
                body, parameters, ..
            } => {
                let _entry_bb = self.create_bb();

                for parameter in parameters {
                    self.create_variable(parameter.id);
                }

                for node in body {
                    self.visit_ast_node(node)?;
                }

                self.set_terminator(Terminator::Return { src: None });
            }
        };

        Ok(())
    }

    fn visit_statement(&mut self, statement: &Stmt) -> Result<(), KaoriError> {
        match &statement.kind {
            StmtKind::Expression(expression) => {
                self.visit_expression(expression);
            }
            StmtKind::Print(expression) => {
                let src = self.visit_expression(expression);

                let instruction = Instruction::print(src);

                self.emit_instruction(instruction);
            }
            StmtKind::Block(nodes) => {
                self.visit_nodes(nodes)?;
            }
            StmtKind::Branch {
                condition,
                then_branch,
                else_branch,
            } => {
                let src = self.visit_expression(condition);

                let then_bb = self.create_bb();
                let else_bb = self.create_bb();
                let terminator_block = self.create_bb();

                self.set_terminator(Terminator::Branch {
                    src,
                    r#true: then_bb,
                    r#false: else_bb,
                });

                self.index = then_bb;
                self.visit_statement(then_branch)?;
                self.set_terminator(Terminator::Goto(terminator_block));

                self.index = else_bb;
                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }

                self.set_terminator(Terminator::Goto(terminator_block));

                self.index = terminator_block;
            }
            StmtKind::Loop {
                init,
                condition,
                block,
                increment,
            } => {
                if let Some(init) = init {
                    self.visit_declaration(init)?;
                }

                let condition_bb = self.create_bb();
                let block_bb = self.create_bb();
                let terminator_bb = self.create_bb();
                let increment_bb = self.create_bb();

                self.set_terminator(Terminator::Goto(condition_bb));

                self.index = condition_bb;
                let src = self.visit_expression(condition);
                self.set_terminator(Terminator::Branch {
                    src,
                    r#true: block_bb,
                    r#false: terminator_bb,
                });

                self.index = block_bb;
                self.active_loops.push(increment_bb, terminator_bb);
                self.visit_statement(block)?;
                self.active_loops.pop();
                self.set_terminator(Terminator::Goto(increment_bb));

                self.index = increment_bb;
                if let Some(increment) = increment {
                    self.visit_statement(increment)?;
                }
                self.set_terminator(Terminator::Goto(condition_bb));

                self.index = terminator_bb;
            }
            StmtKind::Break => {
                let label = self.active_loops.top();

                self.set_terminator(Terminator::Goto(label.terminator_bb_index));
            }
            StmtKind::Continue => {
                let label = self.active_loops.top();

                self.set_terminator(Terminator::Goto(label.increment_bb_index));
            }
            StmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let src = self.visit_expression(expr);

                    self.set_terminator(Terminator::Return { src: Some(src) });
                } else {
                    self.set_terminator(Terminator::Return { src: None });
                }
            }
        };

        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expr) -> Operand {
        match &expression.kind {
            ExprKind::Assign { left, right } => {
                let dest = self.visit_expression(left);
                let src = self.visit_expression(right);

                let instruction = Instruction::move_(dest, src);

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::LogicalAnd { left, right } => {
                let dest = self.create_variable(expression.id);

                let src1 = self.visit_expression(left);

                self.emit_instruction(Instruction::move_(dest, src1));

                let src2_bb = self.create_bb();
                let terminator = self.create_bb();

                self.set_terminator(Terminator::Branch {
                    src: dest,
                    r#true: src2_bb,
                    r#false: terminator,
                });

                self.index = src2_bb;

                let src2 = self.visit_expression(right);
                self.emit_instruction(Instruction::move_(dest, src2));
                self.set_terminator(Terminator::Goto(terminator));

                self.index = terminator;

                dest
            }
            ExprKind::LogicalOr { left, right } => {
                let dest = self.create_variable(expression.id);

                let src1 = self.visit_expression(left);

                self.emit_instruction(Instruction::move_(dest, src1));

                let src2_bb = self.create_bb();
                let terminator = self.create_bb();

                self.set_terminator(Terminator::Branch {
                    src: dest,
                    r#true: terminator,
                    r#false: src2_bb,
                });

                self.index = src2_bb;

                let src2 = self.visit_expression(right);

                self.emit_instruction(Instruction::move_(dest, src2));

                self.set_terminator(Terminator::Goto(terminator));

                self.index = terminator;

                dest
            }
            ExprKind::LogicalNot { expr } => {
                let src = self.visit_expression(expr);
                let dest = self.create_variable(expression.id);

                self.emit_instruction(Instruction::Not { dest, src });

                dest
            }
            ExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let src1 = self.visit_expression(left);
                let src2 = self.visit_expression(right);
                let dest = self.create_variable(expression.id);

                let instruction = match operator.kind {
                    BinaryOpKind::Add => Instruction::add(dest, src1, src2),
                    BinaryOpKind::Subtract => Instruction::subtract(dest, src1, src2),
                    BinaryOpKind::Multiply => Instruction::multiply(dest, src1, src2),
                    BinaryOpKind::Divide => Instruction::divide(dest, src1, src2),
                    BinaryOpKind::Modulo => Instruction::modulo(dest, src1, src2),
                    BinaryOpKind::Equal => Instruction::equal(dest, src1, src2),
                    BinaryOpKind::NotEqual => Instruction::not_equal(dest, src1, src2),
                    BinaryOpKind::Greater => Instruction::greater(dest, src1, src2),
                    BinaryOpKind::GreaterEqual => Instruction::greater_equal(dest, src1, src2),
                    BinaryOpKind::Less => Instruction::less(dest, src1, src2),
                    BinaryOpKind::LessEqual => Instruction::less_equal(dest, src1, src2),
                };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::Unary { right, operator } => {
                let src = self.visit_expression(right);
                let dest = self.create_variable(expression.id);

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => Instruction::negate(dest, src),
                };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::FunctionCall { callee, arguments } => {
                let dest = self.create_variable(expression.id);

                let arguments_src = arguments
                    .iter()
                    .map(|argument| self.visit_expression(argument))
                    .collect::<Vec<Operand>>();

                let src = self.visit_expression(callee);

                for (index, src) in arguments_src.iter().copied().enumerate() {
                    let dest = Operand::Variable(index);

                    let instruction = Instruction::move_arg(dest, src);

                    self.emit_instruction(instruction);
                }

                let instruction = Instruction::call(dest, src);

                self.emit_instruction(instruction);

                dest
            }

            ExprKind::Variable(id) => *self
                .variables
                .get(id)
                .expect("Variable not found for NodeId"),

            ExprKind::Function(id) => {
                let value = *self
                    .node_to_function
                    .get(id)
                    .expect("FunctionRef points to a missing variable node");

                self.constant_pool.push_function(value)
            }
            ExprKind::String(value) => self.constant_pool.push_string(value.to_owned()),
            ExprKind::Boolean(value) => self.constant_pool.push_boolean(*value),
            ExprKind::Number(value) => self.constant_pool.push_number(*value),
            ExprKind::DictLiteral { fields } => {
                let dest = self.create_variable(expression.id);

                self.emit_instruction(Instruction::create_dict(dest));

                for (key, value) in fields {
                    let key = self.constant_pool.push_string(key.to_owned());
                    let value = self.visit_expression(value);

                    self.emit_instruction(Instruction::set_field(dest, key, value));
                }

                dest
            }
        }
    }
}
