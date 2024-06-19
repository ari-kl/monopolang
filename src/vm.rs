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
    JumpForwardIfFalse(isize),
    Jump(usize),
    JumpForward(isize),
    ProcedureCall(String),
    Pop,

    // Economy System
    Cost(f64),
    Gamble,
}

#[derive(Debug, Clone)]
pub struct VM {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    globals: HashMap<String, Value>,
    pub procedures: HashMap<String, Vec<OpCode>>,
    stack: Vec<Value>,
    ip: usize,

    // Economy System
    balance: f64,
    debt: f64,
    stocks: HashMap<String, f64>,
    won_last_gamble: bool,
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
            balance: 100.0,
            debt: 0.0,
            stocks: HashMap::new(),
            won_last_gamble: false,
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
            //     "ip: {}, instruction: {:?}, balance: {:?}, stack: {:?}",
            //     self.ip, self.code[self.ip], self.balance, self.stack
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
                OpCode::GetGlobal(name) => match name as &str {
                    "@balance" => {
                        self.stack.push(Value::Number(self.balance));
                    }
                    "@debt" => {
                        self.stack.push(Value::Number(self.debt));
                    }
                    "@won" => {
                        self.stack.push(Value::Boolean(self.won_last_gamble));
                    }
                    _ => {
                        let value = self.globals.get(name);

                        if let Some(value) = value {
                            self.stack.push(value.clone());
                        } else {
                            panic!("Accessing undefined variable '{}'", name);
                        }
                    }
                },
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
                    } else if let (Value::String(a), Value::Number(b)) = (&a, &b) {
                        self.stack.push(Value::String(format!("{}{}", a, b)));
                    } else if let (Value::Number(a), Value::String(b)) = (&a, &b) {
                        self.stack.push(Value::String(format!("{}{}", a, b)));
                    } else {
                        if a.is_truthy() {
                            self.stack.push(a);
                        } else {
                            self.stack.push(b);
                        }
                        panic!("Operands must be numbers or strings");
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
                        self.ip = (self.ip as isize + *offset) as usize;
                        continue;
                    }
                }
                OpCode::JumpForward(offset) => {
                    self.ip = (self.ip as isize + *offset) as usize;
                    continue;
                }
                OpCode::ProcedureCall(name) => {
                    // Replace the procedure call with the procedure's code
                    let mut procedure = self.procedures.get(name).unwrap().clone();

                    // Modify all the jumps in the procedure to be relative to the current IP
                    for op in procedure.iter_mut() {
                        match op {
                            OpCode::Jump(offset) => {
                                *offset = (*offset as isize + self.ip as isize) as usize;
                            }
                            OpCode::JumpIfFalse(offset) => {
                                *offset = (*offset as isize + self.ip as isize) as usize;
                            }
                            _ => {}
                        }
                    }

                    // Modify all future jumps (jumps pointing to IPs past the call) in the code to be +(procedure length - 1)
                    // This is because the procedure call is replaced with the procedure's code and pushes the IP forward
                    for op in self.code.iter_mut() {
                        match op {
                            OpCode::Jump(offset) => {
                                if *offset > self.ip {
                                    *offset =
                                        (*offset as isize + procedure.len() as isize - 1) as usize;
                                }
                            }
                            OpCode::JumpIfFalse(offset) => {
                                if *offset > self.ip {
                                    *offset =
                                        (*offset as isize + procedure.len() as isize - 1) as usize;
                                }
                            }
                            _ => {}
                        }
                    }

                    self.code
                        .splice(self.ip..self.ip + 1, procedure.iter().cloned());

                    self.ip -= 1;
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Cost(amount) => {
                    self.balance -= amount;

                    if self.balance <= 0.0 {
                        panic!("Insufficient funds!");
                    }
                }
                OpCode::Gamble => {
                    let amount = self.stack.pop().unwrap();

                    if let Value::Number(amount) = amount {
                        if amount > self.balance {
                            panic!("Insufficient funds to gamble!");
                        }

                        let random = rand::random::<f64>();

                        if random < 0.5 {
                            self.balance += amount;
                            self.won_last_gamble = true;
                        } else {
                            self.balance -= amount;
                            self.won_last_gamble = false;
                        }
                    } else {
                        panic!("Operand must be a number");
                    }
                }
            }

            self.ip += 1;
        }
    }
}
