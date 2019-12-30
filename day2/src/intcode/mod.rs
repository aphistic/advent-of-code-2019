use std::fs;
use std::error::Error;

type Opcode = u32;

const OP_ADD: Opcode = 1;
const OP_MUL: Opcode = 2;
const OP_HLT: Opcode = 99;

pub struct Program {
    pc: usize,
    memory: Vec<Opcode>,
    code: Vec<Opcode>,
}

impl Program {
    pub fn load(code: Vec<Opcode>) -> Program {
        Program {
            pc: 0,
            memory: code.clone(),
            code,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Program, String> {
        match fs::read_to_string(path) {
            Ok(data) => {
                let mut code: Vec<Opcode> = Vec::new();
                for item in data.split(',') {
                    match item.trim() {
                        "" => continue,
                        opcode => match opcode.parse::<Opcode>() {
                            Ok(opcode) => code.push(opcode),
                            Err(_) => return Err(format!("invalid opcode found: {}", opcode))
                        }
                    }
                }

                Ok(Program::load(code))
            }
            Err(e) => Err(e.description().to_string()),
        }
    }

    pub fn memory_at(&self, idx: usize) -> Option<Opcode> {
        match self.memory.len() {
            size if idx < size => Some(self.memory[idx]),
            _ => None,
        }
    }

    pub fn reset(&mut self) {
        self.memory = self.code.clone();
        self.pc = 0;
    }

    pub fn run(&mut self) -> Result<bool, String> {
        loop {
            match self.step() {
                Ok(complete) => match complete {
                    true => return Ok(true),
                    _ => continue,
                }
                e => return e,
            }
        }
    }

    pub fn call(&mut self, noun: Opcode, verb: Opcode) -> Result<Opcode, String> {
        self.reset();
        match self.memory.len() {
            l if l < 3 => Err(format!("invalid program length")),
            _ => {
                self.memory[1] = noun;
                self.memory[2] = verb;
                match self.run() {
                    Ok(_) => match self.memory_at(0) {
                        Some(opcode) => Ok(opcode),
                        None => Err(format!("result value not found"))
                    }
                    Err(e) => Err(format!("error running program: {}", e))
                }
            }
        }
    }

    fn step(&mut self) -> Result<bool, String> {
        let code = self.memory[self.pc];
        match code {
            OP_ADD => self.op_add(),
            OP_MUL => self.op_mul(),
            OP_HLT => Ok(true),
            _ => Err(format!("unknown op code {}", code))
        }
    }

    fn op_add(&mut self) -> Result<bool, String> {
        if self.memory[self.pc] != OP_ADD {
            return Err(String::from("OP_ADD: unexpected opcode"));
        }

        let mem_len = self.memory.len();

        if self.pc + 3 > mem_len {
            return Err(String::from("OP_ADD: invalid length"));
        }

        let a = self.memory[self.pc + 1] as usize;
        if a > mem_len {
            return Err(format!("OP_ADD: a value out of range: {}", a));
        }
        let b = self.memory[self.pc + 2] as usize;
        if b > mem_len {
            return Err(format!("OP_ADD: b value out of range: {}", b));
        }
        let dest = self.memory[self.pc + 3] as usize;
        if dest > mem_len {
            return Err(format!("OP_ADD: dest value out of range: {}", dest));
        }

        self.memory[dest] = self.memory[a] + self.memory[b];
        self.pc += 4;

        Ok(false)
    }

    fn op_mul(&mut self) -> Result<bool, String> {
        if self.memory[self.pc] != OP_MUL {
            return Err(String::from("OP_MUL: unexpected opcode"));
        }

        let mem_len = self.memory.len();

        if self.pc + 3 > mem_len {
            return Err(String::from("OP_MUL: invalid length"));
        }

        let a = self.memory[self.pc + 1] as usize;
        if a > mem_len {
            return Err(format!("OP_MUL: a value out of range: {}", a));
        }
        let b = self.memory[self.pc + 2] as usize;
        if b > mem_len {
            return Err(format!("OP_MUL: b value out of range: {}", b));
        }
        let dest = self.memory[self.pc + 3] as usize;
        if dest > mem_len {
            return Err(format!("OP_MUL: dest value out of range: {}", dest));
        }

        self.memory[dest] = self.memory[a] * self.memory[b];
        self.pc += 4;

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    mod program {
        use super::super::*;

        #[test]
        fn memory_at() {
            let p = Program::load(vec![0, 1, 2, 3]);
            assert_eq!(Some(0), p.memory_at(0));
            assert_eq!(Some(1), p.memory_at(1));
            assert_eq!(Some(2), p.memory_at(2));
            assert_eq!(Some(3), p.memory_at(3));
        }

        #[test]
        fn memory_at_out_of_range() {
            let p = Program::load(vec![0, 1, 2, 3]);
            assert_eq!(None, p.memory_at(4))
        }

        #[test]
        fn step_add_once_at_start() {
            let mut p = Program::load(vec![1, 0, 0, 0]);
            assert_eq!(Ok(false), p.step());
            assert_eq!(vec![2, 0, 0, 0], p.memory);
            assert_eq!(4, p.pc);
        }

        #[test]
        fn step_add_twice() {
            let mut p = Program::load(vec![1, 0, 0, 0, 1, 0, 0, 0]);
            assert_eq!(Ok(false), p.step());
            assert_eq!(vec![2, 0, 0, 0, 1, 0, 0, 0], p.memory);
            assert_eq!(4, p.pc);
            assert_eq!(Ok(false), p.step());
            assert_eq!(vec![4, 0, 0, 0, 1, 0, 0, 0], p.memory);
            assert_eq!(8, p.pc);
        }

        #[test]
        fn step_mul_once_at_start() {
            let mut p = Program::load(vec![2, 0, 0, 0]);
            assert_eq!(Ok(false), p.step());
            assert_eq!(vec![4, 0, 0, 0], p.memory);
            assert_eq!(4, p.pc);
        }

        #[test]
        fn step_mul_twice() {
            let mut p = Program::load(vec![2, 0, 0, 0, 2, 0, 0, 0]);
            assert_eq!(Ok(false), p.step());
            assert_eq!(vec![4, 0, 0, 0, 2, 0, 0, 0], p.memory);
            assert_eq!(4, p.pc);
            assert_eq!(Ok(false), p.step());
            assert_eq!(vec![16, 0, 0, 0, 2, 0, 0, 0], p.memory);
            assert_eq!(8, p.pc);
        }

        #[test]
        fn step_hlt() {
            let mut p = Program::load(vec![99]);
            assert_eq!(Ok(true), p.step());
            assert_eq!(0, p.pc);
            assert_eq!(Ok(true), p.step());
            assert_eq!(0, p.pc);
        }

        #[test]
        fn step_hlt_after_add() {
            let mut p = Program::load(vec![1, 0, 0, 0, 99]);
            assert_eq!(Ok(false), p.step());
            assert_eq!(vec![2, 0, 0, 0, 99], p.memory);
            assert_eq!(4, p.pc);
            assert_eq!(Ok(true), p.step());
            assert_eq!(4, p.pc);
            assert_eq!(Ok(true), p.step());
        }

        #[test]
        fn run_hlt_after_two_adds() {
            let mut p = Program::load(vec![1, 0, 0, 0, 1, 0, 0, 0, 99]);
            assert_eq!(Ok(true), p.run());
            assert_eq!(vec![4, 0, 0, 0, 1, 0, 0, 0, 99], p.memory);
            assert_eq!(8, p.pc);
        }

        #[test]
        fn call_with_noun_and_verb() {
            let mut p = Program::load(vec![1, 0, 0, 0, 99, 10, 20]);
            assert_eq!(Ok(30), p.call(5, 6))
        }

        #[test]
        fn example1() {
            let mut p = Program::load(vec![1, 0, 0, 0, 99]);
            assert_eq!(Ok(true), p.run());
            assert_eq!(vec![2, 0, 0, 0, 99], p.memory);
        }

        #[test]
        fn example2() {
            let mut p = Program::load(vec![2, 3, 0, 3, 99]);
            assert_eq!(Ok(true), p.run());
            assert_eq!(vec![2, 3, 0, 6, 99], p.memory);
        }

        #[test]
        fn example3() {
            let mut p = Program::load(vec![2, 4, 4, 5, 99, 0]);
            assert_eq!(Ok(true), p.run());
            assert_eq!(vec![2, 4, 4, 5, 99, 9801], p.memory);
        }

        #[test]
        fn example4() {
            let mut p = Program::load(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
            assert_eq!(Ok(true), p.run());
            assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], p.memory);
        }
    }
}