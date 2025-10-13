use std::collections::HashMap;

use crate::{
    error::kaori_error::KaoriError,
    kaori_error,
    syntax::{binary_op::BinaryOpKind, unary_op::UnaryOpKind},
};

use super::{
    hir_decl::{HirDecl, HirDeclKind},
    hir_expr::{HirExpr, HirExprKind},
    hir_id::HirId,
    hir_node::HirNode,
    hir_stmt::{HirStmt, HirStmtKind},
    hir_ty::{HirTy, HirTyKind},
    type_def::TypeDef,
};

#[derive(Debug, Default)]
pub struct TypeChecker {
    function_return_ty: TypeDef,
    types: HashMap<HirId, HirTy>,
    types_table: HashMap<HirId, TypeDef>,
}

impl TypeChecker {
    pub fn type_check(
        mut self,
        declarations: &[HirDecl],
    ) -> Result<HashMap<HirId, TypeDef>, KaoriError> {
        for declaration in declarations.iter() {
            match &declaration.kind {
                HirDeclKind::Function { .. } => {
                    self.types.insert(declaration.id, declaration.ty.to_owned());
                }
                HirDeclKind::Struct { .. } => {
                    self.types.insert(declaration.id, declaration.ty.to_owned());
                }
                _ => (),
            }
        }

        for declaration in declarations {
            self.type_check_declaration(declaration)?;
        }

        Ok(self.types_table)
    }

    fn type_check_nodes(&mut self, nodes: &[HirNode]) -> Result<(), KaoriError> {
        for node in nodes {
            self.type_check_ast_node(node)?;
        }

        Ok(())
    }

    fn type_check_ast_node(&mut self, node: &HirNode) -> Result<(), KaoriError> {
        match node {
            HirNode::Declaration(declaration) => self.type_check_declaration(declaration),
            HirNode::Statement(statement) => self.type_check_statement(statement),
        }?;

        Ok(())
    }

    fn type_check_declaration(&mut self, declaration: &HirDecl) -> Result<TypeDef, KaoriError> {
        let ty = match &declaration.kind {
            HirDeclKind::Variable { right } => {
                let right = self.type_check_expression(right)?;
                let ty = self.get_type_def(&declaration.ty);

                if right != ty {
                    return Err(kaori_error!(
                        declaration.span,
                        "expected {:#?} type on variable declaration, but found: {:#?}",
                        ty,
                        right
                    ));
                }

                self.types.insert(declaration.id, declaration.ty.to_owned());

                ty
            }
            HirDeclKind::Function { body, parameters } => {
                let return_ty = match &declaration.ty.kind {
                    HirTyKind::Function { return_ty, .. } => self.get_type_def(return_ty),
                    _ => unreachable!(),
                };

                let parameters_ty = parameters
                    .iter()
                    .map(|parameter| self.type_check_declaration(parameter))
                    .collect::<Result<Vec<TypeDef>, KaoriError>>()?;

                let ty = TypeDef::function(parameters_ty, return_ty.to_owned());

                self.function_return_ty = return_ty.to_owned();

                let mut has_return_statement = false;

                for node in body {
                    if let HirNode::Statement(statement) = node
                        && let HirStmtKind::Return(..) = statement.kind
                    {
                        has_return_statement = true;
                        break;
                    }
                }

                if return_ty != TypeDef::Void && !has_return_statement {
                    return Err(kaori_error!(
                        declaration.span,
                        "expected a return statement for this function"
                    ));
                }

                for node in body {
                    self.type_check_ast_node(node)?;
                }

                ty
            }
            HirDeclKind::Struct { fields } => {
                let fields = fields
                    .iter()
                    .map(|field| self.type_check_declaration(field))
                    .collect::<Result<Vec<TypeDef>, KaoriError>>()?;

                TypeDef::struct_(fields)
            }
            HirDeclKind::Parameter => {
                let ty = self.get_type_def(&declaration.ty);

                self.types.insert(declaration.id, declaration.ty.to_owned());

                ty
            }
            HirDeclKind::Field => {
                let ty = self.get_type_def(&declaration.ty);

                self.types.insert(declaration.id, declaration.ty.to_owned());

                ty
            }
        };

        self.types_table.insert(declaration.id, ty.to_owned());

        Ok(ty)
    }

    fn type_check_statement(&mut self, statement: &HirStmt) -> Result<TypeDef, KaoriError> {
        match &statement.kind {
            HirStmtKind::Expression(expression) => {
                self.type_check_expression(expression)?;
            }
            HirStmtKind::Print(expression) => {
                self.type_check_expression(expression)?;
            }
            HirStmtKind::Block(nodes) => {
                self.type_check_nodes(nodes)?;
            }
            HirStmtKind::Branch {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_ty = self.type_check_expression(condition)?;

                let TypeDef::Boolean = condition_ty else {
                    return Err(kaori_error!(
                        condition.span,
                        "expected a boolean for condition, but found {:#?}",
                        condition_ty
                    ));
                };

                self.type_check_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.type_check_statement(branch)?;
                }
            }
            HirStmtKind::Loop {
                init,
                condition,
                block,
                increment,
            } => {
                if let Some(init) = init {
                    self.type_check_declaration(init)?;
                }

                let condition_ty = self.type_check_expression(condition)?;

                let TypeDef::Boolean = condition_ty else {
                    return Err(kaori_error!(
                        condition.span,
                        "expected a boolean for condition, but found {:#?}",
                        condition_ty
                    ));
                };

                self.type_check_statement(block)?;

                if let Some(increment) = increment {
                    self.type_check_statement(increment)?;
                }
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                let ty = match expr {
                    Some(expr) => self.type_check_expression(expr)?,
                    _ => TypeDef::Void,
                };

                if self.function_return_ty != ty {
                    return Err(kaori_error!(
                        statement.span,
                        "expected a return type of {:#?}, but found {:#?}",
                        self.function_return_ty,
                        ty
                    ));
                }
            }
        };

        Ok(TypeDef::Void)
    }

    fn type_check_expression(&mut self, expression: &HirExpr) -> Result<TypeDef, KaoriError> {
        let ty = match &expression.kind {
            HirExprKind::Assign { left, right } => {
                let right_ty = self.type_check_expression(right)?;
                let left_ty = self.type_check_expression(left)?;

                if left_ty == right_ty {
                    TypeDef::Void
                } else {
                    return Err(kaori_error!(
                        expression.span,
                        "expected left and right side of assign operator to be the same type, but found: {:#?} and {:#?}",
                        left_ty,
                        right_ty
                    ));
                }
            }
            HirExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let left_ty = self.type_check_expression(left)?;
                let right_ty = self.type_check_expression(right)?;

                use BinaryOpKind as Op;
                use TypeDef as Ty;

                match (&left_ty, operator.kind, &right_ty) {
                    (
                        Ty::Number,
                        Op::Add | Op::Subtract | Op::Multiply | Op::Divide | Op::Modulo,
                        Ty::Number,
                    ) => Ty::Number,

                    (Ty::Boolean, Op::And | Op::Or, Ty::Boolean) => Ty::Boolean,

                    (left_ty, Op::Equal | Op::NotEqual, right_ty) if left_ty == right_ty => {
                        Ty::Boolean
                    }

                    (
                        Ty::Number,
                        Op::Greater | Op::GreaterEqual | Op::Less | Op::LessEqual,
                        Ty::Number,
                    ) => Ty::Boolean,

                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "expected valid types for {:#?} operator, but found {:#?} and {:#?}",
                            operator.kind,
                            left_ty,
                            right_ty
                        ));
                    }
                }
            }
            HirExprKind::Unary { right, operator } => {
                let right_ty = self.type_check_expression(right)?;

                match (operator.kind, &right_ty) {
                    (UnaryOpKind::Negate, TypeDef::Number) => TypeDef::Number,
                    (UnaryOpKind::Not, TypeDef::Boolean) => TypeDef::Boolean,
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "expected valid type for {:#?} operator, but found {:#?}",
                            operator.kind,
                            right_ty
                        ));
                    }
                }
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let TypeDef::Function {
                    parameters,
                    return_ty,
                } = self.type_check_expression(callee)?
                else {
                    return Err(kaori_error!(
                        callee.span,
                        "expected a valid callable in that function call",
                    ));
                };

                if arguments.len() != parameters.len() {
                    return Err(kaori_error!(
                        expression.span,
                        "expected the same number of arguments and parameters for this function call",
                    ));
                }

                for (argument, parameter) in arguments.iter().zip(parameters) {
                    let argument = self.type_check_expression(argument)?;

                    if argument != parameter {
                        return Err(kaori_error!(
                            expression.span,
                            "expected argument and parameter of the same type, but found: {:#?} and {:#?}",
                            argument,
                            parameter
                        ));
                    }
                }

                *return_ty
            }
            HirExprKind::Variable(id) => {
                let hir_ty = self.types.get(id).unwrap().to_owned();

                self.get_type_def(&hir_ty)
            }
            HirExprKind::Boolean(..) => TypeDef::Boolean,
            HirExprKind::Number(..) => TypeDef::Number,
            HirExprKind::String(..) => TypeDef::String,
            HirExprKind::Function(id) => {
                let hir_ty = self.types.get(id).unwrap().to_owned();

                self.get_type_def(&hir_ty)
            }
        };

        self.types_table.insert(expression.id, ty.to_owned());

        Ok(ty)
    }

    fn get_type_def(&self, ty: &HirTy) -> TypeDef {
        match &ty.kind {
            HirTyKind::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters
                    .iter()
                    .map(|parameter| self.get_type_def(parameter))
                    .collect();

                let return_ty = self.get_type_def(return_ty);

                TypeDef::function(parameters, return_ty)
            }
            HirTyKind::TypeRef(id) => {
                let hir_ty = self.types.get(id).unwrap();

                self.get_type_def(hir_ty)
            }
            HirTyKind::Struct { fields } => {
                let fields = fields
                    .iter()
                    .map(|field| self.get_type_def(field))
                    .collect();

                TypeDef::struct_(fields)
            }
            HirTyKind::Bool => TypeDef::Boolean,
            HirTyKind::Number => TypeDef::Number,
            HirTyKind::Void => TypeDef::Void,
        }
    }
}
