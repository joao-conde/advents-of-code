use std::collections::VecDeque;

fn main() {
    let labels = "963275481";

    let mut cups = labels.chars().map(|c| c.to_digit(10).unwrap()).collect::<VecDeque<u32>>();
    let min = *cups.iter().min().unwrap();
    let max = *cups.iter().max().unwrap();
    for _ in 0..100 {
        let cup = cups[0];
        let pickup = cups.drain(1..4).collect::<VecDeque<u32>>();
        let mut dest_cup = cup - 1;
        if dest_cup < min {
            dest_cup = max;
        }
        while !cups.contains(&dest_cup) {
            dest_cup -= 1;
            if dest_cup < min {
                dest_cup = max;
            }
        }

        while let Some(front) = cups.pop_front() {
            if front != dest_cup {
                cups.push_back(front);
            } else {
                cups.push_front(front);
                break;
            }
        }

        for el in pickup.iter().rev() {
            cups.insert(1, *el);
        }

        let cup_i = cups.iter().position(|x| *x == cup).unwrap();
        let next = cups[(cup_i + 1) % cups.len()];
        while let Some(front) = cups.pop_front() {
            if front != next {
                cups.push_back(front);
            } else {
                cups.push_front(front);
                break;
            }
        }
    }

    while let Some(front) = cups.pop_front() {
        if front == 1 {
            break;
        } else {
            cups.push_back(front);
        }
    }
    println!("{}", cups.iter().map(|d| d.to_string()).collect::<String>());
}
