use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Default)]
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
        vec![self.get_top(), self.get_right(), self.get_bot(), self.get_left()]
    }

    fn get_top(&self) -> Vec<char> {
        self.m[0][..].to_vec()
    }

    fn get_right(&self) -> Vec<char> {
        (0..self.m.len()).map(|i| self.m[i][self.m[i].len() - 1]).collect()
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
    let mut tiles = HashMap::new();
    ////////////////////part1
    let mut borders_to_tiles = HashMap::new();
    for tile in input.split("\n\n") {
        let mut lines = tile.lines();
        let id = lines.next().unwrap();
        let id = id[5..id.len() - 1].parse::<usize>().unwrap();
        let m = lines.map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

        let tile = Tile { id, m };
        tiles.insert(id, tile.clone());
        for edge in tile.get_edges() {
            borders_to_tiles.entry(edge.clone()).or_insert(Vec::new()).push(id);
            borders_to_tiles.entry(edge.into_iter().rev().collect()).or_insert(Vec::new()).push(id);
        }
    }
    let unique_edge_count = borders_to_tiles.values().filter(|ids| ids.len() == 1).fold(HashMap::new(), |mut map, ids| {
        *map.entry(ids[0]).or_insert(0) += 1;
        map
    });
    let corners = unique_edge_count.iter().filter(|(_, v)| **v == 4).map(|(k, _)| *k).collect::<Vec<usize>>();
    println!("Part1: {:?} {}", corners, corners.iter().product::<usize>());
    ////////////////////

    let size = f32::sqrt(tiles.len() as f32) as usize;
    let mut image = vec![vec![Tile::default(); size]; size];

    // place top-left
    let mut corner = tiles[&corners[0]].clone();
    while find_match(&corner, &borders_to_tiles, "top").is_some() || find_match(&corner, &borders_to_tiles, "left").is_some() {
        corner.rotate_90();
    }
    image[0][0] = corner;

    // remaining first col
    for i in 1..size {
        let above_tile = image[i - 1][0].clone();

        let match_id = find_match(&above_tile, &borders_to_tiles, "bot").unwrap();
        let mut match_tile = tiles[&match_id].clone();

        while find_match(&match_tile, &borders_to_tiles, "top") != Some(above_tile.id) {
            match_tile.rotate_90();
        }

        if match_tile.m[0] != above_tile.m[above_tile.m.len() - 1] {
            for s in &mut match_tile.m {
                s.reverse()
            }
        }

        image[i][0] = match_tile;
    }

    for x in image.iter().map(|x| x.iter().map(|y| y.id).collect()).collect::<Vec<Vec<usize>>>() {
        println!("{:?}", x);
    }
}

fn find_match(tile: &Tile, borders_to_tiles: &HashMap<Vec<char>, Vec<usize>>, dir: &str) -> Option<usize> {
    match dir {
        "top" => &borders_to_tiles[&tile.m[0]],
        "right" => &borders_to_tiles[&tile.get_right()],
        "bot" => &borders_to_tiles[&tile.m[tile.m.len() - 1]],
        "left" => &borders_to_tiles[&tile.get_left()],
        _ => unreachable!(),
    }
    .iter()
    .find(|&&id| id != tile.id)
    .copied()
}
