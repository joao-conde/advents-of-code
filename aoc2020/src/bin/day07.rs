use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day07").expect("failure opening input file");
    let map = parse_input(input);
    let my_bag = "shiny gold".to_string();
    let p1 = map
        .keys()
        .filter(|key| **key != my_bag)
        .map(|key| closure(key.to_string(), &map))
        .filter(|closure| closure.contains(&my_bag))
        .count();
    println!("Part1: {}", p1);
    println!("Part2: {}", num_bags(my_bag, &map) - 1);
}

fn closure(bag: String, map: &HashMap<String, Vec<(u32, String)>>) -> Vec<String> {
    map.get(&bag)
        .map(|bags| {
            bags.iter().fold(vec![bag], |mut acc, (_, bag)| {
                acc.append(&mut closure(bag.to_string(), &map));
                acc
            })
        })
        .unwrap_or_else(Vec::new)
}

fn num_bags(bag: String, map: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    map.get(&bag)
        .map(|bags| {
            bags.iter().fold(1, |mut acc, (quant, bag)| {
                acc += quant * num_bags(bag.to_string(), &map);
                acc
            })
        })
        .unwrap_or(1)
}

fn parse_input(input: String) -> HashMap<String, Vec<(u32, String)>> {
    let mut map = HashMap::new();
    for line in input.split('\n') {
        let line = line.replace(" bags", "").replace(" bag", "").replace(".", "").replace(", ", ",");
        let line = line.split(" contain ").collect::<Vec<&str>>();
        let (bag, bags) = (line[0].to_owned(), line[1]);
        let bags = bags.split(',').filter(|x| *x != "no other").collect::<Vec<&str>>();
        for b in bags {
            let mut b = b.chars();
            let quant = b.next().and_then(|q| q.to_digit(10)).unwrap_or(0);
            let b = b.collect::<String>();
            map.entry(bag.to_owned()).or_insert_with(Vec::new).push((quant, b.trim().to_owned()));
        }
    }
    map
}
