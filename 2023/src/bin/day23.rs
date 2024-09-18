use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

fn main() {
    let input = std::fs::read_to_string("input/day23").unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let longest = longest_path(&map);
    dbg!(longest);
}

fn build_graph(map: &Vec<Vec<char>>) -> HashMap<Point, HashMap<Point, usize>> {
    let start = (0, 1);
    let end = (map.len() - 1, map[0].len() - 2);

    let mut graph: HashMap<Point, HashMap<Point, usize>> = HashMap::new();

    // find nodes which are the intersection points where we
    // have a choice other than the previous or next '.'
    let mut nodes = vec![start, end];
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                continue;
            }

            let neighbors = neighbors(map, i, j).iter().count();
            if neighbors > 2 {
                nodes.push((i, j));
            }
        }
    }

    graph.insert(start, HashMap::new());
    graph.insert(end, HashMap::new());

    for (si, sj) in &nodes {
        let si = *si;
        let sj = *sj;

        let mut seen = HashSet::new();
        let mut stack = vec![(si, sj, 0)];
        while let Some((i, j, length)) = stack.pop() {
            if (si, sj) != (i, j) && nodes.contains(&(i, j)) {
                graph
                    .entry((si, sj))
                    .or_insert_with(HashMap::new)
                    .insert((i, j), length);
                continue;
            }

            let neighbors = match map[i][j] {
                '>' => vec![(i, j + 1)],
                '<' => vec![(i, j - 1)],
                '^' => vec![(i - 1, j)],
                'v' => vec![(i + 1, j)],
                '.' => neighbors(map, i, j),
                _ => unreachable!(),
            };

            for (ni, nj) in neighbors {
                if !seen.contains(&(ni, nj)) {
                    stack.push((ni, nj, length + 1));
                    seen.insert((ni, nj));
                }
            }
        }
    }

    graph
}

fn longest_path(map: &Vec<Vec<char>>) -> usize {
    let graph = build_graph(map);

    let mut longest = 0;

    let mut stack = vec![(0, 1, 0)];
    while let Some((i, j, length)) = stack.pop() {
        longest = usize::max(longest, length);

        let edges = graph.get(&(i, j)).unwrap();
        for ((ni, nj), n) in edges {
            stack.push((*ni, *nj, n + length));
        }
    }

    longest
}

fn neighbors(map: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<Point> {
    let nrows = map.len() as isize;
    let ncols = map[0].len() as isize;

    let i = i as isize;
    let j = j as isize;

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|(di, dj)| (i + di, j + dj))
        .filter(|&(i, j)| i >= 0 && i < nrows && j >= 0 && j < ncols)
        .map(|(i, j)| (i as usize, j as usize))
        .filter(|&(i, j)| map[i][j] != '#')
        .collect()
}
