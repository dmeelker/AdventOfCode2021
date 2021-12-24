use core::panic;
use std::{fs, vec};
use interpreter::{Operator, Operand, Register, interpret, Registers};
use itertools::Itertools;

mod interpreter;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let variables = get_values_from_program(&input.lines().collect_vec());
    eprintln!("variables = {:?}", variables);

    let part1 = solve_part1(&variables);
    let part2 = solve_part2(&variables);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

// fn test() {
//     let add1 =  vec![10, 11, 14, 13, -6, -14, 14, 13, -8, -15, 10, -11, -13, -4];
//     let add2 =  vec![1,  9,  12, 6,   9,  15,  7, 12, 15,  3,   6,   2,  10, 12];
//     let div =   vec![1,  1,  1,  1,  26,  26,  1,  1, 26, 26,   1,  26,  26, 26];
//     let input = vec![9,  9,  9,  9,   9,   9,  9,  9,  9,  9,   9,   9,  9,  9];
//     let input = vec![9,  9,  9,  9,   9,   7,  9,  5,  9,  1,   9,   4,  5,  6];
//     let input = vec![4,  5,  3,  1,   1,   1,  9,  1,  5,  1,   6,   1,  1,  1];
//     let mut stack = vec![];

//     for i in 0..add1.len() {
//         eprintln!("stack {} = {:?}", i, stack);
//         let w = input[i];

//         if *stack.last().unwrap_or(&0) == w - add1[i] {
//             if div[i] == 26 {
//                 stack.pop();
//             }
//         } else {
//             if div[i] == 26 {
//                 stack.pop();
//             }

//             stack.push(w + add2[i]);
//         }
//     }
//     eprintln!("stack = {:?}", stack);


//     let z = stack.iter().fold(0, |acc, current| (acc * 26) + current);

//     eprintln!("z = {:?}", z);
// }

fn solve_part1(variables: &ProgramVariables) -> String {
    // let add1 =      vec![10, 11, 14, 13, -6, -14, 14, 13, -8, -15, 10, -11, -13, -4];
    // let add2 =      vec![1,  9,  12, 6,   9,  15,  7, 12, 15,  3,   6,   2,  10, 12];
    // let div =       vec![1,  1,  1,  1,  26,  26,  1,  1, 26, 26,   1,  26,  26, 26];
    let mut input = vec![9,  9,  9,  9,   9,   9,  9,  9,  9,  9,   9,   9,   9,  9];

    let mut stack = vec![];

    for i in 0..variables.add1.len() {
        if variables.div[i] == 26 {
            let push_index = stack.pop().unwrap();
            let sum_push = variables.add2[push_index] + input[push_index];
            let sum_pop = input[i] - variables.add1[i];

            if sum_pop < sum_push {
                input[push_index] = 1 + (sum_push - sum_pop);
            } else {
                input[i] = sum_push + variables.add1[i];
            }
        } else {
            stack.push(i);
        }
    }

    eprintln!("input = {:?}", input);

    input.iter().map(|v| v.to_string()).collect()
}

fn solve_part2(variables: &ProgramVariables) -> String{
    // let add1 =  vec![10, 11, 14, 13, -6, -14, 14, 13, -8, -15, 10, -11, -13, -4];
    // let add2 =  vec![1,  9,  12, 6,   9,  15,  7, 12, 15,  3,   6,   2,  10, 12];
    // let div =   vec![1,  1,  1,  1,  26,  26,  1,  1, 26, 26,   1,  26,  26, 26];
    let mut input = vec![1,  1,  1,  1,   1,  1,  1,  1,  1, 1,   1,  1, 1,  1];

    let mut stack = vec![];

    for i in 0..variables.add1.len() {
        if variables.div[i] == 26 {
            let push_index = stack.pop().unwrap();
            let sum_push = variables.add2[push_index] + input[push_index];
            let sum_pop = input[i] - variables.add1[i];

            if sum_pop < sum_push {
                input[i] = sum_push + variables.add1[i]; 
            } else {
                input[push_index] = 1 + (sum_pop - sum_push);
            }
        } else {
            stack.push(i);
        }
    }

    eprintln!("input = {:?}", input);

    input.iter().map(|v| v.to_string()).collect()
}

// fn parse_input(input: &str) -> Vec<Operator> {
//     input.lines().map(parse_operator).collect()
// }

// fn parse_operator(input: &str) -> Operator {
//     let parts = input.split_whitespace().collect_vec();

//     match parts[0] {
//         "inp" => Operator::Input {register: parse_register(parts[1])},
//         "add" => Operator::Add {register: parse_register(parts[1]), operand: parse_operand(parts[2])},
//         "mul" => Operator::Multiply {register: parse_register(parts[1]), operand: parse_operand(parts[2])},
//         "div" => Operator::Divide {register: parse_register(parts[1]), operand: parse_operand(parts[2])},
//         "mod" => Operator::Modulo {register: parse_register(parts[1]), operand: parse_operand(parts[2])},
//         "eql" => Operator::Equals {register: parse_register(parts[1]), operand: parse_operand(parts[2])},
//         "print" => Operator::Print {register: parse_register(parts[1])},
//         _ => panic!("Unknown operator: {}", parts[0])
//     }
// }

// fn parse_register(input: &str) -> Register {
//     match input {
//         "w" => Register::W,
//         "x" => Register::X,
//         "y" => Register::Y,
//         "z" => Register::Z,
//         _ => panic!()
//     }
// }

// fn parse_operand(input: &str) -> Operand {
//     let value = input.parse();

//     match value {
//         Ok(value) => Operand::Literal(value),
//         Error => Operand::Register(parse_register(input))
//     }
// }

#[derive(Debug)]
struct ProgramVariables {
    add1: Vec<i32>,
    add2: Vec<i32>,
    div: Vec<i32>,
}

fn get_values_from_program(lines: &Vec<&str>) -> ProgramVariables {
    ProgramVariables {
        div: (0..14).map(|i| get_last_operand(&lines[(i * 18) + 4])).collect(),
        add1: (0..14).map(|i| get_last_operand(&lines[(i * 18) + 5])).collect(),
        add2: (0..14).map(|i| get_last_operand(&lines[(i * 18) + 15])).collect(),
    }
}

fn get_last_operand(input: &str) -> i32 {
    let parts = input.split_whitespace().collect_vec();

    parts.last().unwrap().parse().unwrap()
}

// fn part1(operators: &[Operator]) -> String {
//     let mut add1 = vec![];
//     let mut add2 = vec![];
//     let mut div = vec![];

//     for i in 0..14 {
//         let d = &operators[(i * 18) + 4];
//         let v1 = &operators[(i * 18) + 5];
//         let v2 = &operators[(i * 18) + 15];

//         div.push(match d {
//             Operator::Divide {register, operand} => 
//                 match operand {
//                     Operand::Literal(value) => value,
//                     _ => panic!()
//                 },
//             _ => panic!()
//         });

//         add1.push(match v1 {
//             Operator::Add {register, operand} => 
//                 match operand {
//                     Operand::Literal(value) => value,
//                     _ => panic!()
//                 },
//             _ => panic!()
//         });

//         add2.push(match v2 {
//             Operator::Add {register, operand} => 
//                 match operand {
//                     Operand::Literal(value) => value,
//                     _ => panic!()
//                 },
//             _ => panic!()
//         });

//         //eprintln!("add1 = {:?} add2 = {:?}", add1, add2);
//     }

//     eprintln!("div = {:?}", div);
//     // eprintln!("add1 = {:?}", add1);
//     // eprintln!("add2 = {:?}", add2);
//     // let add1 = vec![];
//     // let add2 = vec![];

// //     X = ((Z % 26) + 10) != W

// // Y = (25 * X) + 1
// // Z = Z * Y

// // Y = (W + 1) * X
// // Z = Z + Y

//     verify_model_number(&[9; 14], operators);

// // for i in 1..=9 {
// //     let input: Vec<i32> = vec![i];

// //     let mut registers = Registers {
// //         w: 0,
// //         x: 0,
// //         y: 0,
// //         z: 10,   
// //     };

// //     interpret(operators, &input, &mut registers);

// //     println!("{} => {}", i, registers.z);
// // }

//     //format_program(operators);

//     // let mut model_number = [9; 14]; //[9,9,9,9,9,9,9,9,9,9,9,9,9,9];
//     // let mut count = 0;
//     // loop {

//     //     if count % 100000 == 0 {
//     //         eprintln!("model_number = {:?}", model_number);
//     //     }
//     //     if verify_model_number(&model_number, operators) {
//     //         panic!("Found one! {:?}", model_number);
//     //         //return model_number.iter().copied().collect();
//     //     }

//     //     count +=1;
//     //     if !next_model_number(&mut &mut model_number) {
//     //         break;
//     //     }
//     // }

//     String::from("")
// }

// fn next_model_number(input: &mut [i32;14]) -> bool {
//     subtract(input, input.len() - 1)
// }

// fn subtract(input: &mut [i32;14], index: usize) -> bool {
//     if input[index] > 1 {
//         input[index] -= 1;
//         return true;
//     } else {
//         input[index] = 9;
//         if index > 0 {
//             subtract(input, index-1);
//             return true;
//         } else {
//             return false;
//         }
//     }
// }

// fn part2(values: &[Operator]) -> i32 {
//     2
// }

// fn format_program(program: &[Operator]) {
//    for operator in program.iter() {
//        match *operator {
//            Operator::Input{register} => println!("{} = ReadInput", register.to_string()),
//            Operator::Add{register, operand} => println!("{} = {} + {}", register.to_string(), register.to_string(), operand),
//            Operator::Multiply{register, operand} => println!("{} = {} * {}", register.to_string(), register.to_string(), operand),
//            Operator::Divide{register, operand} => println!("{} = {} / {}", register.to_string(), register.to_string(), operand),
//            Operator::Modulo{register, operand} => println!("{} = {} % {}", register.to_string(), register.to_string(), operand),
//            Operator::Equals{register, operand} => println!("{} = {} == {}", register.to_string(), register.to_string(), operand),
//            Operator::Print{register} => (),
//        }
//    }
// }

// fn verify_model_number(input: &[i32; 14], program: &[Operator]) -> bool {
//     let input: Vec<i32> = input.iter().copied().collect_vec();

//     let mut registers = Registers {
//         w: 0,
//         x: 0,
//         y: 0,
//         z: 0,   
//     };

//     interpret(program, &input, &mut registers);
//     eprintln!("registers = {:?}", registers);
//     registers.z == 0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_should_work() {
        let input = vec![String::from("123")];
        let result = part1(&input);

        assert_eq!(1, result);
    }

    #[test]
    fn part2_should_work() {
        let input = vec![String::from("123")];
        let result = part2(&input);

        assert_eq!(2, result);
    }
}