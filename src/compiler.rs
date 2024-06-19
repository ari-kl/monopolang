use crate::{
    ast::*,
    value::Value,
    vm::{OpCode, VM},
};

#[derive(Debug, Clone)]
pub struct Compiler {
    ast: Vec<Declaration>,
    pub vm: VM,
    pub parent: Option<Box<Compiler>>,
}

impl Compiler {
    pub fn new(ast: Vec<Declaration>, vm: VM) -> Self {
        Compiler {
            ast,
            vm,
            parent: None,
        }
    }

    pub fn compile(&mut self) {
        for decl in self.ast.clone() {
            self.declaration(decl);
        }
    }

    fn declaration(&mut self, decl: Declaration) {
        match decl {
            Declaration::Statement(stmt) => self.statement(stmt),
            Declaration::Procedure(name, code) => self.procedure(name, code),
        }
    }

    fn procedure(&mut self, name: String, code: Vec<Statement>) {
        // Create a new compiler for the procedure, with the current compiler as the parent
        let mut compiler = Compiler {
            ast: code
                .into_iter()
                .map(|stmt| Declaration::Statement(stmt))
                .collect(),
            vm: VM::new(),
            parent: Some(Box::new(self.clone())),
        };

        compiler.compile();

        println!("Compiled procedure: {}", &name);
        println!("{:?}", compiler.vm.code);

        self.vm.constants = compiler.parent.unwrap().vm.constants.clone();
        self.vm.procedures.insert(name, compiler.vm.code);
    }

    fn statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::Print(expr) => {
                self.expression(expr);
                self.vm.write_op(OpCode::Print);
            }
            Statement::Expression(expr) => {
                self.expression(expr);
                self.vm.write_op(OpCode::Pop);
            }
            Statement::VariableAssignment(name, expr) => {
                self.expression(expr);
                self.vm.write_op(OpCode::SetGlobal(name));
            }
            Statement::Block(stmts) => {
                for stmt in stmts {
                    self.statement(stmt);
                }
            }
            Statement::ProcedureCall(name) => {
                self.vm.write_op(OpCode::ProcedureCall(name));
            }
            Statement::If(cond, then_branch, else_branch) => {
                self.expression(cond);
                let jump_forward = self.vm.write_op(OpCode::JumpIfFalse(0));
                self.statement(*then_branch);
                // If there is an else branch, we need to jump over it
                // Patch jump_forward to jump to the end of the then branch
                let current_idx = self.vm.code.len();
                self.vm.code[jump_forward] = OpCode::JumpIfFalse(current_idx + 1);
                // If there is an else branch, we need to jump over it
                if let Some(else_branch) = else_branch {
                    let skip_else = self.vm.write_op(OpCode::Jump(0));
                    self.statement(*else_branch);
                    let current_idx = self.vm.code.len();
                    self.vm.code[skip_else] = OpCode::Jump(current_idx);
                }
            }
            Statement::While(cond, body) => {
                let loop_start = self.vm.code.len();
                self.expression(cond);
                let jump_forward = self.vm.write_op(OpCode::JumpIfFalse(0));
                self.statement(*body);
                self.vm.write_op(OpCode::Jump(loop_start));
                let current_idx = self.vm.code.len();
                self.vm.code[jump_forward] = OpCode::JumpIfFalse(current_idx);
            }
            Statement::Range(variable, start, end, step, body) => {
                // Convert the range to a while loop
                self.expression(start);
                self.vm.write_op(OpCode::SetGlobal(variable.clone()));

                self.statement(Statement::While(
                    Expression::Binary(
                        BinaryOperator::Less,
                        Box::new(Expression::Variable(variable.clone())),
                        Box::new(end),
                    ),
                    Box::new(Statement::Block(vec![
                        *body.clone(),
                        Statement::VariableAssignment(
                            variable.clone(),
                            Expression::Binary(
                                BinaryOperator::Add,
                                Box::new(Expression::Variable(variable)),
                                Box::new(step),
                            ),
                        ),
                    ])),
                ));
            }
            // TODO: Economic statements
            Statement::Buy(_, _) => {}
            Statement::Sell(_, _) => {}
            Statement::Loan(_) => {}
            Statement::Pay(_) => {}
            Statement::Gamble(_) => {}
        }
    }

    fn expression(&mut self, expr: Expression) {
        match expr {
            Expression::Number(n) => {
                let idx = self.write_constant(Value::from_number(n));
                self.vm.write_op(OpCode::Constant(idx));
            }
            Expression::String(s) => {
                let idx = self.write_constant(Value::from_string(s.as_str()));
                self.vm.write_op(OpCode::Constant(idx));
            }
            Expression::Boolean(b) => {
                let idx = self.write_constant(Value::from_boolean(b));
                self.vm.write_op(OpCode::Constant(idx));
            }
            Expression::Variable(name) => {
                self.vm.write_op(OpCode::GetGlobal(name));
            }
            Expression::ReadonlyVariable(name) => {
                self.vm.write_op(OpCode::GetGlobal(name));
            }
            Expression::Unary(op, expr) => {
                self.expression(*expr);
                match op {
                    UnaryOperator::Negate => self.vm.write_op(OpCode::Negate),
                    UnaryOperator::Not => self.vm.write_op(OpCode::Not),
                };
            }
            Expression::Binary(op, left, right) => {
                self.expression(*left);
                self.expression(*right);
                match op {
                    BinaryOperator::Add => self.vm.write_op(OpCode::Add),
                    BinaryOperator::Subtract => self.vm.write_op(OpCode::Subtract),
                    BinaryOperator::Multiply => self.vm.write_op(OpCode::Multiply),
                    BinaryOperator::Divide => self.vm.write_op(OpCode::Divide),
                    BinaryOperator::Equal => self.vm.write_op(OpCode::Equal),
                    BinaryOperator::NotEqual => self.vm.write_op(OpCode::NotEqual),
                    BinaryOperator::Less => self.vm.write_op(OpCode::Less),
                    BinaryOperator::LessEqual => self.vm.write_op(OpCode::LessEqual),
                    BinaryOperator::Greater => self.vm.write_op(OpCode::Greater),
                    BinaryOperator::GreaterEqual => self.vm.write_op(OpCode::GreaterEqual),
                };
            }
            Expression::Logical(op, left, right) => {
                self.expression(*left);
                self.expression(*right);
                match op {
                    LogicalOperator::And => self.vm.write_op(OpCode::And),
                    LogicalOperator::Or => self.vm.write_op(OpCode::Or),
                };
            }
            Expression::Void => {} // TODO: Implement void expression
        }
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        // Write to parent if exists
        if let Some(parent) = &mut self.parent {
            return parent.write_constant(value);
        } else {
            self.vm.write_constant(value)
        }
    }
}
