use std::cmp::Ordering;
// use std::sync::OnceLock;
use std::{cmp::Reverse, collections::HashMap};

// static MEM: OnceLock<HashMap<usize, usize>> = OnceLock::new();

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    kind: HandKind,
}

#[derive(Debug)]
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.strength().cmp(&other.strength()))
    }
}

impl Hand {
    fn cmp_strength(h1: &Hand, h2: &Hand) -> Ordering {
        match (h1.kind.strength(), h2.kind.strength()) {
            (s1, s2) if s1 < s2 => Ordering::Less,
            (s1, s2) if s1 > s2 => Ordering::Greater,
            _ => {
                let (c1, c2) = h1
                    .cards
                    .iter()
                    .zip(h2.cards.iter())
                    .filter(|(c1, c2)| c1 != c2)
                    .nth(0)
                    .unwrap();
                c1.partial_cmp(c2).unwrap()
            }
        }
    }
}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards.chars().map(|c| c.into()).collect::<Vec<Card>>();
        let cards = cards.try_into().unwrap();
        let bid = bid.parse().unwrap();
        let kind = HandKind::new(cards);
        Hand { cards, bid, kind }
    }
}

impl HandKind {
    fn new(cards: [Card; 5]) -> HandKind {
        let mut counts: HashMap<Card, usize> = cards.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });

        let jokers = counts.remove(&Card::Joker).unwrap_or(0);

        let mut counts: Vec<(Card, usize)> = counts.into_iter().collect();
        counts.sort_by_key(|(_, cnt)| Reverse(*cnt));

        if counts.get(0).is_some() {
            counts[0].1 += jokers
        }

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
            (None, _) => HandKind::Five(Card::Ace),
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

fn main() {
    let input = std::fs::read_to_string("input/day07").unwrap();

    let mut hands: Vec<Hand> = input.lines().map(|l| l.into()).collect();
    hands.sort_by(Hand::cmp_strength);

    let p1: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    println!("Part1: {p1}");

    let mut hands: Vec<Hand> = hands
        .into_iter()
        .map(|h| {
            let cards = h.cards.map(|c| match c {
                Card::Jack => Card::Joker,
                card => card,
            });
            let kind = HandKind::new(cards);

            Hand {
                cards,
                kind,
                bid: h.bid,
            }
        })
        .collect();
    hands.sort_by(Hand::cmp_strength);

    let p2: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    println!("Part2: {p2}");
}
