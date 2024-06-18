use crate::ast::{self};

pub fn traverse_print(decl: &ast::Declaration) {
    traverse_print_decl(decl, 0);
}

pub fn traverse_print_decl(decl: &ast::Declaration, indent: usize) {
    match decl {
        ast::Declaration::Variable(name, initializer) => {
            println!("{}Variable: {}", " ".repeat(indent), name);
            println!("{}Initializer:", " ".repeat(indent));
            traverse_print_expr(initializer, indent + 1);
        }
        ast::Declaration::Statement(stmt) => {
            println!("{}Statement:", " ".repeat(indent));
            traverse_print_stmt(stmt, indent + 1);
        }
    }
}

pub fn traverse_print_stmt(stmt: &ast::Statement, indent: usize) {
    match stmt {
        ast::Statement::Expression(expr) => {
            println!("{}Expression:", " ".repeat(indent));
            traverse_print_expr(expr, indent + 1);
        }
        ast::Statement::Print(expr) => {
            println!("{}Print:", " ".repeat(indent));
            traverse_print_expr(expr, indent + 1);
        }
        ast::Statement::Block(decls) => {
            println!("{}Block:", " ".repeat(indent));
            for decl in decls {
                traverse_print_decl(decl, indent + 1);
            }
        }
        ast::Statement::If(condition, then_branch) => {
            println!("{}If:", " ".repeat(indent));
            println!("{}Condition:", " ".repeat(indent + 1));
            traverse_print_expr(condition, indent + 2);
            println!("{}Then:", " ".repeat(indent + 1));
            traverse_print_stmt(then_branch, indent + 2);
        }
        ast::Statement::Gamble(expr) => {
            println!("{}Gamble:", " ".repeat(indent));
            traverse_print_expr(expr, indent + 1);
        }
        ast::Statement::Buy(stock, amount) => {
            println!("{}Buy:", " ".repeat(indent));
            println!("{}Stock:", " ".repeat(indent + 1));
            traverse_print_expr(stock, indent + 2);
            println!("{}Amount:", " ".repeat(indent + 1));
            traverse_print_expr(amount, indent + 2);
        }
        ast::Statement::Sell(stock, amount) => {
            println!("{}Sell:", " ".repeat(indent));
            println!("{}Stock:", " ".repeat(indent + 1));
            traverse_print_expr(stock, indent + 2);
            println!("{}Amount:", " ".repeat(indent + 1));
            traverse_print_expr(amount, indent + 2);
        }
        ast::Statement::Loan(amount) => {
            println!("{}Loan:", " ".repeat(indent));
            traverse_print_expr(amount, indent + 1);
        }
        ast::Statement::Pay(amount) => {
            println!("{}Pay:", " ".repeat(indent));
            traverse_print_expr(amount, indent + 1);
        }
    }
}

pub fn traverse_print_expr(expr: &ast::Expression, indent: usize) {
    match expr {
        ast::Expression::Number(value) => {
            println!("{}Number: {}", " ".repeat(indent), value);
        }
        ast::Expression::String(value) => {
            println!("{}String: {}", " ".repeat(indent), value);
        }
        ast::Expression::Boolean(value) => {
            println!("{}Boolean: {}", " ".repeat(indent), value);
        }
        ast::Expression::Void => {
            println!("{}Void", " ".repeat(indent));
        }
        ast::Expression::Variable(name) => {
            println!("{}Variable: {}", " ".repeat(indent), name);
        }
        ast::Expression::ReadonlyVariable(name) => {
            println!("{}ReadonlyVariable: {}", " ".repeat(indent), name);
        }
        ast::Expression::Unary(operator, right) => {
            println!("{}Unary: {:?}", " ".repeat(indent), operator);
            traverse_print_expr(right, indent + 1);
        }
        ast::Expression::Binary(operator, left, right) => {
            println!("{}Binary: {:?}", " ".repeat(indent), operator);
            traverse_print_expr(left, indent + 1);
            traverse_print_expr(right, indent + 1);
        }
        ast::Expression::Logical(operator, left, right) => {
            println!("{}Logical: {:?}", " ".repeat(indent), operator);
            traverse_print_expr(left, indent + 1);
            traverse_print_expr(right, indent + 1);
        }
    }
}
