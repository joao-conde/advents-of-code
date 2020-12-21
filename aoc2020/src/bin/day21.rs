use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day21").expect("failure opening input file");
    let re = Regex::new(r"(.*) \(contains (.*)\)").unwrap();

    let mut ings_freq = HashMap::new();
    let mut allerg_to_ings = HashMap::<&str, HashSet<&str>>::new();
    input.lines().for_each(|line| {
        let matches = re.captures(line).unwrap();
        let ings = matches.get(1).unwrap().as_str().split(' ').collect::<HashSet<&str>>();
        let allergens = matches.get(2).unwrap().as_str().split(", ").collect::<Vec<&str>>();

        ings.iter().for_each(|i| *ings_freq.entry(*i).or_insert(0) += 1);

        for allergen in allergens {
            match allerg_to_ings.get(allergen) {
                Some(cur) => {
                    let cur = cur.to_owned();
                    allerg_to_ings.insert(allergen, cur.to_owned().intersection(&ings).copied().collect())
                }
                None => allerg_to_ings.insert(allergen, ings.clone()),
            };
        }
    });

    println!("Part1: {}", p1(&allerg_to_ings, &ings_freq));
    println!("Part2: {}", p2(allerg_to_ings));
}

fn p1(allerg_to_ings: &HashMap<&str, HashSet<&str>>, ings_freq: &HashMap<&str, usize>) -> usize {
    let mut not_allergen_ings = ings_freq.keys().copied().collect::<HashSet<&str>>();
    allerg_to_ings.values().for_each(|ingredients| {
        not_allergen_ings = not_allergen_ings.difference(ingredients).copied().collect();
    });
    not_allergen_ings.iter().map(|i| ings_freq.get(i).unwrap()).sum()
}

fn p2(mut allerg_to_ings: HashMap<&str, HashSet<&str>>) -> String {
    while allerg_to_ings.values().any(|set| set.len() > 1) {
        let allergen = allerg_to_ings.values().find(|set| set.len() == 1).unwrap().to_owned();
        allerg_to_ings = allerg_to_ings
            .into_iter()
            .map(|(i, s)| match s.len() {
                1 => (i, s),
                _ => (i, s.difference(&allergen).copied().collect()),
            })
            .collect();
    }

    let mut matches = allerg_to_ings.iter().map(|(k, v)| (k, v.iter().next().unwrap())).collect::<Vec<(&&str, &&str)>>();
    matches.sort();
    matches.iter().fold(String::new(), |mut canonical, ing| {
        canonical = format!("{},{}", canonical, ing.1);
        canonical
    })[1..]
        .to_string()
}
