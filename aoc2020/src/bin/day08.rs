use aoc2020::vm::VM;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input/day08").expect("failure opening input file");
    println!("Part1: {}", p1(&input).0);
    println!("Part2: {}", p2(&input));
}

fn p1(input: &str) -> (i32, bool) {
    let mut vm = VM::from_str(&input).unwrap();
    let mut looped = false;
    let mut pcs = HashSet::new();
    while !looped {
        vm.process_instr();
        looped = !pcs.insert(vm.pc);
    }
    (vm.accumulator, looped)
}

fn p2(input: &str) -> i32 {
    let mut instructions = input.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();
    for i in 0..instructions.len() {
        let original = instructions[i].clone();

        instructions[i] = if instructions[i].contains("nop") {
            instructions[i].replace("nop", "jmp")
        } else if instructions[i].contains("jmp") {
            instructions[i].replace("jmp", "nop")
        } else {
            continue;
        };

        let instr_str = instructions.join("\n");

        let mut vm = VM::from_str(&instr_str).unwrap();
        let mut looped = false;
        let mut pcs = HashSet::new();
        while !vm.terminated && !looped {
            vm.process_instr();
            looped = !pcs.insert(vm.pc);
        }
        if !looped {
            return vm.accumulator;
        }
        instructions[i] = original;
    }
    -1
}
