type Pattern = Vec<Vec<char>>;

fn main() {
    let input = std::fs::read_to_string("input/day13").unwrap();

    let patterns: Vec<Pattern> = input
        .split("\n\n")
        .map(|l| l.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let p1: usize = patterns.iter().map(|p| summarize(p, 0)).sum();
    println!("Part1: {p1}");

    let p2: usize = patterns.iter().map(|p| summarize(p, 1)).sum();
    println!("Part2: {p2}");
}

fn summarize(pattern: &Pattern, smudges: usize) -> usize {
    if let Some(row) = search_row_mirror(pattern, smudges) {
        (row + 1) * 100
    } else if let Some(col) = search_col_mirror(pattern, smudges) {
        col + 1
    } else {
        unreachable!()
    }
}

fn search_row_mirror(pattern: &Pattern, smudges: usize) -> Option<usize> {
    (0..pattern.len() - 1)
        .find(|r| is_mirror(pattern, smudges, *r, 0, pattern.len() - 1, &take_row))
}

fn search_col_mirror(pattern: &Pattern, smudges: usize) -> Option<usize> {
    (0..pattern[0].len() - 1)
        .find(|c| is_mirror(pattern, smudges, *c, 0, pattern[0].len() - 1, &take_col))
}

fn is_mirror(
    pattern: &Pattern,
    mut smudges: usize,
    pivot: usize,
    lb: usize,
    ub: usize,
    take_line: &dyn Fn(&Pattern, usize) -> String,
) -> bool {
    let lb = lb as isize;
    let ub = ub as isize;
    let mut left_i = pivot as isize;
    let mut right_i = pivot as isize + 1;

    while left_i >= lb && right_i <= ub {
        let left_row = take_line(pattern, left_i as usize);
        let right_row = take_line(pattern, right_i as usize);

        if left_row != right_row {
            let diff = differences(&left_row, &right_row);
            if diff == 1 && smudges > 0 {
                smudges -= 1;
            } else {
                return false;
            }
        }

        left_i -= 1;
        right_i += 1;
    }

    left_i as usize != pivot && smudges == 0
}

fn take_row(pattern: &Pattern, row: usize) -> String {
    pattern[row].iter().collect()
}

fn take_col(pattern: &Pattern, col: usize) -> String {
    (0..pattern.len()).map(|i| pattern[i][col]).collect()
}

fn differences(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}
