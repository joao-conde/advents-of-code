use std::collections::{HashMap, HashSet};
use std::fs;
use std::mem::swap;

type Tile = Vec<Vec<char>>;

fn main() {
    let input = fs::read_to_string("input/day20").expect("failure opening input file");

    let mut tiles = HashMap::new();
    let mut borders_to_tiles = HashMap::new();
    for tile in input.split("\n\n") {
        let mut lines = tile.lines();
        let id = lines.next().unwrap();
        let id = id[5..id.len() - 1].parse::<usize>().unwrap();
        let tile = lines.map(|line| line.chars().collect()).collect::<Tile>();
        let edges = get_edges(&tile);
        tiles.insert(id, tile);
        for edge in edges {
            borders_to_tiles.entry(edge.clone()).or_insert(Vec::new()).push(id);
            borders_to_tiles.entry(edge.into_iter().rev().collect()).or_insert(Vec::new()).push(id);
        }
    }
    let count_map = borders_to_tiles.values().filter(|ids| ids.len() == 1).fold(HashMap::new(), |mut map, ids| {
        *map.entry(ids[0]).or_insert(0) += 1;
        map
    });
    let corners = count_map.iter().filter(|(_, v)| **v == 4).map(|(k, _)| *k).collect::<Vec<usize>>();
    // println!("Part1: {:?}", corners);

    let size = 3;
    let mut img = vec![vec![usize::MAX; size]; size];

    let mut used = HashSet::new();

    // top left corner
    for corner in &corners {
        let tile = &tiles[&corner];
        let top = get_top(tile);
        let left = get_left(tile);
        if borders_to_tiles[&top].len() == 1 && borders_to_tiles[&left].len() == 1 {
            img[0][0] = *corner;
            used.insert(*corner);
            break;
        }
    }

    // first row
    for j in 1..size {
        let left_id = img[0][j - 1];
        let left_tile = &tiles[&left_id];
        let border = get_right(&left_tile);
        let tiles = &borders_to_tiles[&border];
        let next = tiles.iter().filter(|tid| **tid != left_id).next().unwrap();
        img[0][j] = *next;
        used.insert(*next);
    }

    // first col
    for i in 1..size {
        let top_id = img[i - 1][0];
        let top_tile = &tiles[&top_id];
        let border = get_bot(&top_tile);
        let tiles = &borders_to_tiles[&border];
        let next = tiles.iter().filter(|tid| **tid != top_id).next().unwrap();
        img[i][0] = *next;
        used.insert(*next);
    }

    // other rows
    for i in 1..size {
        for j in 1..size {
            let top_id = img[i - 1][j];
            let left_id = img[i][j - 1];

            let top_tile = &tiles[&top_id];
            let left_tile = &tiles[&left_id];

            let bot_border = get_bot(&top_tile);
            let right_border = get_right(&left_tile);

            let tiles1 = &borders_to_tiles[&bot_border];
            let tiles2 = &borders_to_tiles[&right_border];

            let tiles1 = tiles1.clone();
            let tiles2 = tiles2.clone();
            let tot = vec![tiles1, tiles2];
            let next = tot.iter().flatten().filter(|tid| **tid != top_id && **tid != left_id && !used.contains(tid)).next().unwrap();
            img[i][j] = *next;
            used.insert(*next);
        }
    }

    for x in &img {
        println!("{:?}", x);
    }
    println!("\n\n");
    rotate_90(&mut img);
    rotate_90(&mut img);
    flip_y(&mut img);
    for x in &img {
        println!("{:?}", x);
    }
}

// fn rearrange(original: &Tile) -> Vec<Tile> {
//     let mut tile = original.clone();
//     let mut combinations = vec![original.clone()];
//     let mut flipped = tile.clone();
//     flip_y(&mut flipped);
//     combinations.push(flipped);
//     for _ in 0..3 {
//         rotate_90(&mut tile);
//         combinations.push(tile.clone());
//         let mut flipped = tile.clone();
//         flip_y(&mut flipped);
//         combinations.push(flipped);
//     }
//     combinations
// }

fn rotate_90<T: Clone + Copy + Default>(tile: &mut Vec<Vec<T>>) {
    tranpose(tile);
    flip_y(tile);
}

fn tranpose<T: Clone + Copy + Default>(tile: &mut Vec<Vec<T>>) {
    let mut tmp = vec![vec![T::default(); tile[0].len()]; tile.len()];
    for i in 0..tile.len() {
        for j in 0..tile[i].len() {
            tmp[j][i] = tile[i][j];
        }
    }
    swap(tile, &mut &mut tmp);
}

fn flip_y<T>(tile: &mut Vec<Vec<T>>) {
    let size = tile[0].len();
    for i in 0..tile.len() {
        for j in 0..size / 2 {
            tile[i].swap(j, size - 1 - j);
        }
    }
}

fn get_edges(tile: &Tile) -> Vec<Vec<char>> {
    vec![get_top(&tile), get_right(&tile), get_bot(&tile), get_left(&tile)]
}

fn strip_borders<T: Copy>(tile: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut new = vec![];
    for i in 1..tile.len() - 1 {
        let mut row = vec![];
        for j in 1..tile[i].len() - 1 {
            row.push(tile[i][j]);
        }
        new.push(row);
    }
    new
}

fn get_top(tile: &Tile) -> Vec<char> {
    tile[0][..].to_vec()
}

fn get_bot(tile: &Tile) -> Vec<char> {
    tile[tile.len() - 1][..].to_vec()
}

fn get_left(tile: &Tile) -> Vec<char> {
    (0..tile.len()).map(|i| tile[i][0]).collect()
}

fn get_right(tile: &Tile) -> Vec<char> {
    (0..tile.len()).map(|i| tile[i][tile[i].len() - 1]).collect()
}
