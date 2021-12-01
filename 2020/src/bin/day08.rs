use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input/day08").expect("failure opening input file");
    let console = RustyConsole::from_str(&input).expect("invalid list of instructions");
    println!("Part1: {}", p1(&mut console.clone()).0);
    println!("Part2: {}", p2(&console).0);
}

fn p1(console: &mut RustyConsole) -> (i32, bool) {
    let mut looped = false;
    let mut pcs = HashSet::new();
    while !console.terminated() && !looped {
        console.step();
        looped = !pcs.insert(console.pc);
    }
    (console.accumulator, looped)
}

fn p2(console: &RustyConsole) -> (i32, bool) {
    console
        .program
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut test_console = console.clone();
            test_console.program[i] = match test_console.program[i] {
                Instruction::Jmp(_) => Instruction::Nop,
                Instruction::Nop => Instruction::Jmp(1),
                _ => test_console.program[i],
            };
            p1(&mut test_console)
        })
        .find(|(_, looped)| !*looped)
        .expect("no fix for the infinite loop")
}

#[derive(Clone)]
pub struct RustyConsole {
    pub pc: i32,
    pub program: Vec<Instruction>,
    pub accumulator: i32,
}

#[derive(Clone, Copy)]
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
        let instructions = program
            .split('\n')
            .map(|instr| Instruction::from_str(instr))
            .collect::<Result<Vec<Instruction>, InstructionError>>()?;

        Ok(RustyConsole {
            pc: 0,
            program: instructions,
            accumulator: 0,
        })
    }
}

impl FromStr for Instruction {
    type Err = InstructionError;
    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let mut fields = instruction.split(' ');
        let op = fields.next().ok_or(InstructionError::MissingOperation)?;
        let arg = fields.next().ok_or(InstructionError::MissingArgument)?;
        let arg = arg
            .parse::<i32>()
            .map_err(InstructionError::InvalidArgument)?;
        match op {
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            "nop" => Ok(Instruction::Nop),
            other => Err(InstructionError::InvalidOperation(other.to_string())),
        }
    }
}
