use std::collections::{HashMap, HashSet};
use std::fs;

struct Tile {
    id: usize,
    m: Vec<Vec<char>>,
}

impl Tile {
    fn rotate_90(&mut self) {
        let (h, w) = (self.m.len(), self.m[0].len());
        let mut rot = vec![vec!['\0'; w]; h];
        for i in 0..self.m.len() {
            for j in 0..self.m[i].len() {
                rot[j][self.m[0].len() - 1 - i] = self.m[i][j];
            }
        }
        self.m = rot
    }

    fn get_edges(&self) -> Vec<Vec<char>> {
        vec![
            self.get_top(),
            self.get_right(),
            self.get_bot(),
            self.get_left(),
        ]
    }

    fn get_top(&self) -> Vec<char> {
        self.m[0][..].to_vec()
    }

    fn get_right(&self) -> Vec<char> {
        (0..self.m.len())
            .map(|i| self.m[i][self.m[i].len() - 1])
            .collect()
    }

    fn get_bot(&self) -> Vec<char> {
        self.m[self.m.len() - 1][..].to_vec()
    }

    fn get_left(&self) -> Vec<char> {
        (0..self.m.len()).map(|i| self.m[i][0]).collect()
    }

    fn strip_borders(&mut self) {
        let mut new = vec![];
        for i in 1..self.m.len() - 1 {
            let mut row = vec![];
            for j in 1..self.m[i].len() - 1 {
                row.push(self.m[i][j]);
            }
            new.push(row);
        }
        self.m = new;
    }
}

fn main() {
    let input = fs::read_to_string("input/day20").expect("failure opening input file");

    let mut borders_to_tiles = HashMap::new();
    for tile in input.split("\n\n") {
        let mut lines = tile.lines();
        let id = lines.next().unwrap();
        let id = id[5..id.len() - 1].parse::<usize>().unwrap();
        let m = lines
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let tile = Tile { id, m };
        for edge in tile.get_edges() {
            borders_to_tiles
                .entry(edge.clone())
                .or_insert(Vec::new())
                .push(id);
            borders_to_tiles
                .entry(edge.into_iter().rev().collect())
                .or_insert(Vec::new())
                .push(id);
        }
    }
    let count_map = borders_to_tiles.values().filter(|ids| ids.len() == 1).fold(
        HashMap::new(),
        |mut map, ids| {
            *map.entry(ids[0]).or_insert(0) += 1;
            map
        },
    );
    println!("{:?}", count_map);
    let corners = count_map
        .iter()
        .filter(|(_, v)| **v == 4)
        .map(|(k, _)| *k)
        .collect::<Vec<usize>>();
    println!("Part1: {:?} {}", corners, corners.iter().product::<usize>());
}
