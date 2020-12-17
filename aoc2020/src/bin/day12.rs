use std::fs;

fn main() {
    let input = fs::read_to_string("input/day12").expect("failure opening input file");
    let actions = input.lines().map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap())).collect::<Vec<(char, i32)>>();
    println!("Part1: {}", p1(&actions));
    println!("Part2: {}", p2(&actions));
}

fn p1(actions: &[(char, i32)]) -> i32 {
    let mut ship = (0, 0, 0.0f32);
    for (action, value) in actions {
        match action {
            'N' => ship.1 += value,
            'S' => ship.1 -= value,
            'E' => ship.0 += value,
            'W' => ship.0 -= value,
            'R' => ship.2 -= *value as f32,
            'L' => ship.2 += *value as f32,
            'F' => {
                ship.0 += ship.2.to_radians().cos().round() as i32 * value;
                ship.1 += ship.2.to_radians().sin().round() as i32 * value;
            }
            _ => panic!("invalid action"),
        }
    }
    ship.0.abs() + ship.1.abs()
}

fn p2(actions: &[(char, i32)]) -> i32 {
    let mut ship = (0, 0);
    let mut waypoint = (10, 1);
    for (action, value) in actions {
        match action {
            'N' => waypoint.1 += value,
            'S' => waypoint.1 -= value,
            'E' => waypoint.0 += value,
            'W' => waypoint.0 -= value,
            'R' => waypoint = rotate_point(waypoint.0, waypoint.1, -*value as f32),
            'L' => waypoint = rotate_point(waypoint.0, waypoint.1, *value as f32),
            'F' => {
                ship.0 += waypoint.0 * value;
                ship.1 += waypoint.1 * value;
            }
            _ => panic!("invalid action"),
        }
    }
    ship.0.abs() + ship.1.abs()
}

fn rotate_point(x: i32, y: i32, deg: f32) -> (i32, i32) {
    let sin = deg.to_radians().sin().round() as i32;
    let cos = deg.to_radians().cos().round() as i32;
    (x * cos - y * sin, x * sin + y * cos)
}
