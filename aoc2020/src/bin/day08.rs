use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input/day08").expect("failure opening input file");

    let mut instructions = input.split('\n').into_iter().map(|s| s.to_owned()).collect();

    let (acc, _) = emulate(&instructions);
    println!("Part1: {}", acc);

    let acc = mutate(&mut instructions);
    println!("Part2: {}", acc)
}

fn mutate(instructions: &mut Vec<String>) -> i64 {
    for i in 0..instructions.len() {
        let original = instructions[i].clone();
    
        instructions[i] = if instructions[i].contains("nop"){
            instructions[i].replace("nop", "jmp")
        } else {
            instructions[i].replace("jmp", "nop")
        };
        
        let (acc, looped) = emulate(&instructions);
        instructions[i] = original;

        if !looped {
            return acc;
        }
    }
    -1
}

fn emulate(instructions: &Vec<String>) -> (i64, bool) {
    let mut set = HashSet::new();
    let mut looped = false;
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    while (pc as usize) < instructions.len() {
        if set.contains(&pc) {
            looped = true;
            break;
        }
        set.insert(pc);
        let line = instructions[pc as usize].split(" ").collect::<Vec<&str>>();
        let (op, arg) = (line[0], line[1]);
        let arg = arg.parse::<i64>().unwrap();
        let mut jmp: i64 = 1;
        match op {
            "acc" => acc += arg,
            "jmp" => jmp = arg,
            "nop" => (),
            _ => panic!("op not recognized")
        }
        
        pc += jmp;
    }
    (acc, looped)
}
