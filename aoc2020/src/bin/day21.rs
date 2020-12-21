use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day21").expect("failure opening input file");

    let re = Regex::new(r"(.*) \(contains (.*)\)").unwrap();

    let mut all_ingredients = HashMap::new();
    let mut allerg_to_ing = HashMap::<&str, HashSet<&str>>::new();
    for line in input.lines() {
        let matches = re.captures(line).unwrap();
        let ingredients = matches.get(1).unwrap().as_str().split(" ").collect::<HashSet<&str>>();

        for i in &ingredients {
            *all_ingredients.entry(*i).or_insert(0) += 1;
        }

        let allergens = matches.get(2).unwrap().as_str().split(", ").collect::<Vec<&str>>();
        for allergen in allergens {
            if allerg_to_ing.contains_key(allergen) {
                let cur = allerg_to_ing.get(allergen).unwrap().clone();
                allerg_to_ing.insert(allergen, cur.intersection(&ingredients).copied().collect());
            } else {
                allerg_to_ing.insert(allergen, ingredients.clone());
            }
        }
    }

    let mut not_allergen_ing = all_ingredients.keys().copied().collect::<HashSet<&str>>();
    for ingredients in allerg_to_ing.values() {
        not_allergen_ing = not_allergen_ing.difference(ingredients).copied().collect();
    }

    let p1 = not_allergen_ing.iter().map(|i| all_ingredients.get(i).unwrap()).sum::<usize>();
    println!("Part1: {}", p1);

    while allerg_to_ing.values().any(|set| set.len() > 1) {
        let allergen = allerg_to_ing.values().find(|set| set.len() == 1).unwrap().to_owned();
        allerg_to_ing = allerg_to_ing
            .into_iter()
            .map(|(i, s)| match s.len() {
                1 => (i, s),
                _ => (i, s.difference(&allergen).copied().collect()),
            })
            .collect();
    }

    let mut matches = allerg_to_ing.iter().map(|(k, v)| (k, v.iter().next().unwrap())).collect::<Vec<(&&str, &&str)>>();
    matches.sort();

    let p2 = matches.iter().fold(String::new(), |mut canonical, ing| {
        canonical = format!("{},{}", canonical, ing.1);
        canonical
    });

    println!("Part2: {}", &p2[1..]);
}
