use core::fmt;
use std::num::IntErrorKind;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    W,
    X,
    Y,
    Z
}

impl Register {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Register::W => 'W',
            Register::X => 'X',
            Register::Y => 'Y',
            Register::Z => 'Z',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Registers {
    pub w: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Registers{
    pub fn get(&mut self, register: &Register) -> i32 {
        *self.get_field(register)
    }

    pub fn set(&mut self, register: &Register, value: i32) {
        *self.get_field(register) = value;
    }

    fn get_field(&mut self, register: &Register) -> &mut i32 {
        match register {
            Register::W => &mut self.w,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Literal(i32),
    Register(Register),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Operand::Literal(value) => value.to_string(),
            Operand::Register(register) => register.to_string(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Input {register: Register},
    Add {register: Register, operand: Operand},
    Multiply {register: Register, operand: Operand},
    Divide {register: Register, operand: Operand},
    Modulo {register: Register, operand: Operand},
    Equals {register: Register, operand: Operand},
    Print {register: Register}
}

pub fn interpret(operators: &[Operator], input: &[i32], registers: &mut Registers) {
    let mut input = input.iter().rev().collect_vec();

    for operator in operators {
        match operator {
            Operator::Input {register} => {
                let value = input.pop();

                if let Some(value) = value {
                    registers.set(&register, *value);
                } else {
                    panic!("No more input available");
                }
            },
            Operator::Add {register, operand} => {
                let result = registers.get(&register) + evaluate(&operand, registers);
                registers.set(&register, result);
            },
            Operator::Multiply {register, operand} => {
                let result = registers.get(&register) * evaluate(&operand, registers);
                registers.set(&register, result);
            },
            Operator::Divide {register, operand} => {
                let result = registers.get(&register) / evaluate(&operand, registers);
                registers.set(&register, result);
            },
            Operator::Modulo {register, operand} => {
                let result = registers.get(&register) % evaluate(&operand, registers);
                registers.set(&register, result);
            },
            Operator::Equals {register, operand} => {
                let equal = registers.get(&register) == evaluate(&operand, registers);
                let result = match equal {
                    true => 1,
                    false => 0,
                };
                registers.set(&register, result);
            },
            Operator::Print {register} => {
                println!("{} = {}", register.to_string(), registers.get(register));
            },
            _ => panic!()
        }
    }
}

fn evaluate(operand: &Operand, registers: &mut Registers) -> i32 {
    match operand {
        Operand::Literal(value) => *value,
        Operand::Register(register) => registers.get(register)
    }
}
