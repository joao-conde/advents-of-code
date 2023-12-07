use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    kind: HandKind,
    bid: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    Five(Card),
    Four(Card),
    Full(Card, Card),
    Three(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    High(Card),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(usize),
    Joker,
}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards.chars().map(|c| c.into()).collect::<Vec<Card>>();
        let cards = cards.try_into().unwrap();
        let kind = HandKind::new(cards);
        let bid = bid.parse().unwrap();
        Hand { cards, kind, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let s1 = self.kind.strength();
        let s2 = other.kind.strength();
        match s1.cmp(&s2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                // find the first pair of cards in both hands
                // that differs and compare that pair, if none
                // compares the first pair (will return equal)
                let (c1, c2) = self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .filter(|(c1, c2)| c1 != c2)
                    .nth(0)
                    .unwrap_or((&self.cards[0], &other.cards[0]));
                c1.partial_cmp(c2).unwrap()
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HandKind {
    fn new(cards: [Card; 5]) -> HandKind {
        let mut counts: HashMap<Card, usize> = cards.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });

        // removes the jokers from the card counts to be later
        // added to the most frequent card count or returns
        // immediately the best hand possible of five aces
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        if jokers == 5 {
            return HandKind::Five(Card::Ace);
        }

        // sorts the cards and counts by frequency and adds jokers
        // to the most frequent to get the best possible hand
        let mut counts: Vec<(Card, usize)> = counts.into_iter().collect();
        counts.sort_by_key(|(_, cnt)| Reverse(*cnt));
        counts[0].1 += jokers;

        // determines the best hand kind and used cards based on the game rules
        match (counts.get(0), counts.get(1)) {
            (Some((c1, cnt1)), _) if *cnt1 == 5 => HandKind::Five(*c1),
            (Some((c1, cnt1)), _) if *cnt1 == 4 => HandKind::Four(*c1),
            (Some((c1, cnt1)), Some((c2, cnt2))) if *cnt1 == 3 && *cnt2 == 2 => {
                HandKind::Full(*c1, *c2)
            }
            (Some((c1, cnt1)), _) if *cnt1 == 3 => HandKind::Three(*c1),
            (Some((c1, cnt1)), Some((c2, cnt2))) if *cnt1 == 2 && *cnt2 == 2 => {
                HandKind::TwoPair(*c1, *c2)
            }
            (Some((c1, cnt1)), _) if *cnt1 == 2 => HandKind::OnePair(*c1),
            (Some((c1, _)), _) => HandKind::High(*c1),
            _ => unreachable!(),
        }
    }

    fn strength(&self) -> usize {
        match self {
            HandKind::Five(_) => 7,
            HandKind::Four(_) => 6,
            HandKind::Full(_, _) => 5,
            HandKind::Three(_) => 4,
            HandKind::TwoPair(_, _) => 3,
            HandKind::OnePair(_) => 2,
            HandKind::High(_) => 1,
        }
    }
}

impl Card {
    fn strength(&self) -> usize {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Number(n) => *n,
            Self::Joker => 1,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Number(10),
            n => Self::Number(n.to_digit(10).unwrap() as usize),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day07").unwrap();

    let mut regular_hands: Vec<Hand> = input.lines().map(|l| l.into()).collect();
    regular_hands.sort();

    let mut joker_hands: Vec<Hand> = input
        .lines()
        .map(|l| {
            let hand = Hand::from(l);
            let cards = hand.cards.map(|c| match c {
                Card::Jack => Card::Joker,
                card => card,
            });
            let kind = HandKind::new(cards);
            Hand {
                cards,
                kind,
                bid: hand.bid,
            }
        })
        .collect();
    joker_hands.sort();

    let p1: usize = regular_hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum();
    println!("Part1: {p1}");

    let p2: usize = joker_hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum();
    println!("Part2: {p2}");
}
