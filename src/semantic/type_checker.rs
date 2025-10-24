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
    r#type::Type,
};

#[derive(Debug, Default)]
pub struct TypeChecker {
    function_return_ty: Type,
    types: HashMap<HirId, Type>,
}

impl TypeChecker {
    pub fn type_check(
        mut self,
        declarations: &[HirDecl],
    ) -> Result<HashMap<HirId, Type>, KaoriError> {
        for declaration in declarations.iter() {
            match &declaration.kind {
                HirDeclKind::Function {
                    parameters,
                    return_ty,
                    ..
                } => {
                    self.types.insert(declaration.id, ty.to_owned());

                    let return_ty = if let Some(ty) = return_ty {
                        self.create_type_def(ty)
                    } else {
                        Type::Void
                    };

                    //self.types.insert(k, v)
                }
                HirDeclKind::Struct { fields } => {
                    self.types.insert(declaration.id, ty.to_owned());
                }
                _ => (),
            }
        }

        for declaration in declarations {
            self.type_check_declaration(declaration)?;
        }

        Ok(self.types)
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

    fn type_check_declaration(&mut self, declaration: &HirDecl) -> Result<Type, KaoriError> {
        let ty = match &declaration.kind {
            HirDeclKind::Variable { right, ty } => {
                let right = self.type_check_expression(right)?;

                match ty {
                    Some(ty) => {
                        let ty = self.create_type_def(ty);

                        if right != ty {
                            return Err(kaori_error!(
                                declaration.span,
                                "expected {:#?} type on variable declaration, but found: {:#?}",
                                ty,
                                right
                            ));
                        }

                        ty
                    }
                    _ => right.to_owned(),
                }
            }
            HirDeclKind::Function {
                body,
                parameters,
                return_ty,
            } => {
                for parameter in parameters {
                    let ty = self.create_type_def(&parameter.ty);

                    self.types.insert(parameter.id, ty);
                }

                let return_ty = match return_ty {
                    Some(ty) => self.create_type_def(ty),
                    _ => Type::Void,
                };

                self.function_return_ty = return_ty.to_owned();

                for node in body {
                    self.type_check_ast_node(node)?;
                }

                ty
            }
            HirDeclKind::Struct { fields, .. } => {
                let fields = fields
                    .iter()
                    .map(|field| {
                        let ty = self.create_type_def(&field.ty);

                        self.types.insert(field.id, ty.to_owned());

                        ty
                    })
                    .collect();

                Type::struct_(fields)
            }
        };

        self.types.insert(declaration.id, ty.to_owned());

        Ok(ty)
    }

    fn type_check_statement(&mut self, statement: &HirStmt) -> Result<Type, KaoriError> {
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

                let Type::Boolean = condition_ty else {
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

                let Type::Boolean = condition_ty else {
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
                    _ => Type::Void,
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

        Ok(Type::Void)
    }

    fn type_check_expression(&mut self, expression: &HirExpr) -> Result<Type, KaoriError> {
        let ty = match &expression.kind {
            HirExprKind::Assign { left, right } => {
                let right_ty = self.type_check_expression(right)?;
                let left_ty = self.type_check_expression(left)?;

                if left_ty == right_ty {
                    Type::Void
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
                use Type as Ty;

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
                    (UnaryOpKind::Negate, Type::Number) => Type::Number,
                    (UnaryOpKind::Not, Type::Boolean) => Type::Boolean,
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
                let Type::Function {
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
            HirExprKind::Variable(id) => self.types.get(id).unwrap().to_owned(),
            HirExprKind::Boolean(..) => Type::Boolean,
            HirExprKind::Number(..) => Type::Number,
            HirExprKind::String(..) => Type::String,
            HirExprKind::Function(id) => self.types.get(id).unwrap().to_owned(),
        };

        self.types.insert(expression.id, ty.to_owned());

        Ok(ty)
    }

    fn get_type_def(&self, id: HirId) -> Type {
        match self.types.get(k).unwrap() {
            Type::Struct { fields }
        }
    }

    fn create_type_def(&self, ty: &HirTy) -> Type {
        match &ty.kind {
            HirTyKind::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters.iter().map(|parameter| parameter.id).collect();

                let return_ty = return_ty.id;

                Type::function(parameters, return_ty)
            }
            HirTyKind::TypeRef(id) => {
                if let Some(ty) = self.types.get(id) {
                    ty.to_owned()
                } else {
                    Type::type_ref(*id)
                }
            }
            HirTyKind::Struct { fields } => {
                let fields = fields.iter().map(|field| field.id).collect();

                Type::struct_(fields)
            }
            HirTyKind::Bool => Type::Boolean,
            HirTyKind::Number => Type::Number,
        }
    }
}
