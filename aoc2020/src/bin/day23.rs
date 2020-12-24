use std::fs;

fn main() {
    let labels = fs::read_to_string("input/day23").expect("failure opening input file");

    let mut cups = labels.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    println!("Part1: {}", p1(&cups));

    (10..1000000 + 1).for_each(|x| cups.push(x));
    println!("Part2: {}", p2(&cups));
}

fn game(cups: &[usize], moves: usize) -> Vec<usize> {
    let min = *cups.iter().min().unwrap();
    let max = *cups.iter().max().unwrap();

    // l[x] -> element to which 'x' is connected
    let ncups = cups.len();
    let mut list = vec![0; max + 1];
    (0..ncups).for_each(|i| list[cups[i]] = cups[(i + 1) % ncups]);

    let mut cur_cup = cups[0];
    for _ in 0..moves {
        let p1 = list[cur_cup];
        let p2 = list[p1];
        let p3 = list[p2];
        list[cur_cup] = list[p3];

        let mut dest_cup = if cur_cup > min { cur_cup - 1 } else { max };
        while [p1, p2, p3].contains(&dest_cup) || dest_cup < min || dest_cup > max {
            dest_cup = if dest_cup > min { dest_cup - 1 } else { max };
        }

        let tmp = list[dest_cup];
        list[dest_cup] = p1;
        list[p1] = p2;
        list[p2] = p3;
        list[p3] = tmp;

        cur_cup = list[cur_cup];
    }

    list
}

fn p1(cups: &[usize]) -> String {
    let list = game(&cups, 100);
    let mut p1 = String::new();
    let mut i = 1;
    while list[i] != 1 {
        p1.push_str(&list[i].to_string());
        i = list[i];
    }
    p1
}

fn p2(cups: &[usize]) -> usize {
    let list = game(&cups, 10000000);
    let x = list[1];
    let y = list[x];
    x * y
}
