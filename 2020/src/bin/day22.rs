use std::collections::{HashSet, VecDeque};
use std::fs;

type Deck = VecDeque<usize>;

fn main() {
    let input = fs::read_to_string("input/day22").expect("failure opening input file");

    let (mut deck1, mut deck2) = build_decks(&input);
    combat(&mut deck1, &mut deck2);
    println!(
        "Part1: {}",
        score(if deck1.is_empty() { &deck2 } else { &deck1 })
    );

    let (mut deck1, mut deck2) = build_decks(&input);
    rec_combat(&mut deck1, &mut deck2);
    println!(
        "Part2: {}",
        score(if deck1.is_empty() { &deck2 } else { &deck1 })
    );
}

fn combat(deck1: &mut Deck, deck2: &mut Deck) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let draw1 = deck1.pop_front().unwrap();
        let draw2 = deck2.pop_front().unwrap();
        if draw1 > draw2 {
            deck1.push_back(draw1);
            deck1.push_back(draw2);
        } else {
            deck2.push_back(draw2);
            deck2.push_back(draw1);
        }
    }
}

fn rec_combat(deck1: &mut Deck, deck2: &mut Deck) -> bool {
    let mut prev_rounds = HashSet::<(Deck, Deck)>::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        let state = (deck1.clone(), deck2.clone());
        match prev_rounds.contains(&state) {
            true => return true,
            false => prev_rounds.insert(state),
        };

        let draw1 = deck1.pop_front().unwrap();
        let draw2 = deck2.pop_front().unwrap();

        let p1_win = if draw1 <= deck1.len() && draw2 <= deck2.len() {
            let mut next_deck1 = deck1.clone().into_iter().take(draw1).collect();
            let mut next_deck2 = deck2.clone().into_iter().take(draw2).collect();
            rec_combat(&mut next_deck1, &mut next_deck2)
        } else {
            draw1 > draw2
        };

        if p1_win {
            deck1.push_back(draw1);
            deck1.push_back(draw2);
        } else {
            deck2.push_back(draw2);
            deck2.push_back(draw1);
        }
    }

    deck2.is_empty()
}

fn score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * card)
        .sum()
}

fn build_decks(input: &str) -> (Deck, Deck) {
    let mut input = input.split("\n\n");
    let deck1 = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|card| card.parse().unwrap())
        .collect::<Deck>();
    let deck2 = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|card| card.parse().unwrap())
        .collect::<Deck>();
    (deck1, deck2)
}
