use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct RustyConsole {
    pub pc: i32,
    pub program: Vec<Instruction>,
    pub accumulator: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

#[derive(Debug)]
pub enum InstructionError {
    MissingOperation,
    MissingArgument,
    InvalidOperation(String),
    InvalidArgument(ParseIntError),
}

impl RustyConsole {
    pub fn run(&mut self) {
        while !self.terminated() {
            self.step()
        }
    }

    pub fn step(&mut self) {
        let mut jmp = 1;
        match self.program[self.pc as usize] {
            Instruction::Acc(arg) => self.accumulator += arg,
            Instruction::Jmp(arg) => jmp = arg,
            Instruction::Nop => (),
        }
        self.pc += jmp;
    }

    pub fn terminated(&self) -> bool {
        (self.pc as usize) >= self.program.len()
    }
}

impl FromStr for RustyConsole {
    type Err = InstructionError;
    fn from_str(program: &str) -> Result<Self, Self::Err> {
        let instructions =
            program.split('\n').map(|instr| Instruction::from_str(instr)).collect::<Result<Vec<Instruction>, InstructionError>>()?;

        Ok(RustyConsole { pc: 0, program: instructions, accumulator: 0 })
    }
}

impl FromStr for Instruction {
    type Err = InstructionError;
    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let mut fields = instruction.split(' ');
        let op = fields.next().ok_or(InstructionError::MissingOperation)?;
        let arg = fields.next().ok_or(InstructionError::MissingArgument)?;
        let arg = arg.parse::<i32>().map_err(InstructionError::InvalidArgument)?;
        match op {
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            "nop" => Ok(Instruction::Nop),
            other => Err(InstructionError::InvalidOperation(other.to_string())),
        }
    }
}
