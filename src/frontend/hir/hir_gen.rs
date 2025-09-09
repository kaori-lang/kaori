use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        semantic::{
            environment::Environment,
            resolution_table::{Resolution, ResolutionTable},
            symbol::SymbolKind,
        },
        syntax::{
            ast_node::AstNode,
            decl::{Decl, DeclKind},
            expr::{Expr, ExprKind},
            stmt::{Stmt, StmtKind},
            ty::{Ty, TyKind},
        },
    },
    kaori_error,
};

use super::{
    hir_decl::HirDecl, hir_expr::HirExpr, hir_node::HirNode, hir_stmt::HirStmt, hir_ty::HirTy,
};

pub struct HirGen<'a> {
    environment: Environment,
    active_loops: u8,
    resolution_table: &'a mut ResolutionTable,
}

impl<'a> HirGen<'a> {
    pub fn new(resolution_table: &'a mut ResolutionTable) -> Self {
        Self {
            environment: Environment::default(),
            active_loops: 0,
            resolution_table,
        }
    }

    pub fn generate_hir(&self, declarations: &[Decl]) -> Result<Vec<HirDecl>, KaoriError> {
        for declaration in declarations.iter() {
            match &declaration.kind {
                DeclKind::Function { name, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    self.environment
                        .declare_function(declaration.id, name.to_owned());
                }
                DeclKind::Struct { name, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    self.environment
                        .declare_struct(declaration.id, name.to_owned());
                }
                _ => (),
            };
        }

        let declarations = declarations
            .iter()
            .map(|declaration| self.resolve_declaration(declaration))
            .collect();

        declarations
    }

    fn resolve_node(&self, node: &AstNode) -> Result<HirNode, KaoriError> {
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

    fn resolve_declaration(&self, declaration: &Decl) -> Result<HirDecl, KaoriError> {
        let hir_decl = match &declaration.kind {
            DeclKind::Parameter { name, ty } => {
                let ty = self.resolve_type(ty);
                HirDecl::parameter(name.to_owned(), ty, declaration.span)
            }
            DeclKind::Field { name, ty } => {
                let ty = self.resolve_type(ty);
                HirDecl::field(name.to_owned(), ty, declaration.span)
            }
            DeclKind::Variable { name, right, ty } => {
                let right = self.resolve_expression(right)?;
                let ty = self.resolve_type(ty);

                HirDecl::variable(name.to_owned(), right, ty, declaration.span)
            }
            DeclKind::Function {
                parameters,
                body,
                name,
                return_ty,
            } => {
                let body = body
                    .iter()
                    .map(|node| self.resolve_node(node))
                    .collect::<Result<Vec<HirNode>, KaoriError>>()?;
                let parameters = parameters
                    .iter()
                    .map(|p| self.resolve_declaration(p))
                    .collect::<Result<Vec<HirDecl>, KaoriError>>()?;
                let return_ty = return_ty.as_ref().map(|ty| self.resolve_type(ty));

                HirDecl::function(
                    name.to_owned(),
                    parameters,
                    body,
                    return_ty,
                    declaration.span,
                )
            }
            DeclKind::Struct { name, fields } => {
                let fields = fields
                    .iter()
                    .map(|f| self.resolve_declaration(f))
                    .collect::<Result<Vec<HirDecl>, KaoriError>>()?;

                HirDecl::struct_(name.to_owned(), fields, declaration.span)
            }
        };

        Ok(hir_decl)
    }

    fn resolve_statement(&self, statement: &Stmt) -> Result<HirStmt, KaoriError> {
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
                let nodes = nodes
                    .iter()
                    .map(|node| self.resolve_node(node))
                    .collect::<Result<Vec<HirNode>, KaoriError>>()?;

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
                    None => None,
                };

                HirStmt::branch_(condition, then_branch, else_branch, statement.span)
            }
            StmtKind::WhileLoop { condition, block } => {
                let condition = self.resolve_expression(condition)?;
                let block = self.resolve_statement(block)?;
                HirStmt::while_loop(condition, block, statement.span)
            }
            StmtKind::ForLoop {
                init,
                condition,
                increment,
                block,
            } => {
                let mut inner_block = match &block.kind {
                    StmtKind::Block(nodes) => nodes
                        .iter()
                        .map(|node| self.resolve_node(node))
                        .collect::<Result<Vec<HirNode>, KaoriError>>()?,
                    _ => unreachable!(),
                };

                let increment = HirNode::from(self.resolve_statement(increment)?);
                inner_block.push(increment);

                let inner_block = HirStmt::block(inner_block, block.span);

                let condition = self.resolve_expression(condition)?;
                let while_loop = HirStmt::while_loop(condition, inner_block, block.span);

                let mut outer_block = Vec::new();
                let init = HirNode::from(self.resolve_declaration(init)?);
                let while_loop = HirNode::from(while_loop);

                outer_block.push(init);
                outer_block.push(while_loop);

                HirStmt::block(outer_block, statement.span)
            }
            StmtKind::Break => HirStmt::break_(statement.span),
            StmtKind::Continue => HirStmt::continue_(statement.span),
            StmtKind::Return(expr) => {
                let expr = match expr {
                    Some(e) => Some(self.resolve_expression(e)?),
                    None => None,
                };

                HirStmt::return_(expr, statement.span)
            }
        };

        Ok(hir_stmt)
    }

    fn resolve_expression(&self, expression: &Expr) -> Result<HirExpr, KaoriError> {
        let hir_expr = match &expression.kind {
            ExprKind::Assign { left, right } => {
                let right = self.resolve_expression(right)?;
                let left = self.resolve_expression(left)?;

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
                    .map(|a| self.resolve_expression(a))
                    .collect::<Result<Vec<HirExpr>, KaoriError>>()?;

                HirExpr::function_call(callee, arguments, expression.span)
            }
            ExprKind::NumberLiteral(value) => HirExpr::number_literal(*value, expression.span),
            ExprKind::BooleanLiteral(value) => HirExpr::boolean_literal(*value, expression.span),
            ExprKind::StringLiteral(value) => {
                HirExpr::string_literal(value.to_owned(), expression.span)
            }
            ExprKind::Identifier(name) => {
                if let Some(symbol) = self.environment.search(name) {
                    let resolution = match symbol.kind {
                        SymbolKind::Variable => Resolution::variable(symbol.id),
                        SymbolKind::Function => Resolution::function(symbol.id),
                        SymbolKind::Struct => Resolution::struct_(symbol.id),
                    };
                } else {
                    return Err(kaori_error!(expression.span, "{} is not declared", name));
                }
                HirExpr::identifier(expression.span)
            }
        };

        Ok(hir_expr)
    }

    fn resolve_type(&self, ty: &Ty) -> HirTy {
        match &ty.kind {
            TyKind::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters.iter().map(|p| self.resolve_type(p)).collect();
                let return_ty = return_ty.as_ref().map(|ty| self.resolve_type(ty));
                HirTy::function(parameters, return_ty, ty.span)
            }
            TyKind::Identifier(name) => HirTy::identifier(name.to_owned(), ty.span),
            TyKind::Bool => HirTy::bool(ty.span),
            TyKind::Number => HirTy::number(ty.span),
        }
    }
}
