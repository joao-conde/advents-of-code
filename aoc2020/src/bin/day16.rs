use std::{fs, ops::Range};

const DEPARTURE_COLS: Range<usize> = 0..6;

type Rule = Vec<Range<usize>>;
type Ticket = Vec<usize>;

fn main() {
    let input = fs::read_to_string("input/day16").expect("failure opening input file");
    let (rules, ticket, tickets) = parse_input(&input);
    println!("Part1: {}", p1(&rules, &tickets));
    println!("Part2: {}", p2(&rules, &tickets, &ticket));
}

fn p1(rules: &[Rule], tickets: &[Ticket]) -> usize {
    tickets.iter().map(|t| t.iter().map(|num| if is_valid(&num, rules) { 0 } else { *num })).flatten().sum()
}

fn p2(rules: &[Rule], tickets: &[Ticket], ticket: &Ticket) -> usize {
    let valid_tickets = tickets.iter().filter(|t| t.iter().all(|num| is_valid(&num, &rules))).collect::<Vec<&Ticket>>();

    let mut rules_per_col = (0..valid_tickets[0].len())
        .map(|col_i| {
            let col = valid_tickets.iter().filter_map(|t| t.get(col_i)).collect::<Vec<&usize>>();
            (0..rules.len())
                .filter(|rule_i| {
                    let rule = rules.get(*rule_i).unwrap();
                    col.iter().all(|num| rule.iter().any(|range| range.contains(&num)))
                })
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut col_to_rule = vec![usize::MAX; valid_tickets[0].len()];
    while col_to_rule.iter().any(|el| *el == usize::MAX) {
        let (i, rule) = rules_per_col.iter().enumerate().find(|(_, x)| x.len() == 1).map(|(i, rules)| (i, rules[0])).unwrap();
        col_to_rule[i] = rule;
        rules_per_col.iter_mut().for_each(|rules| {
            if let Some(i) = rules.iter().position(|x| *x == rule) {
                rules.remove(i);
            }
        });
    }

    col_to_rule.iter().enumerate().filter(|(_, col)| DEPARTURE_COLS.contains(*col)).map(|(pos, _)| ticket[pos]).product()
}

fn is_valid(num: &usize, rules: &[Rule]) -> bool {
    rules.iter().any(|rule| rule.iter().any(|range| range.contains(&num)))
}

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut input = input.split("\n\n");
    let rules = input.next().unwrap().split('\n').map(parse_rule).collect::<Vec<Rule>>();
    let ticket = input.next().unwrap().split('\n').nth(1).map(parse_ticket).unwrap();
    let tickets = input.next().unwrap().split('\n').skip(1).map(parse_ticket).collect::<Vec<Ticket>>();
    (rules, ticket, tickets)
}

fn parse_rule(rule: &str) -> Rule {
    rule.split(':')
        .nth(1)
        .unwrap()
        .split(" or ")
        .map(|r| {
            let range = r.split('-').filter_map(|n| n.trim().parse().ok()).collect::<Vec<usize>>();
            range[0]..range[1] + 1
        })
        .collect()
}

fn parse_ticket(ticket: &str) -> Ticket {
    ticket.split(',').filter_map(|n| n.parse().ok()).collect()
}
