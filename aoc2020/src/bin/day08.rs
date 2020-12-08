use aoc2020::console::{Instruction, RustyConsole};
use std::collections::HashSet;
use std::fs;
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
    while !console.terminated && !looped {
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
                Instruction::JMP(_) => Instruction::NOP,
                Instruction::NOP => Instruction::JMP(1),
                _ => test_console.program[i],
            };
            p1(&mut test_console)
        })
        .find(|(_, looped)| !*looped)
        .expect("no fix for the infinite loop")
}
