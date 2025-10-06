use std::collections::HashMap;

use crate::{
    error::kaori_error::KaoriError,
    kaori_error,
    lexer::span::Span,
    syntax::{
        assign_op::AssignOpKind,
        ast_id::AstId,
        ast_node::AstNode,
        binary_op::{BinaryOp, BinaryOpKind},
        decl::{Decl, DeclKind},
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
        ty::{Ty, TyKind},
    },
};

use super::{
    hir_decl::HirDecl,
    hir_expr::{HirExpr, HirExprKind},
    hir_id::HirId,
    hir_node::HirNode,
    hir_stmt::HirStmt,
    hir_ty::HirTy,
    symbol::SymbolKind,
    symbol_table::SymbolTable,
};

#[derive(Default)]
pub struct Resolver {
    symbol_table: SymbolTable,
    active_loops: u8,
    local_scope: bool,
    ids: HashMap<AstId, HirId>,
}

impl Resolver {
    pub fn enter_function(&mut self) {
        self.symbol_table.enter_scope();
        self.local_scope = true;
    }

    pub fn exit_function(&mut self) {
        self.symbol_table.exit_scope();
        self.local_scope = false;
    }

    pub fn generate_hir_id(&mut self, id: AstId) -> HirId {
        let hir_id = HirId::default();

        self.ids.insert(id, hir_id);

        hir_id
    }

    fn resolve_main_function(&mut self, declarations: &mut [Decl]) -> Result<(), KaoriError> {
        for (index, declaration) in declarations.iter().enumerate() {
            if let DeclKind::Function { name, .. } = &declaration.kind
                && name == "main"
            {
                declarations.swap(0, index);

                return Ok(());
            }
        }

        Err(kaori_error!(
            Span::default(),
            "expected a main function to be declared in the program"
        ))
    }

    fn resolve_declaration_scope(&self, declaration: &Decl) -> Result<(), KaoriError> {
        let has_error = match &declaration.kind {
            DeclKind::Function { .. } if self.local_scope => true,
            DeclKind::Struct { .. } if self.local_scope => true,
            DeclKind::Variable { .. } if !self.local_scope => true,
            _ => false,
        };

        if has_error {
            Err(kaori_error!(
                declaration.span,
                "expected declaration to be made in the correct scope"
            ))
        } else {
            Ok(())
        }
    }

    pub fn resolve(&mut self, declarations: &mut [Decl]) -> Result<Vec<HirDecl>, KaoriError> {
        self.resolve_main_function(declarations)?;

        for declaration in declarations.iter() {
            match &declaration.kind {
                DeclKind::Function { name, .. } => {
                    if self.symbol_table.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let id = self.generate_hir_id(declaration.id);

                    self.symbol_table.declare_function(id, name.to_owned());
                }
                DeclKind::Struct { name, .. } => {
                    if self.symbol_table.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let id = self.generate_hir_id(declaration.id);

                    self.symbol_table.declare_struct(id, name.to_owned());
                }
                _ => (),
            };
        }

        let declarations = declarations
            .iter()
            .map(|declaration| self.resolve_declaration(declaration))
            .collect::<Result<Vec<HirDecl>, KaoriError>>()?;

        Ok(declarations)
    }

    fn resolve_node(&mut self, node: &AstNode) -> Result<HirNode, KaoriError> {
        let node = match node {
            AstNode::Declaration(declaration) => {
                let declaration = self.resolve_declaration(declaration)?;
                HirNode::from(declaration)
            }
            AstNode::Statement(statement) => {
                let statement = self.resolve_statement(statement)?;
                HirNode::from(statement)
            }
        };

        Ok(node)
    }

    fn resolve_declaration(&mut self, declaration: &Decl) -> Result<HirDecl, KaoriError> {
        self.resolve_declaration_scope(declaration)?;

        let hir_decl = match &declaration.kind {
            DeclKind::Parameter { name } => {
                if self.symbol_table.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "function can't have parameters with the same name: {}",
                        name,
                    ));
                };

                let id = HirId::default();

                self.symbol_table.declare_variable(id, name.to_owned());

                let ty = self.resolve_type(&declaration.ty)?;

                HirDecl::parameter(id, ty, declaration.span)
            }
            DeclKind::Field { name } => {
                if self.symbol_table.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "struct can't have fields with the same name: {}",
                        name,
                    ));
                };

                let id = HirId::default();

                self.symbol_table.declare_variable(id, name.to_owned());

                let ty = self.resolve_type(&declaration.ty)?;

                HirDecl::field(id, ty, declaration.span)
            }
            DeclKind::Variable { name, right } => {
                let right = self.resolve_expression(right)?;

                if self.symbol_table.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                };

                let id = HirId::default();

                self.symbol_table.declare_variable(id, name.to_owned());

                let ty = self.resolve_type(&declaration.ty)?;

                HirDecl::variable(id, right, ty, declaration.span)
            }
            DeclKind::Function {
                parameters, body, ..
            } => {
                self.enter_function();

                let parameters = parameters
                    .iter()
                    .map(|parameter| self.resolve_declaration(parameter))
                    .collect::<Result<Vec<HirDecl>, KaoriError>>()?;

                let body = body
                    .iter()
                    .map(|node| self.resolve_node(node))
                    .collect::<Result<Vec<HirNode>, KaoriError>>()?;

                let ty = self.resolve_type(&declaration.ty)?;

                self.exit_function();

                let id = self.ids.get(&declaration.id).unwrap();

                HirDecl::function(*id, parameters, body, ty, declaration.span)
            }
            DeclKind::Struct { fields, .. } => {
                let fields = fields
                    .iter()
                    .map(|field| self.resolve_declaration(field))
                    .collect::<Result<Vec<HirDecl>, KaoriError>>()?;

                let ty = self.resolve_type(&declaration.ty)?;
                let id = self.ids.get(&declaration.id).unwrap();

                HirDecl::struct_(*id, fields, ty, declaration.span)
            }
        };

        Ok(hir_decl)
    }

    fn resolve_statement(&mut self, statement: &Stmt) -> Result<HirStmt, KaoriError> {
        let hir_stmt = match &statement.kind {
            StmtKind::Expression(expression) => {
                let expr = self.resolve_expression(expression)?;

                HirStmt::expression(expr, statement.span)
            }
            StmtKind::Print(expression) => {
                let expr = self.resolve_expression(expression)?;

                HirStmt::print(expr, statement.span)
            }
            StmtKind::Block(nodes) => {
                self.symbol_table.enter_scope();

                let nodes = nodes
                    .iter()
                    .map(|node| self.resolve_node(node))
                    .collect::<Result<Vec<HirNode>, KaoriError>>()?;

                self.symbol_table.exit_scope();

                HirStmt::block(nodes, statement.span)
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.resolve_expression(condition)?;
                let then_branch = self.resolve_statement(then_branch)?;
                let else_branch = match else_branch {
                    Some(branch) => Some(self.resolve_statement(branch)?),
                    _ => None,
                };

                HirStmt::branch_(condition, then_branch, else_branch, statement.span)
            }
            StmtKind::WhileLoop { condition, block } => {
                let init = None;
                let condition = self.resolve_expression(condition)?;
                let increment = None;

                self.active_loops += 1;
                let block = self.resolve_statement(block)?;
                self.active_loops -= 1;

                HirStmt::loop_(init, condition, block, increment, statement.span)
            }
            StmtKind::ForLoop {
                init,
                condition,
                increment,
                block,
            } => {
                self.symbol_table.enter_scope();

                let init = Some(self.resolve_declaration(init)?);
                let condition = self.resolve_expression(condition)?;
                let increment = Some(self.resolve_statement(increment)?);

                self.active_loops += 1;
                let block = self.resolve_statement(block)?;
                self.active_loops -= 1;

                self.symbol_table.exit_scope();

                HirStmt::loop_(init, condition, block, increment, statement.span)
            }
            StmtKind::Break => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        statement.span,
                        "break statement can't appear outside of loops"
                    ));
                }

                HirStmt::break_(statement.span)
            }
            StmtKind::Continue => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        statement.span,
                        "continue statement can't appear outside of loops"
                    ));
                }

                HirStmt::continue_(statement.span)
            }
            StmtKind::Return(expr) => {
                let expr = match expr {
                    Some(e) => Some(self.resolve_expression(e)?),
                    _ => None,
                };

                HirStmt::return_(expr, statement.span)
            }
        };

        Ok(hir_stmt)
    }

    fn resolve_expression(&self, expression: &Expr) -> Result<HirExpr, KaoriError> {
        let hir_expr = match &expression.kind {
            ExprKind::Assign {
                operator,
                left,
                right,
            } => {
                let right = self.resolve_expression(right)?;
                let left = self.resolve_expression(left)?;

                let HirExprKind::VariableRef(..) = &left.kind else {
                    return Err(kaori_error!(
                        left.span,
                        "expected a valid left hand side to assign values to"
                    ));
                };

                let operator_kind = match &operator.kind {
                    AssignOpKind::AddAssign => BinaryOpKind::Add,
                    AssignOpKind::SubtractAssign => BinaryOpKind::Subtract,
                    AssignOpKind::MultiplyAssign => BinaryOpKind::Multiply,
                    AssignOpKind::DivideAssign => BinaryOpKind::Divide,
                    AssignOpKind::ModuloAssign => BinaryOpKind::Modulo,
                    _ => return Ok(HirExpr::assign(left, right, expression.span)),
                };

                let operator = BinaryOp::new(operator_kind, operator.span);

                let right =
                    HirExpr::binary(operator, left.to_owned(), right.to_owned(), right.span);

                HirExpr::assign(left, right, expression.span)
            }
            ExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.resolve_expression(left)?;
                let right = self.resolve_expression(right)?;

                HirExpr::binary(*operator, left, right, expression.span)
            }
            ExprKind::Unary { right, operator } => {
                let right = self.resolve_expression(right)?;

                HirExpr::unary(*operator, right, expression.span)
            }
            ExprKind::FunctionCall { callee, arguments } => {
                let callee = self.resolve_expression(callee)?;
                let arguments = arguments
                    .iter()
                    .map(|argument| self.resolve_expression(argument))
                    .collect::<Result<Vec<HirExpr>, KaoriError>>()?;

                HirExpr::function_call(callee, arguments, expression.span)
            }
            ExprKind::NumberLiteral(value) => HirExpr::number(*value, expression.span),
            ExprKind::BooleanLiteral(value) => HirExpr::boolean(*value, expression.span),
            ExprKind::StringLiteral(value) => HirExpr::string(value.to_owned(), expression.span),
            ExprKind::Identifier(name) => {
                let Some(symbol) = self.symbol_table.search(name) else {
                    return Err(kaori_error!(expression.span, "{} is not declared", name));
                };

                match symbol.kind {
                    SymbolKind::Function => HirExpr::function_ref(symbol.id, expression.span),
                    SymbolKind::Variable => HirExpr::variable_ref(symbol.id, expression.span),
                    SymbolKind::Struct => {
                        return Err(kaori_error!(expression.span, "{} is not a value", name));
                    }
                }
            }
        };

        Ok(hir_expr)
    }

    fn resolve_type(&mut self, ty: &Ty) -> Result<HirTy, KaoriError> {
        let hir_ty = match &ty.kind {
            TyKind::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters
                    .iter()
                    .map(|parameter| self.resolve_type(parameter))
                    .collect::<Result<Vec<HirTy>, KaoriError>>()?;

                let return_ty = match return_ty {
                    Some(ty) => Some(self.resolve_type(ty)?),
                    _ => None,
                };

                HirTy::function(parameters, return_ty, ty.span)
            }
            TyKind::Struct { fields } => {
                let fields = fields
                    .iter()
                    .map(|field| self.resolve_type(field))
                    .collect::<Result<Vec<HirTy>, KaoriError>>()?;

                HirTy::struct_(fields, ty.span)
            }
            TyKind::Identifier(name) => {
                let Some(symbol) = self.symbol_table.search(name) else {
                    return Err(kaori_error!(ty.span, "{} type is not declared", name));
                };

                let SymbolKind::Struct = symbol.kind else {
                    return Err(kaori_error!(ty.span, "{} is not a valid type", name));
                };

                HirTy::type_ref(symbol.id, ty.span)
            }
            TyKind::Bool => HirTy::bool(ty.span),
            TyKind::Number => HirTy::number(ty.span),
        };

        Ok(hir_ty)
    }
}
