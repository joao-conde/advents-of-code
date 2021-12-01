use std::fs;

fn main() {
    let input = fs::read_to_string("input/day13").expect("failure opening input file");
    let buses = input.lines().nth(1).expect("missing buses");

    let depart = input
        .lines()
        .next()
        .expect("missing earliest depart timestamp")
        .parse::<u64>()
        .expect("earliest depart timestamp must be an unsigned integer");
    let p1 = buses
        .split(',')
        .flat_map(|bus| bus.parse::<u64>().ok())
        .map(|bus| (bus - depart % bus, bus))
        .min()
        .map(|(waiting, bus)| bus * waiting)
        .unwrap();
    println!("Part1: {}", p1);

    let (residues, moduli): (Vec<_>, Vec<_>) = buses
        .split(',')
        .enumerate()
        .filter_map(|(i, bus)| bus.parse::<u64>().ok().map(|bus| (i as i64, bus as i64)))
        .map(|(i, bus_id)| (bus_id - i, bus_id))
        .unzip();
    println!("Part2: {}", chinese_remainder(&residues, &moduli).unwrap());
}

/// Chinese Remainder Theorem solver
/// source: https://github.com/acmeism/RosettaCodeData/blob/master/Task/Chinese-remainder-theorem/Rust/chinese-remainder-theorem.rust
pub fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
