use crate::{
    ast::{binary_op::BinaryOpKind, unary_op::UnaryOpKind},
    error::kaori_error::KaoriError,
    hir::{
        decl::{Decl, DeclKind},
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    Number,
    String,
    Boolean,
    Dictionary,
    Unknown,
    Function,
}

fn type_check(declarations: Vec<Decl>) -> Result<(), KaoriError> {
    for declaration in declarations {
        check_declaration(declaration)?;
    }

    Ok(())
}

fn check_declaration(declaration: Decl) -> Result<(), KaoriError> {
    match &declaration.kind {
        DeclKind::Function { body, .. } => {
            for statement in body {
                check_statement(statement)?;
            }
        }
    };

    Ok(())
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
            check_expression(condition)?;
            check_statement(then_branch)?;

            if let Some(else_branch) = else_branch {
                check_statement(else_branch)?;
            };
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

            check_expression(condition)?;
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
                | BinaryOpKind::Modulo
                | BinaryOpKind::Greater
                | BinaryOpKind::GreaterEqual
                | BinaryOpKind::Less
                | BinaryOpKind::LessEqual => match (left, right) {
                    (Type::Number, Type::Number) => Type::Number,
                    (Type::Unknown, Type::Number) => Type::Unknown,
                    (Type::Number, Type::Unknown) => Type::Unknown,
                    (Type::Unknown, Type::Unknown) => Type::Unknown,
                    _ => panic!("error"),
                },
                BinaryOpKind::Equal | BinaryOpKind::NotEqual => {
                    if left == right {
                        Type::Boolean
                    } else {
                        panic!("error")
                    }
                }
            }
        }
        ExprKind::Unary { operator, right } => match &operator.kind {
            UnaryOpKind::Negate => {
                let right = check_expression(right)?;

                match right {
                    Type::Number => Type::Number,
                    Type::Unknown => Type::Unknown,
                    _ => panic!("error"),
                }
            }
        },
        ExprKind::LogicalNot { expr } => {
            let expr = check_expression(expr)?;

            match expr {
                Type::Boolean => Type::Boolean,
                Type::Unknown => Type::Unknown,
                _ => panic!("error"),
            }
        }
        ExprKind::LogicalAnd { left, right } => {
            let left = check_expression(left)?;
            let right = check_expression(right)?;

            match (left, right) {
                (Type::Boolean, Type::Boolean) => Type::Boolean,
                (Type::Unknown, Type::Boolean) => Type::Unknown,
                (Type::Boolean, Type::Unknown) => Type::Unknown,
                (Type::Unknown, Type::Unknown) => Type::Unknown,
                _ => panic!("error"),
            }
        }
        ExprKind::LogicalOr { left, right } => {
            let left = check_expression(left)?;
            let right = check_expression(right)?;

            match (left, right) {
                (Type::Boolean, Type::Boolean) => Type::Boolean,
                (Type::Unknown, Type::Boolean) => Type::Unknown,
                (Type::Boolean, Type::Unknown) => Type::Unknown,
                (Type::Unknown, Type::Unknown) => Type::Unknown,
                _ => panic!("error"),
            }
        }

        ExprKind::Assign { right, .. } => check_expression(right)?,
        ExprKind::DeclareAssign { right, .. } => check_expression(right)?,
        ExprKind::Variable(_) => Type::Unknown,
        ExprKind::Number(_) => Type::Number,
        ExprKind::String(_) => Type::String,
        ExprKind::Boolean(_) => Type::Boolean,
        ExprKind::Function(_) => Type::Function,
        ExprKind::FunctionCall { callee, arguments } => {
            check_expression(callee)?;

            for argument in arguments {
                check_expression(argument)?;
            }

            Type::Unknown
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

            Type::Unknown
        }
        ExprKind::Parameter(_) => Type::Unknown,
    };

    Ok(ty)
}
