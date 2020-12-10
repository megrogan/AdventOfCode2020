use crate::day08::Instruction::*;
use std::fs;

pub fn run() -> (i32, i32) {
    let input = fs::read_to_string("input/day8.txt").unwrap();

    let mut instructions = Program::load_instructions_from_str(&input);

    let result1: i32;
    {
        result1 = run_original_program(&mut instructions);
    }

    let result2: i32;
    {
        result2 = fix_program(&mut instructions).expect("Could not fix program");
    }

    (
        result1, 
        result2
    )
}

fn run_original_program(instructions: &mut Vec<Instruction>) -> i32 {
    
    let mut program = Program::load(&instructions);
    let result = program.run();

    match result {
        Ok(value) => value,
        Err(err) => {
            log::debug!("{}", err.message);
            log::debug!("program exited at instruction {}", err.counter);
            err.value
        }
    }
}

fn fix_program(instructions: &mut Vec<Instruction>) -> Option<i32> {
    
    for i in 0..instructions.len() {
        let instruction = instructions[i];

        if let JMP(_) = instruction {
            instructions[i] = NOP;
            let mut program = Program::load(&instructions);
            let result = program.run();
            instructions[i] = instruction;

            match result {
                Ok(value) => return Some(value),
                Err(err) => {
                    log::debug!("{}", err.message);
                    log::debug!("program exited at instruction {}", err.counter);
                    log::debug!("");
                }
            }
        }
    } 

    None
}

#[derive(Debug)]
struct Program {
    instructions: Vec<(Instruction, bool)>,
    counter: i32,
    value: i32,
}

impl Program {

    pub fn load_instructions_from_str(input: &str) -> Vec<Instruction> {

        fn parse_instruction(line: &str) -> Option<Instruction> {

            let mut parts = line.split_whitespace();

            let instruction = parts.next()?.trim();
            let value = parts
                .next()?
                .trim()
                .trim_start_matches('+')
                .parse()
                .ok()?;

            match instruction {
                "acc" => Some(ACC(value)),
                "jmp" => Some(JMP(value)),
                "nop" => Some(NOP),
                _ => None
            }
        }

        input
            .lines()
            .filter_map(parse_instruction)
            .collect()
    }

    pub fn load(instructions: &Vec<Instruction>) -> Program {
        Program {
            counter: 0,
            value: 0,
            instructions: instructions
                .iter()
                .map(|i| (*i, false))
                .collect()
        }
    }

    pub fn run(&mut self) -> Result<i32, ProgramErr> {

        let mut i = 0;

        loop {
            log::debug!(
                "Counter {}, Value {}, Iteration {}",
                self.counter,
                self.value,
                i);

            if self.counter >= (self.instructions.len() as i32) {
                return Ok(self.value);
            }

            if self.counter < 0 {
                return Err(ProgramErr { 
                    message: String::from("counter < 0"),
                    value: self.value, 
                    counter: self.counter
                });
            }

            let mut instruction = &mut self.instructions[self.counter as usize];
            
            if instruction.1 {
                return Err(ProgramErr { 
                    message: String::from("loop detected"),
                    value: self.value, 
                    counter: self.counter
                });                
            }

            match instruction.0 {
                ACC(value) => {
                    self.value += value;
                    self.counter += 1;
                },
                JMP(value) => 
                    self.counter += value,
                NOP => 
                    self.counter += 1
            };

            instruction.1 = true;

            i += 1;
        }
    }   
}

#[derive(Debug)]
struct ProgramErr {
    message: String,
    value: i32,
    counter: i32,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NOP,
    ACC(i32),
    JMP(i32)
}

#[cfg(test)]
mod tests {
    use crate::day08::*;

    #[test]
    fn test_program() {
        let input = "
        nop +0\n
        acc +1\n
        jmp +4\n
        acc +3\n
        jmp -3\n
        acc -99\n
        acc +1\n
        jmp -4\n
        acc +6";

        let mut program = Program::load(&Program::load_instructions_from_str(input));
        let result = program.run();

        assert!(result.is_err());
        assert_eq!(5, result.unwrap_err().value);
    }

    #[test]
    fn test_fix_program() {
        let input = "
        nop +0\n
        acc +1\n
        jmp +4\n
        acc +3\n
        jmp -3\n
        acc -99\n
        acc +1\n
        jmp -4\n
        acc +6";

        let mut instructions = Program::load_instructions_from_str(&input);

        let result = fix_program(&mut instructions);

        assert!(result.is_some());
        assert_eq!(8, result.unwrap());
    }
}