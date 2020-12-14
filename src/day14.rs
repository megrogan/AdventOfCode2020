use std::fs;

pub fn run() -> (u64, u64) {
    let input = fs::read_to_string("input/day14.txt").unwrap();
    (
        part1::Computer::new().run(&input),
        part2::Computer::new().run(&input),
    )
}

mod common {
    #[derive(Debug, Copy, Clone)]
    pub enum Instruction {
        SetMask(Mask),
        SetMemory(Write)
    }

    impl Instruction {
        pub fn parse(line: &str) -> Instruction {
            let mut parts = line.split(" = ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();

            match &left[0..4] {
                "mask" => {
                    Instruction::SetMask(Mask {
                        zeros: u64::from_str_radix(&right.replace("X","1"), 2).unwrap(),
                        ones: u64::from_str_radix(&right.replace("X","0"), 2).unwrap(),
                        floats: u64::from_str_radix(&right.replace("1","0").replace("X","1"), 2).unwrap(),
                    })
                },
                "mem[" => {
                    Instruction::SetMemory(Write {
                        location: left[4..left.len()-1].parse().unwrap(), 
                        value: right.parse().unwrap()
                    })
                },
                _ => panic!("Unknown instruction")
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Mask {
        pub zeros: u64,
        pub ones: u64,
        pub floats: u64
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Write {
        pub location: u64,
        pub value: u64
    }
}

mod part1 {
    use crate::day14::common;
    use std::collections::HashMap;
    use common::Instruction::*;
    use common::*;

    #[derive(Debug)]   
    pub struct Computer {
        memory: HashMap<u64, u64>,
        mask: Mask
    }

    impl Computer {

        pub fn new() -> Computer {
            Computer {
                memory: HashMap::new(),
                mask: Mask { zeros: 0, ones: 0, floats: 0 }
            }
        }

        pub fn run(&mut self, program: &str) -> u64 {
            let instructions: Vec<_> = program
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| line.len() > 0)
                    .map(Instruction::parse)
                    .collect();
    
            for i in instructions {
                match i {
                    SetMask(mask) => self.set_mask(mask),
                    SetMemory(write) => self.set_memory(write.location, write.value)
                };
            }

            self.get_memory_sum()
        }

        fn set_mask(&mut self, mask: Mask) {
            self.mask = mask;
        }

        fn set_memory(&mut self, location: u64, value: u64) {
            let value = value & self.mask.zeros;
            let value = value | self.mask.ones;
            self.memory.insert(location, value);
        }

        fn get_memory_sum(&self) -> u64 {
            self.memory.iter().map(|(_, value)| value).sum()
        } 
    }

    #[cfg(test)]
    mod tests {
        use crate::day14::*;
    
        #[test]
        fn test_run_program() {
            let input = r"
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

            let result = part1::Computer::new().run(&input);
            
            assert_eq!(165, result);
        }
    }
}

mod part2 {
    use crate::day14::common;
    use std::collections::HashMap;
    use common::Instruction::*;
    use common::*;

    #[derive(Debug)]   
    pub struct Computer {
        memory: HashMap<u64, u64>,
        mask: Mask,
        mask_combinations: Vec<u64>
    }

    impl Computer {

        pub fn new() -> Computer {
            Computer {
                memory: HashMap::new(),
                mask: Mask { zeros: 0, ones: 0, floats: 0 },
                mask_combinations: Vec::new()
            }
        }

        pub fn run(&mut self, program: &str) -> u64 {
            let instructions: Vec<_> = program
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| line.len() > 0)
                    .map(Instruction::parse)
                    .collect();
    
            for i in instructions {
                match i {
                    SetMask(mask) => self.set_mask(mask),
                    SetMemory(write) => self.set_memory(write.location, write.value)
                };
            }

            self.get_memory_sum()
        }

        fn set_mask(&mut self, mask: Mask) {
            self.mask = mask;
            self.mask_combinations = Computer::generate_combos(mask.floats);
        }

        fn generate_combos(mask: u64) -> Vec<u64> {
            let mut results: Vec<u64> = vec![0];
            for i in 0..36 {
                let num = 1 << i;
                if num & mask > 0 {
                    for j in 0..results.len() {
                        let c = results[j];
                        results.push(c | num);
                    }
                }
            }
            results
        }    

        fn set_memory(&mut self, location: u64, value: u64) {

            let location = location & (self.mask.floats ^ 0b111111111111111111111111111111111111_u64);
            let location = location | self.mask.ones;

            for a in &self.mask_combinations {
                self.memory.insert(location | a, value);
            }
        }

        fn get_memory_sum(&self) -> u64 {
            self.memory.iter().map(|(_, value)| value).sum()
        } 
    }

    #[cfg(test)]
    mod tests {
        use crate::day14::*;
    
        #[test]
        fn test_generate_combos() {
            let results = part2::Computer::generate_combos(10);
            assert_eq!(vec![0, 2, 8, 10], results);
        }

        #[test]
        fn test_program_2() {
            let input = r"
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

            let result = part2::Computer::new().run(&input);
            assert_eq!(208, result);
        }
    }
}