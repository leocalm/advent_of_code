use common::base_day::BaseDay;
use common::file::get_input_path;
use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

pub struct Day17 {
    day_number: u32,
    file_path: PathBuf,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum OperandType {
    Literal,
    Combo,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Operator {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct State {
    instructions_index: usize,
    registers: Registers,
}

impl Operator {
    fn perform_operation(&self, instructions: &Vec<u32>, state: State) -> (State, Option<u32>) {
        let mut value = instructions[state.instructions_index + 1];
        if get_type_of_operand(*self) == OperandType::Combo {
            value = get_combo_value(value, state.registers);
        }
        match self {
            Self::Adv => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
                // The denominator is found by raising 2 to the power of the instruction's combo operand. (
                // So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
                // The result of the division operation is truncated to an integer and then written to the A register.
                let numerator = state.registers.a;
                let denominator = 2u32.pow(value);
                (
                    State {
                        instructions_index: state.instructions_index + 2,
                        registers: Registers {
                            a: numerator / denominator,
                            b: state.registers.b,
                            c: state.registers.c,
                        },
                    },
                    None,
                )
            }
            Self::Bxl => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                let operand_1 = state.registers.b;
                let operand_2 = instructions[state.instructions_index + 1];
                (
                    State {
                        instructions_index: state.instructions_index + 2,
                        registers: Registers {
                            a: state.registers.a,
                            b: operand_1 ^ operand_2,
                            c: state.registers.c,
                        },
                    },
                    None,
                )
            }
            Self::Bst => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                let operand = value;
                (
                    State {
                        instructions_index: state.instructions_index + 2,
                        registers: Registers {
                            a: state.registers.a,
                            b: operand % 8,
                            c: state.registers.c,
                        },
                    },
                    None,
                )
            }
            Self::Jnz => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0.
                // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
                // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if state.registers.a == 0 {
                    (
                        State {
                            instructions_index: state.instructions_index + 2,
                            registers: state.registers,
                        },
                        None,
                    )
                } else {
                    let value = instructions[state.instructions_index + 1];
                    (
                        State {
                            instructions_index: value as usize,
                            registers: state.registers,
                        },
                        None,
                    )
                }
            }
            Self::Bxc => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
                // (For legacy reasons, this instruction reads an operand but ignores it.)
                let operand_1 = state.registers.b;
                let operand_2 = state.registers.c;
                (
                    State {
                        instructions_index: state.instructions_index + 2,
                        registers: Registers {
                            a: state.registers.a,
                            b: operand_1 ^ operand_2,
                            c: state.registers.c,
                        },
                    },
                    None,
                )
            }
            Self::Out => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
                // (If a program outputs multiple values, they are separated by commas.)
                (
                    State {
                        instructions_index: state.instructions_index + 2,
                        registers: state.registers,
                    },
                    Some(value % 8),
                )
            }
            Self::Bdv => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
                // (The numerator is still read from the A register.)
                let numerator = state.registers.a;
                let denominator = 2u32.pow(value);
                (
                    State {
                        instructions_index: state.instructions_index + 2,
                        registers: Registers {
                            a: state.registers.a,
                            b: numerator / denominator,
                            c: state.registers.c,
                        },
                    },
                    None,
                )
            }
            Self::Cdv => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
                // (The numerator is still read from the A register.)
                let numerator = state.registers.a;
                let denominator = 2u32.pow(value);
                (
                    State {
                        instructions_index: state.instructions_index + 2,
                        registers: Registers {
                            a: state.registers.a,
                            b: state.registers.b,
                            c: numerator / denominator,
                        },
                    },
                    None,
                )
            }
        }
    }
}

fn get_type_of_operand(operator: Operator) -> OperandType {
    match operator {
        Operator::Adv => OperandType::Combo,
        Operator::Bxl => OperandType::Literal,
        Operator::Bst => OperandType::Combo,
        Operator::Jnz => OperandType::Literal,
        Operator::Bxc => OperandType::Literal,
        Operator::Out => OperandType::Combo,
        Operator::Bdv => OperandType::Combo,
        Operator::Cdv => OperandType::Combo,
    }
}

fn get_operator_by_code(code: u32) -> Operator {
    match code {
        0 => Operator::Adv,
        1 => Operator::Bxl,
        2 => Operator::Bst,
        3 => Operator::Jnz,
        4 => Operator::Bxc,
        5 => Operator::Out,
        6 => Operator::Bdv,
        7 => Operator::Cdv,
        _ => unreachable!(),
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

fn get_combo_value(value: u32, registers: Registers) -> u32 {
    if value <= 3 {
        value
    } else if value == 4 {
        registers.a
    } else if value == 5 {
        registers.b
    } else if value == 6 {
        registers.c
    } else {
        unreachable!()
    }
}

/// Considering `F(A) = ((((A % 8) ^ 3) ^ 5) ^ (A / (2.pow(((A % 8) ^ 3))))) % 8`
/// ```
/// r = A % 8
/// s = r ^ 3
/// t = s ^ 5
/// b = A / (2u32.pow(s))
/// ```
/// So, F becomes `F(A) = (t ^ b) % 8`
fn f(a: u64) -> u64 {
    let r = a % 8;
    let s = r ^ 3;
    let t = s ^ 5;
    let b = a / (2u64.pow(s as u32));

    (t ^ b) % 8
}

/// Considering that we need to produce the same values as the input = [[2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0]]
///
/// To get the solutions, we need to find a value of A that satisfies the following:
/// ```
/// for i in input {
///   f(A) == i
///   A = A / 8
/// }
/// ```
fn solve_part_2(expected_output: Vec<u64>) -> u64 {
    // Start with small values for A
    let mut valid: HashSet<u64> = (0..200)
        .filter(|a| f(*a) == *expected_output.last().unwrap())
        .collect();

    for index in (0..expected_output.len() - 1).rev() {
        let target = expected_output[index];
        let mut tmp_valid = HashSet::new();
        for _a in valid.iter() {
            for r in 0..8 {
                let a = _a * 8 + r;
                if f(a) == target {
                    tmp_valid.insert(a);
                }
            }
        }
        valid = tmp_valid;
    }

    let solutions = valid.iter().sorted().collect::<Vec<_>>();
    **solutions.first().unwrap()
}

impl Day17 {
    pub fn new() -> Day17 {
        Day17 {
            day_number: 17,
            file_path: get_input_path(2024, 17),
        }
    }

    fn run_program(&self, registers: Registers, instructions: &Vec<u32>) -> (State, String) {
        let mut result = Vec::new();

        let mut state = State {
            instructions_index: 0,
            registers,
        };

        while state.instructions_index as usize <= instructions.len() - 2 {
            let operator_code = instructions[state.instructions_index];
            let operator = get_operator_by_code(operator_code);
            let (new_state, output) = operator.perform_operation(&instructions, state);
            if output.is_some() {
                result.push(output.unwrap().to_string());
            }
            state = new_state;
        }

        (state, result.join(","))
    }

    fn read_input(&self) -> (Registers, Vec<u32>) {
        let input = self.read_file_into_vec();
        let _register_a_line = input.get(0).unwrap();
        let _register_b_line = input.get(1).unwrap();
        let _register_c_line = input.get(2).unwrap();
        let _program_line = input.get(4).unwrap();

        let register_a = _register_a_line
            .split(": ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let register_b = _register_b_line
            .split(": ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let register_c = _register_c_line
            .split(": ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let program = _program_line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let registers = Registers {
            a: register_a,
            b: register_b,
            c: register_c,
        };
        (registers, program)
    }
}

impl BaseDay for Day17 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let (registers, program) = self.read_input();

        let (_, result) = self.run_program(registers, &program);
        Ok(result)
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let (_, program) = self.read_input();
        Ok(solve_part_2(program.iter().map(|p| *p as u64).collect()).to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn part_1_example_1_test() {
        init_logger();
        let _path = "./data/day_17/example_1.txt";

        let expected = "4,6,3,5,6,3,5,2,1,0";

        let mut day = Day17::new();
        let result = day.part_1();

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn bst_test() {
        init_logger();
        let register_a = 0;
        let register_b = 0;
        let register_c = 9;
        let instructions = vec![2, 6];
        let expected = 1;

        let day = Day17::new();
        let registers = Registers {
            a: register_a,
            b: register_b,
            c: register_c,
        };
        let (state, result) = day.run_program(registers, &instructions);

        assert_eq!(expected, state.registers.b);
        assert_eq!(result, "");
    }

    #[test]
    fn run_test() {
        init_logger();
        let register_a = 10;
        let register_b = 0;
        let register_c = 0;
        let instructions = vec![5, 0, 5, 1, 5, 4];
        let expected = "0,1,2";

        let day = Day17::new();
        let registers = Registers {
            a: register_a,
            b: register_b,
            c: register_c,
        };
        let (_, result) = day.run_program(registers, &instructions);

        assert_eq!(result, expected);
    }

    #[test]
    fn run_and_check_register_a_test() {
        init_logger();
        let register_a = 2024;
        let register_b = 0;
        let register_c = 0;
        let instructions = vec![0, 1, 5, 4, 3, 0];
        let expected = "4,2,5,6,7,7,7,7,3,1,0";
        let register_a_expected = 0;

        let day = Day17::new();
        let registers = Registers {
            a: register_a,
            b: register_b,
            c: register_c,
        };
        let (state, result) = day.run_program(registers, &instructions);

        assert_eq!(result, expected);
        assert_eq!(state.registers.a, register_a_expected);
    }

    #[test]
    fn bxl_test() {
        init_logger();
        let register_a = 0;
        let register_b = 29;
        let register_c = 0;
        let instructions = vec![1, 7];
        let expected = 26;

        let day = Day17::new();
        let registers = Registers {
            a: register_a,
            b: register_b,
            c: register_c,
        };
        let (state, result) = day.run_program(registers, &instructions);

        assert_eq!(expected, state.registers.b);
        assert_eq!(result, "");
    }

    #[test]
    fn bxc_test() {
        init_logger();
        let register_a = 0;
        let register_b = 2024;
        let register_c = 43690;
        let instructions = vec![4, 0];
        let expected = 44354;

        let day = Day17::new();
        let registers = Registers {
            a: register_a,
            b: register_b,
            c: register_c,
        };
        let (state, result) = day.run_program(registers, &instructions);

        assert_eq!(expected, state.registers.b);
        assert_eq!(result, "");
    }
}
