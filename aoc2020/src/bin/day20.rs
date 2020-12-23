use std::{fs, mem::swap};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Tile {
    pub id: usize,
    pub img: Vec<Vec<char>>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    let input = fs::read_to_string("input/day20").expect("failure opening input file");
    let tiles = input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id = lines.next().unwrap();
            let id = id[5..id.len() - 1].parse().unwrap();
            let tile = lines.map(|line| line.chars().collect()).collect();
            Tile { id: id, img: tile }
        })
        .collect::<Vec<Tile>>();

    let tiles = tiles.iter().map(|tile| rearrange(tile)).flatten().collect::<Vec<Tile>>();
    
   
    let mut corners = tiles.iter().filter(|tile| find_matches(tile, &tiles).len() == 2).map(|tile| tile.id).collect::<Vec<usize>>();
    corners.dedup();
    println!("Part1: {} {:?}", corners.iter().product::<usize>());

    // let size = f32::sqrt(tiles.len() as f32) as usize;
}

fn rearrange(original: &Tile) -> Vec<Tile> {
    let mut tile = original.clone();
    let mut combinations = vec![original.clone()];
    let mut flipped = tile.clone();
    flip_y(&mut flipped);
    combinations.push(flipped);
    for _ in 0..3 {
        rotate_90(&mut tile);
        combinations.push(tile.clone());
        let mut flipped = tile.clone();
        flip_y(&mut flipped);
        combinations.push(flipped);
    }
    combinations
}

fn rotate_90(tile: &mut Tile) {
    tranpose(tile);
    flip_y(tile);
}

fn tranpose(tile: &mut Tile) {
    let mut tmp = vec![vec!['0'; tile.img[0].len()]; tile.img.len()];
    for i in 0..tile.img.len() {
        for j in 0..tile.img[i].len() {
            tmp[j][i] = tile.img[i][j];
        }
    }
    swap(&mut tile.img, &mut tmp);
}

fn flip_y(tile: &mut Tile) {
    let size = tile.img[0].len();
    for i in 0..tile.img.len() {
        for j in 0..size / 2 {
            tile.img[i].swap(j, size - 1 - j);
        }
    }
}

fn get_edges(tile: &Tile) -> Vec<Vec<char>> {
    let mut borders = vec![];
    borders.push(tile.img[0][..].to_vec());
    borders.push(tile.img[tile.img.len() - 1][..].to_vec());
    let first_col = (0..tile.img.len()).map(|i| tile.img[i][0]).collect();
    let last_col = (0..tile.img.len()).map(|i| tile.img[i][tile.img[i].len() - 1]).collect();
    borders.push(first_col);
    borders.push(last_col);
    borders
}

fn find_matches(tile: &Tile, tiles: &Vec<Tile>) -> Vec<Tile> {
    let tile_borders = get_edges(tile);
    let mut matches = tiles
        .clone()
        .into_iter()
        .filter(|t| tile.id != t.id)
        .filter(|t| get_edges(t).iter().any(|border| tile_borders.contains(border)))
        .collect::<Vec<Tile>>();

    matches.dedup();
    matches
}
