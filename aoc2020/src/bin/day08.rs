use aoc2020::vm::{Instruction, VM};
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input/day08").expect("failure opening input file");
    let vm = VM::from_str(&input).expect("invalid list of instructions");
    println!("Part1: {}", p1(&mut vm.clone()).0);
    println!("Part2: {}", p2(&vm));
}

fn p1(vm: &mut VM) -> (i32, bool) {
    let mut looped = false;
    let mut pcs = HashSet::new();
    while !vm.terminated && !looped {
        vm.process_instr();
        looped = !pcs.insert(vm.pc);
    }
    (vm.accumulator, looped)
}

fn p2(vm: &VM) -> i32 {
    for i in 0..vm.program.len() {
        let mut test_vm = vm.clone();
        test_vm.program[i] = match test_vm.program[i] {
            Instruction::JMP(_) => Instruction::NOP,
            Instruction::NOP => Instruction::JMP(1),
            _ => test_vm.program[i],
        };

        let (acc, looped) = p1(&mut test_vm);
        if !looped {
            return acc;
        }
    }
    0
}
