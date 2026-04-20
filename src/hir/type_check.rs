use crate::{
    ast::{binary_op::BinaryOpKind, unary_op::UnaryOpKind},
    error::kaori_error::KaoriError,
    hir::{
        decl::{Decl, DeclKind},
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
    },
    kaori_error,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    Number,
    String,
    Boolean,
    Dictionary,
    Function,
    Any,
}

pub fn run_type_check(declarations: &[Decl]) -> Result<(), KaoriError> {
    for declaration in declarations {
        check_declaration(declaration)?;
    }

    Ok(())
}

fn check_declaration(declaration: &Decl) -> Result<(), KaoriError> {
    match &declaration.kind {
        DeclKind::Function { body, .. } => {
            for statement in body {
                check_statement(statement)?;
            }
        }
    };

    Ok(())
}

fn check_expression(expression: &Expr) -> Result<Type, KaoriError> {
    let ty = match &expression.kind {
        ExprKind::Binary {
            left,
            right,
            operator,
        } => {
            let left = check_expression(left)?;
            let right = check_expression(right)?;

            match &operator.kind {
                BinaryOpKind::Add
                | BinaryOpKind::Subtract
                | BinaryOpKind::Multiply
                | BinaryOpKind::Divide
                | BinaryOpKind::Modulo => match (left, right) {
                    (Type::Number, Type::Number)
                    | (Type::Any, Type::Number)
                    | (Type::Number, Type::Any)
                    | (Type::Any, Type::Any) => Type::Number,
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "cannot apply arithmetic to {:?} and {:?}",
                            left,
                            right
                        ));
                    }
                },
                BinaryOpKind::Greater
                | BinaryOpKind::GreaterEqual
                | BinaryOpKind::Less
                | BinaryOpKind::LessEqual => match (left, right) {
                    (Type::Number, Type::Number)
                    | (Type::Any, Type::Number)
                    | (Type::Number, Type::Any)
                    | (Type::Any, Type::Any) => Type::Boolean,
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "cannot compare {:?} and {:?}",
                            left,
                            right
                        ));
                    }
                },
                BinaryOpKind::Equal | BinaryOpKind::NotEqual => match (left, right) {
                    (Type::Any, _) | (_, Type::Any) => Type::Boolean,
                    (a, b) if a == b => Type::Boolean,
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "cannot compare {:?} and {:?}",
                            left,
                            right
                        ));
                    }
                },
            }
        }
        ExprKind::Unary { operator, right } => match &operator.kind {
            UnaryOpKind::Negate => match check_expression(right)? {
                Type::Number | Type::Any => Type::Number,
                t => return Err(kaori_error!(expression.span, "cannot negate {:?}", t)),
            },
        },
        ExprKind::LogicalNot { expr } => match check_expression(expr)? {
            Type::Boolean | Type::Any => Type::Boolean,
            t => return Err(kaori_error!(expression.span, "cannot apply ! to {:?}", t)),
        },
        ExprKind::LogicalAnd { left, right } | ExprKind::LogicalOr { left, right } => {
            let left = check_expression(left)?;
            let right = check_expression(right)?;

            match (left, right) {
                (Type::Boolean, Type::Boolean)
                | (Type::Any, Type::Boolean)
                | (Type::Boolean, Type::Any)
                | (Type::Any, Type::Any) => Type::Boolean,
                _ => {
                    return Err(kaori_error!(
                        expression.span,
                        "logical operators require booleans"
                    ));
                }
            }
        }
        ExprKind::Assign { right, .. } => check_expression(right)?,
        ExprKind::DeclareAssign { right, .. } => check_expression(right)?,
        ExprKind::Variable(_) => Type::Any,
        ExprKind::Number(_) => Type::Number,
        ExprKind::String(_) => Type::String,
        ExprKind::Boolean(_) => Type::Boolean,
        ExprKind::Function(_) => Type::Function,
        ExprKind::FunctionCall { callee, arguments } => {
            check_expression(callee)?;
            for argument in arguments {
                check_expression(argument)?;
            }
            Type::Any
        }
        ExprKind::DictLiteral { fields } => {
            for (key, value) in fields {
                check_expression(key)?;
                check_expression(value)?;
            }
            Type::Dictionary
        }
        ExprKind::MemberAccess { object, property } => {
            check_expression(object)?;
            check_expression(property)?;
            Type::Any
        }
        ExprKind::Parameter(_) => Type::Any,
    };

    Ok(ty)
}

fn check_statement(statement: &Stmt) -> Result<(), KaoriError> {
    match &statement.kind {
        StmtKind::Expression(expression) => {
            check_expression(expression)?;
        }
        StmtKind::Print(expression) => {
            check_expression(expression)?;
        }
        StmtKind::Block(statements) | StmtKind::UncheckedBlock(statements) => {
            for statement in statements {
                check_statement(statement)?;
            }
        }
        StmtKind::Branch {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            let cond_type = check_expression(condition)?;
            if !matches!(cond_type, Type::Boolean | Type::Any) {
                return Err(kaori_error!(
                    condition.span,
                    "condition must be a boolean, got {:?}",
                    cond_type
                ));
            }
            check_statement(then_branch)?;
            if let Some(else_branch) = else_branch {
                check_statement(else_branch)?;
            }
        }
        StmtKind::Loop {
            init,
            condition,
            block,
            increment,
        } => {
            if let Some(init) = init {
                check_expression(init)?;
            }
            let cond_type = check_expression(condition)?;
            if !matches!(cond_type, Type::Boolean | Type::Any) {
                return Err(kaori_error!(
                    condition.span,
                    "loop condition must be a boolean, got {:?}",
                    cond_type
                ));
            }
            check_statement(block)?;
            if let Some(increment) = increment {
                check_statement(increment)?;
            }
        }
        StmtKind::Break | StmtKind::Continue => (),
        StmtKind::Return(expression) => {
            if let Some(expression) = expression {
                check_expression(expression)?;
            }
        }
    }
    Ok(())
}
