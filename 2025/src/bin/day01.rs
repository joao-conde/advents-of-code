const DIAL_START: isize = 50;
const DIAL_MAX: isize = 99;

struct Rotation {
    direction: Direction,
    distance: usize,
}

enum Direction {
    Left,
    Right,
}

fn main() {
    let input = std::fs::read_to_string("input/day01").unwrap();

    let rotations: Vec<Rotation> = input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let direction = if direction == "L" {
                Direction::Left
            } else {
                Direction::Right
            };
            let distance = distance.parse().unwrap();
            Rotation {
                direction,
                distance,
            }
        })
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut dial = DIAL_START;
    for rotation in rotations {
        let distance = match rotation.direction {
            Direction::Left => -(rotation.distance as isize),
            Direction::Right => rotation.distance as isize,
        };

        let total = dial + distance;
        let spins = total.div_euclid(DIAL_MAX + 1).abs();
        dial = total.rem_euclid(DIAL_MAX + 1);

        p2 += spins;
        if dial == 0 {
            p1 += 1;
        }
    }

    println!("Part1: {p1}");
    println!("Part2: {p2}");
}
