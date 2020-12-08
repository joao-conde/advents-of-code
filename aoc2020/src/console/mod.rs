use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RustyConsole {
    pub pc: i32,
    pub program: Vec<Instruction>,
    pub accumulator: i32,
    pub terminated: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP,
}

#[derive(Debug)]
pub enum InstructionError {
    MissingOperation,
    MissingArgument,
    InvalidOperation(String),
    InvalidArgumentValue(ParseIntError),
}

impl RustyConsole {
    pub fn step(&mut self) {
        let mut jmp = 1;
        match self.program[self.pc as usize] {
            Instruction::ACC(arg) => self.accumulator += arg,
            Instruction::JMP(arg) => jmp = arg,
            Instruction::NOP => (),
        }
        self.pc += jmp;
        self.terminated = (self.pc as usize) >= self.program.len();
    }
}

impl FromStr for RustyConsole {
    type Err = InstructionError;
    fn from_str(program: &str) -> Result<Self, Self::Err> {
        let instructions =
            program.split('\n').map(|instr| Instruction::from_str(instr)).collect::<Result<Vec<Instruction>, InstructionError>>()?;

        Ok(RustyConsole { pc: 0, program: instructions, accumulator: 0, terminated: false })
    }
}

impl FromStr for Instruction {
    type Err = InstructionError;
    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let mut fields = instruction.split(' ');
        let op = fields.next().ok_or(InstructionError::MissingOperation)?;
        let arg = fields.next().ok_or(InstructionError::MissingArgument)?;
        let arg = arg.parse::<i32>().map_err(InstructionError::InvalidArgumentValue)?;
        match op {
            "acc" => Ok(Instruction::ACC(arg)),
            "jmp" => Ok(Instruction::JMP(arg)),
            "nop" => Ok(Instruction::NOP),
            other => Err(InstructionError::InvalidOperation(other.to_string())),
        }
    }
}
