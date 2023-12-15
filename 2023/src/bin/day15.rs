use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/day15").unwrap();
    let steps: Vec<&str> = input.split(',').collect();

    let (boxes, lengths) = fill_boxes(&steps);

    let p1: usize = steps.iter().map(|s| ha256(s)).sum();
    println!("Part1: {p1}");

    let p2: usize = lengths
        .keys()
        .map(|label| focus_power(label, &boxes, &lengths))
        .sum();
    println!("Part2: {p2}");
}

fn fill_boxes<'a>(steps: &[&'a str]) -> (Vec<Vec<&'a str>>, HashMap<&'a str, usize>) {
    let mut boxes: Vec<Vec<&str>> = vec![vec![]; 256];
    let mut lengths: HashMap<&str, usize> = HashMap::new();

    for step in steps {
        if let Some((label, length)) = step.split_once('=') {
            let length: usize = length.parse().unwrap();
            lengths.insert(label, length);

            let box_i = ha256(label);
            if !boxes[box_i].contains(&label) {
                boxes[box_i].push(label);
            }
        } else if let Some((label, _)) = step.split_once('-') {
            let box_i = ha256(label);
            boxes[box_i].retain(|l| l != &label);
            lengths.remove(label);
        }
    }

    (boxes, lengths)
}

fn focus_power(label: &str, boxes: &[Vec<&str>], focal_lengths: &HashMap<&str, usize>) -> usize {
    let box_i = ha256(label);
    let slot = boxes[box_i].iter().position(|l| l == &label).unwrap();
    let focal_length = focal_lengths[label];
    (box_i + 1) * (slot + 1) * focal_length
}

fn ha256(value: &str) -> usize {
    value
        .chars()
        .fold(0, |hash, c| ((hash + c as usize) * 17) % 256)
}
