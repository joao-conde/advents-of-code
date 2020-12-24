use std::collections::HashMap;
use std::fs;

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

    // top left corner
    for corner in &corners {
        let tile = &tiles[&corner];
        let top = get_top(tile);
        let left = get_left(tile);
        if borders_to_tiles[&top].len() == 1 && borders_to_tiles[&left].len() == 1 {
            img[0][0] = *corner;
            break;
        }
    }

    for i in 1..size {
        let left_id = img[0][i - 1];
        let left_tile = &tiles[&left_id];
        let border = get_right(&left_tile);
        let tiles = &borders_to_tiles[&border];
        let next = tiles.iter().filter(|tid| **tid != left_id).next().unwrap();
        img[0][i] = *next;
    }

    for x in img {
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

// fn rotate_90(tile: &mut Tile) {
//     tranpose(tile);
//     flip_y(tile);
// }

// fn tranpose(tile: &mut Tile) {
//     let mut tmp = vec![vec!['0'; tile[0].len()]; tile..len()];
//     for i in 0..tile..len() {
//         for j in 0..tile[i].len() {
//             tmp[j][i] = tile[i][j];
//         }
//     }
//     swap(&mut tile., &mut tmp);
// }

// fn flip_y(tile: &mut Tile) {
//     let size = tile[0].len();
//     for i in 0..tile..len() {
//         for j in 0..size / 2 {
//             tile[i].swap(j, size - 1 - j);
//         }
//     }
// }

fn get_edges(tile: &Tile) -> Vec<Vec<char>> {
    vec![get_top(&tile), get_right(&tile), get_bot(&tile), get_left(&tile)]
}

// fn find_matches(tile: &Tile, tiles: &Vec<Tile>) -> Vec<Tile> {
//     let tile_borders = get_edges(tile);
//     let mut matches = tiles
//         .clone()
//         .into_iter()
//         .filter(|t| tile.id != t.id)
//         .filter(|t| get_edges(t).iter().any(|border| tile_borders.contains(border)))
//         .collect::<Vec<Tile>>();

//     matches.dedup();
//     matches
// }

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
