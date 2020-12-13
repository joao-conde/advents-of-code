use ring_algorithm;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day13").expect("failure opening input file");
    let depart = input
        .split('\n')
        .next()
        .expect("missing earliest depart timestamp")
        .parse::<u64>()
        .expect("earliest depart timestamp must be a number");
    let buses = input.split('\n').nth(1).expect("missing buses").split(',').filter_map(|bus| bus.parse().ok()).collect::<Vec<u64>>();

    let p1 = buses.iter().map(|bus| (bus - depart % bus, bus)).min().and_then(|(bus, arrival)| Some(bus * arrival)).unwrap();
    println!("Part1: {}", p1);

    let (ts, moduli): (Vec<_>, Vec<_>) = input
        .split('\n')
        .nth(1)
        .expect("missing buses")
        .split(',')
        .enumerate()
        .filter_map(|(i, bus)| bus.parse::<u64>().ok().and_then(|bus| Some((i, bus))))
        .map(|(i, bus_id)| (-(i as isize), bus_id as isize))
        .unzip();
    println!("Part2: {}", ring_algorithm::chinese_remainder_theorem(&ts, &moduli).unwrap());
}
