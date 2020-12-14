use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day14").expect("failure opening input file");
    println!("Part1: {}", execute(&input, update_mem1));
    println!("Part2: {}", execute(&input, update_mem2));
}

fn execute(input: &str, update_mem: fn(&mut HashMap<usize, usize>, &str, usize, usize)) -> usize {
    let re = Regex::new(r"mem\[(?P<addr>[0-9]*)\]").unwrap();
    let mut mem = HashMap::new();
    let mut mask = "";
    for line in input.split('\n') {
        let mut instr = line.split(" = ");
        let (lhs, rhs) = (instr.next().unwrap(), instr.next().unwrap());
        match lhs {
            "mask" => mask = rhs,
            _ => {
                let addr = re.captures(lhs).unwrap()["addr"].parse::<usize>().unwrap();
                let val = rhs.parse::<usize>().unwrap();
                update_mem(&mut mem, mask, addr, val);
            }
        }
    }
    mem.values().sum::<usize>()
}

fn update_mem1(mem: &mut HashMap<usize, usize>, mask: &str, addr: usize, val: usize) {
    let val = format!("{:036b}", val)
        .chars()
        .zip(mask.chars())
        .map(|(vc, mc)| match mc {
            'X' => vc,
            _ => mc,
        })
        .collect::<String>();
    let val = usize::from_str_radix(&val, 2).unwrap();
    mem.insert(addr, val);
}

fn update_mem2(mem: &mut HashMap<usize, usize>, mask: &str, addr: usize, val: usize) {
    let addr = format!("{:036b}", addr)
        .chars()
        .zip(mask.chars())
        .map(|(vc, mc)| match mc {
            '0' => vc,
            _ => mc,
        })
        .collect::<String>();
    for addr in combinations(&addr) {
        let addr = usize::from_str_radix(&addr, 2).unwrap();
        mem.insert(addr, val);
    }
}

fn combinations(addr: &str) -> Vec<String> {
    if addr.contains('X') {
        vec![combinations(&addr.replacen("X", "0", 1)), combinations(&addr.replacen("X", "1", 1))].into_iter().flatten().collect()
    } else {
        vec![addr.to_owned()]
    }
}
