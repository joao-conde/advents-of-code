use std::fs;

fn main() {
    let input = fs::read_to_string("input/day12").expect("failure opening input file");
    let actions = input.split('\n').map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap())).collect::<Vec<(char, i32)>>();
    println!("Part1: {}", p1(&actions));
    println!("Part2: {}", p2(&actions));
}

fn p1(actions: &[(char, i32)]) -> i32 {
    let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let (mut x, mut y, mut cur_dir) = (0, 0, 0);
    for (action, value) in actions {
        match action {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'F' => {
                let dir = dirs[cur_dir as usize];
                x += dir.0 * value;
                y += dir.1 * value;
            }
            'R' => {
                cur_dir += value / 90;
                cur_dir %= 4;
            }
            'L' => {
                cur_dir -= value / 90;
                cur_dir = (cur_dir + 4) % 4;
            }
            _ => panic!("invalid action"),
        }
    }
    x.abs() + y.abs()
}

fn p2(actions: &[(char, i32)]) -> i32 {
    let (mut wx, mut wy) = (10, 1);
    let (mut x, mut y) = (0, 0);
    for (action, value) in actions {
        match action {
            'N' => wy += value,
            'S' => wy -= value,
            'E' => wx += value,
            'W' => wx -= value,
            'F' => {
                x += wx * value;
                y += wy * value;
            }
            'R' => {
                let (x, y) = rotate_clockwise(wx, wy, *value);
                wx = x;
                wy = y;
            }
            'L' => {
                let (x, y) = rotate_clockwise(wx, wy, 360 - *value);
                wx = x;
                wy = y;
            }
            _ => panic!("invalid action"),
        }
    }
    x.abs() + y.abs()
}

fn rotate_clockwise(mut x: i32, mut y: i32, deg: i32) -> (i32, i32) {
    (0..deg / 90).for_each(|_| {
        let tmp = x;
        x = y;
        y = -tmp;
    });
    (x, y)
}
