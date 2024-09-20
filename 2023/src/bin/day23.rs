use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

fn main() {
    let input = std::fs::read_to_string("input/day23").unwrap();
    let slippery_map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let p1 = longest_path(&slippery_map);
    println!("Part1: {p1}");

    let normal_map = slippery_map
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| if *c == '#' { '#' } else { '.' })
                .collect()
        })
        .collect();
    let p2 = longest_path(&normal_map);
    println!("Part2: {p2}");
}

fn longest_path(map: &Vec<Vec<char>>) -> usize {
    // build our graph and statically define start
    // and end points as defined in the problem
    let start = (0, 1);
    let end = (map.len() - 1, map[0].len() - 2);
    let graph = build_graph(map, start, end);

    // do a DFS where each state has the current
    // position, path length and nodes visited
    let mut longest = 0;
    let mut stack = vec![(start, 0, HashSet::new())];
    while let Some((cur, length, path)) = stack.pop() {
        // if we reached an end state record check and upate
        // the longest length we have seen
        if cur == end {
            longest = usize::max(longest, length);
            continue;
        }

        // for each of the node's neighbors mark them to explore
        // updating the current path and path length
        let neighbors = graph.get(&cur).unwrap();
        for (neighbor, cost) in neighbors {
            if !path.contains(neighbor) {
                let mut next_path = path.clone();
                next_path.insert(cur);
                stack.push((*neighbor, cost + length, next_path));
            }
        }
    }

    longest
}

// Builds the problem graph and performs edge contraction.
fn build_graph(
    map: &Vec<Vec<char>>,
    start: Point,
    end: Point,
) -> HashMap<Point, HashMap<Point, usize>> {
    let mut graph: HashMap<Point, HashMap<Point, usize>> = HashMap::new();
    graph.insert(start, HashMap::new());
    graph.insert(end, HashMap::new());

    // find nodes which are the intersection points where we
    // have a choice other than going back or forwards
    let mut nodes = vec![start, end];
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                continue;
            }

            let neighbors = neighbors(map, i, j).len();
            if neighbors > 2 {
                nodes.push((i, j));
            }
        }
    }

    // for each node fill out its adjacency list
    // of neighbors and distance to them
    for (si, sj) in &nodes {
        let si = *si;
        let sj = *sj;

        let mut seen = HashSet::new();
        let mut stack = vec![(si, sj, 0)];
        while let Some((i, j, length)) = stack.pop() {
            // if we reached another node add it as a neighbor
            // with the computed distance and move on to the next
            if (si, sj) != (i, j) && nodes.contains(&(i, j)) {
                graph.entry((si, sj)).or_default().insert((i, j), length);
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

            // for each of the node's neighbors that was
            // not already seen we add to be explored
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
