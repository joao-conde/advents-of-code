use aoc2020::crt::chinese_remainder;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day13").expect("failure opening input file");
    let buses = input.split('\n').nth(1).expect("missing buses");

    let depart = input
        .split('\n')
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
        .filter_map(|(i, bus)| bus.parse::<u64>().ok().and_then(|bus| Some((i as i64, bus as i64))))
        .map(|(i, bus_id)| (bus_id - i, bus_id))
        .unzip();

    println!("Part2: {}", chinese_remainder(&residues, &moduli).unwrap());
}
