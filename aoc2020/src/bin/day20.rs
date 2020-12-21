use std::{fs, mem::swap};

type Tile = Vec<Vec<char>>;

fn main() {
    let input = fs::read_to_string("input/day20").expect("failure opening input file");

    let tiles = input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id = lines.next().unwrap();
            let id = id[5..id.len() - 1].parse().unwrap();
            let tile = lines.map(|line| line.chars().collect()).collect::<Tile>();
            (id, tile)
        })
        .collect::<Vec<(usize, Tile)>>();

    let size = f32::sqrt(tiles.len() as f32) as usize;
    println!("{:?}", size);

    let img = vec![vec![-1; size]; size];

    let mut test = vec![vec!['A', 'B', 'C'], vec!['D', 'E', 'F'], vec!['G', 'H', 'I']];

    rotate_90(&mut test);

    for line in test {
        println!("{:?}", line);
    }

    // create size x size matrix with -1 ids
    // search for piece that fits
    // if find one (possibly rotated/flipped), place it, repeat
    // if none fits, backtrack once
}

pub fn rotate_90(tile: &mut Tile) {
    tranpose(tile);
    flip_y(tile);
}

pub fn tranpose(tile: &mut Tile) {
    let mut tmp = vec![vec!['0'; tile[0].len()]; tile.len()];
    for i in 0..tile.len() {
        for j in 0..tile[i].len() {
            tmp[j][i] = tile[i][j];
        }
    }
    swap(tile, &mut tmp);
}

fn flip_y(tile: &mut Tile) {
    let size = tile[0].len();
    for i in 0..tile.len() {
        for j in 0..size / 2 {
            let tmp = tile[i][j];
            tile[i][j] = tile[i][size - 1 - j];
            tile[i][size - 1 - j] = tmp;
        }
    }
}

fn get_borders(tile: &mut Tile) {}
