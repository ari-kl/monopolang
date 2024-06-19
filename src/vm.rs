use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(usize),
    Print,
    GetGlobal(String),
    SetGlobal(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Not,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    JumpIfFalse(usize),
    JumpForwardIfFalse(usize),
    Jump(usize),
    JumpForward(usize),
    ProcedureCall(String),
}

pub struct VM {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    globals: HashMap<String, Value>,
    procedures: HashMap<String, Vec<OpCode>>,
    stack: Vec<Value>,
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            code: Vec::new(),
            constants: Vec::new(),
            globals: HashMap::new(),
            procedures: HashMap::new(),
            stack: Vec::new(),
            ip: 0,
        }
    }

    pub fn write_op(&mut self, op: OpCode) -> usize {
        self.code.push(op);
        self.code.len() - 1
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        for (i, constant) in self.constants.iter().enumerate() {
            if constant == &value {
                return i;
            }
        }

        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn write_procedure(&mut self, name: String, code: Vec<OpCode>) {
        self.procedures.insert(name, code);
    }

    pub fn read_constant(&self, index: usize) -> Value {
        self.constants[index].clone()
    }

    pub fn execute(&mut self) {
        while self.ip < self.code.len() {
            // Print the instruction pointer, instruction, and stack
            // println!(
            //     "ip: {}, instruction: {:?}, stack: {:?}",
            //     self.ip, self.code[self.ip], self.stack
            // );

            // Print the code
            // for (i, instruction) in self.code.iter().enumerate() {
            //     if i == self.ip {
            //         print!("-> ");
            //     } else {
            //         print!("   ");
            //     }

            //     println!("{:?}", instruction);
            // }

            match &self.code[self.ip] {
                OpCode::Constant(index) => {
                    self.stack.push(self.read_constant(*index));
                }
                OpCode::Print => {
                    let value = self.stack.pop().unwrap();
                    println!("{}", value.format());
                }
                OpCode::GetGlobal(name) => {
                    self.stack.push(self.globals.get(name).unwrap().clone());
                }
                OpCode::SetGlobal(name) => {
                    let value = self.stack.pop().unwrap();
                    self.globals.insert(name.to_string(), value);
                }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (&a, &b) {
                        self.stack.push(Value::Number(a + b));
                    } else if let (Value::String(a), Value::String(b)) = (&a, &b) {
                        self.stack.push(Value::String(format!("{}{}", a, b)));
                    } else {
                        panic!("Operands must be two numbers or two strings");
                    }
                }
                OpCode::Subtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a - b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::Multiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a * b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::Divide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a / b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::Negate => {
                    let a = self.stack.pop().unwrap();

                    if let Value::Number(a) = a {
                        self.stack.push(Value::Number(-a));
                    } else {
                        panic!("Operand must be a number");
                    }
                }
                OpCode::Not => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_boolean(!a.is_truthy()));
                }
                OpCode::Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_boolean(a == b));
                }
                OpCode::NotEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_boolean(a != b));
                }
                OpCode::Greater => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::from_boolean(a > b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::GreaterEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::from_boolean(a >= b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::Less => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::from_boolean(a < b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::LessEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::from_boolean(a <= b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::And => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack
                        .push(Value::from_boolean(a.is_truthy() && b.is_truthy()));
                }
                OpCode::Or => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack
                        .push(Value::from_boolean(a.is_truthy() || b.is_truthy()));
                }
                OpCode::JumpIfFalse(offset) => {
                    let condition = self.stack.pop().unwrap();

                    if !condition.is_truthy() {
                        self.ip = *offset;
                        continue;
                    }
                }
                OpCode::Jump(offset) => {
                    self.ip = *offset;
                    continue;
                }
                OpCode::JumpForwardIfFalse(offset) => {
                    let condition = self.stack.pop().unwrap();

                    if !condition.is_truthy() {
                        self.ip += *offset;
                        continue;
                    }
                }
                OpCode::JumpForward(offset) => {
                    self.ip += *offset;
                    continue;
                }
                OpCode::ProcedureCall(name) => {
                    // Replace the procedure call with the procedure's code
                    let procedure = self.procedures.get(name).unwrap();

                    self.code
                        .splice(self.ip + 1..self.ip + 1, procedure.iter().cloned());
                }
            }

            self.ip += 1;
        }
    }
}
