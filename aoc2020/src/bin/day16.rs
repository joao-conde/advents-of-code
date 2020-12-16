use std::{fs, ops::Range};

type Rule = Vec<Range<usize>>;
type Ticket = Vec<usize>;

fn main() {
    let input = fs::read_to_string("input/day16").expect("failure opening input file");
    let (rules, ticket, tickets) = parse_input(&input);
    println!("Part1: {}", p1(&rules, &tickets));
    println!("Part2: {}", p2(&rules, &tickets, &ticket));
}

fn p1(rules: &Vec<Rule>, tickets: &Vec<Ticket>) -> usize {
    tickets.iter().map(|t| t.iter().map(|num| if is_valid(&num, &rules) { 0 } else { *num })).flatten().sum()
}

fn p2(rules: &Vec<Rule>, tickets: &Vec<Ticket>, ticket: &Ticket) -> usize {
    let valid_tickets = tickets.iter().filter(|t| t.iter().all(|num| is_valid(&num, &rules))).collect::<Vec<&Ticket>>();

    let mut col_rules = (0..valid_tickets[0].len())
        .map(|col_i| {
            let col = valid_tickets.iter().filter_map(|t| t.iter().nth(col_i)).collect::<Vec<&usize>>();
            (0..rules.len())
                .filter(|rule_i| {
                    let rule = rules.iter().nth(*rule_i).unwrap();
                    col.iter().all(|num| rule.iter().any(|range| range.contains(&num)))
                })
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut map = vec![usize::MAX; valid_tickets[0].len()];
    while map.iter().any(|el| *el == usize::MAX) {
        let (i, certain) = col_rules.iter().enumerate().find(|(_, x)| x.len() == 1).unwrap();
        map[i] = certain[0];
        col_rules = col_rules
            .iter()
            .map(|l| match l.into_iter().position(|x| *x == certain[0]) {
                Some(i) => {
                    let mut c = l.clone();
                    c.remove(i);
                    c
                }
                None => l.clone(),
            })
            .collect();
    }

    let pos = map.iter().enumerate().filter(|(_, col)| (0..6).contains(*col)).map(|(pos, _)| pos).collect::<Vec<usize>>();
    pos.iter().map(|i| ticket[*i]).product()
}

fn is_valid(num: &usize, rules: &Vec<Rule>) -> bool {
    rules.iter().any(|rule| rule.iter().any(|range| range.contains(&num)))
}

fn parse_input(input: &String) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut input = input.split("\n\n");

    let rules = input
        .next()
        .unwrap()
        .split("\n")
        .map(|rule| {
            rule.split(":")
                .nth(1)
                .unwrap()
                .split(" or ")
                .map(|r| {
                    let range = r.split("-").filter_map(|n| n.trim().parse().ok()).collect::<Vec<usize>>();
                    range[0]..range[1] + 1
                })
                .collect::<Rule>()
        })
        .collect::<Vec<Rule>>();

    let ticket = input.next().unwrap().split("\n").nth(1).map(|t| t.split(',').filter_map(|n| n.parse().ok())).unwrap().collect::<Ticket>();

    let tickets =
        input.next().unwrap().split("\n").skip(1).map(|t| t.split(',').filter_map(|n| n.parse().ok()).collect()).collect::<Vec<Ticket>>();

    (rules, ticket, tickets)
}
