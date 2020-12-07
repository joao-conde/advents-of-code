use std::collections::HashMap;
use std::fs;

type BagTree = HashMap<String, Vec<(u32, String)>>;

fn main() {
    let input = fs::read_to_string("input/day07").expect("failure opening input file");
    let map = parse_input(&input);
    let my_bag = "shiny gold";
    let p1 = map.keys().filter(|key| **key != my_bag).map(|key| closure(key, &map)).filter(|closure| closure.contains(&my_bag)).count();
    println!("Part1: {}", p1);
    println!("Part2: {}", num_bags(my_bag, &map) - 1);
}

fn closure<'a>(bag: &'a str, map: &'a BagTree) -> Vec<&'a str> {
    map.get(bag)
        .map(|bags| {
            bags.iter().fold(vec![bag], |mut acc, (_, bag)| {
                acc.append(&mut closure(bag, &map));
                acc
            })
        })
        .unwrap_or_else(Vec::new)
}

fn num_bags<'a>(bag: &'a str, map: &BagTree) -> u32 {
    map.get(bag)
        .map(|bags| {
            bags.iter().fold(1, |mut acc, (quant, bag)| {
                acc += quant * num_bags(bag, &map);
                acc
            })
        })
        .unwrap_or(1)
}

fn parse_input(input: &str) -> BagTree {
    let mut map = HashMap::new();
    for line in input.split('\n') {
        let line = line.replace(" bags", "").replace(" bag", "").replace(".", "").replace(", ", ",");
        let mut line = line.split(" contain ");
        let (bag, bags) = (line.next().unwrap(), line.next().unwrap());
        let bags = bags.split(',').filter(|b| *b != "no other").collect::<Vec<&str>>();
        for child_bag in bags {
            let mut child_bag = child_bag.chars();
            let quantity = child_bag.next().and_then(|q| q.to_digit(10)).unwrap_or(0);
            let child_bag = child_bag.collect::<String>();
            let (bag, child_bag) = (bag.to_owned(), child_bag.trim().to_owned());
            map.entry(bag).or_insert_with(Vec::new).push((quantity, child_bag));
        }
    }
    map
}
