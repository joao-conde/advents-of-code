fn main() {
    let labels = "389125467";
    let cups = labels.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    let min = *cups.iter().min().unwrap();
    let max = *cups.iter().max().unwrap();

    // l[x] -> element to which 'x' is is connected
    let mut list = vec![0; max + 1];
    for i in 0..cups.len() {
        list[cups[i]] = cups[(i + 1) % cups.len()];
    }

    let mut cur_cup = cups[0];
    for _ in 0..100 {
        let p1 = list[cur_cup];
        let p2 = list[p1];
        let p3 = list[p2];
        list[cur_cup] = list[p3];

        let mut dest_cup = cur_cup - 1;
        if dest_cup < min {
            dest_cup = max;
        }
        while [p1, p2, p3].contains(&dest_cup) || !cups.contains(&dest_cup) {
            dest_cup -= 1;
            if dest_cup < min {
                dest_cup = max;
            }
        }

        let tmp = list[dest_cup];
        list[dest_cup] = p1;
        list[p1] = p2;
        list[p2] = p3;
        list[p3] = tmp;

        cur_cup = list[cur_cup];
    }

    let mut p1 = String::new();
    let mut i = 1;
    while list[i] != 1 {
        p1.push_str(&list[i].to_string());
        i = list[i];
    }
    println!("Part1: {}", p1);
}
