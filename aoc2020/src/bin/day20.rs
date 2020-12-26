use std::collections::{HashMap, HashSet};
use std::fs;

const MONSTER: [&str; 3] = ["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "];

#[derive(Clone, Debug, Default)]
struct Tile {
    id: usize,
    m: Vec<Vec<char>>,
}

impl Tile {
    fn rotate_90(&mut self) {
        self.m = rotate_90_matrix(&self.m);
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
}

fn main() {
    let input = fs::read_to_string("input/day20").expect("failure opening input file");
    let mut tiles = HashMap::new();

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
    // println!("Part1: {}", corners.iter().product::<usize>());

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

        // check if match flipped
        if match_tile.m[0] != above_tile.m[above_tile.m.len() - 1] {
            for s in &mut match_tile.m {
                s.reverse()
            }
        }

        image[i][0] = match_tile;
    }

    // for all rows x cols, except first col
    for i in 0..size {
        for j in 1..size {
            let left_tile = image[i][j - 1].clone();
            let match_id = find_match(&left_tile, &borders_to_tiles, "right").unwrap();
            let mut match_tile = tiles[&match_id].clone();

            while find_match(&match_tile, &borders_to_tiles, "left") != Some(left_tile.id) {
                match_tile.rotate_90();
            }

            // check if match flipped
            let len = left_tile.m.len();
            if (0..len).any(|i| left_tile.m[i][len - 1] != match_tile.m[i][0]) {
                for i in 0..match_tile.m[0].len() / 2 {
                    match_tile.m[i][0] = match_tile.m[len - 1 - i][0];
                }
            }

            image[i][j] = match_tile;
        }
    }

    // build image by replacing tile IDs by char maps
    let width = image[0][0].m.len();
    let trimmed_width = width - 2;
    let mut final_image = vec![Vec::<char>::new(); trimmed_width * size];
    for i in 0..size {
        for j in 0..size {
            let tile = &image[i][j];
            for k in 1..width - 1 {
                final_image[i * trimmed_width + (k - 1)].extend(&tile.m[k][1..width - 1]);
            }
        }
    }

    let total_hashtags = final_image.iter().flatten().filter(|c| **c == '#').count();
    let monster_coords = MONSTER
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.chars().enumerate().filter(|&(_, c)| c == '#').map(move |(j, _)| (i as isize - 1, j as isize)))
        .collect::<HashSet<(isize, isize)>>();

    loop {
        let num_monsters = count_monsters(&final_image, &monster_coords);
        println!("Part2: {}", num_monsters);
        if num_monsters != 0 {
            let p2 = total_hashtags - num_monsters * monster_coords.len();
            println!("Part2: {}", p2);
            break;
        }
        final_image = rotate_90_matrix(&final_image);
    }

    println!("{:?} {:?}", final_image.len(), final_image[0].len());
}

fn count_monsters(image: &Vec<Vec<char>>, monster_coords: &HashSet<(isize, isize)>) -> usize {
    let hashtags = image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, &c)| c == '#').map(move |(j, _)| (i as isize, j as isize)))
        .collect::<HashSet<(isize, isize)>>();

    hashtags.iter().filter(|(i, j)| monster_coords.iter().map(|(di, dj)| (i + di, j + dj)).all(|pos| hashtags.contains(&pos))).count()
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

fn rotate_90_matrix<T: Clone + Default>(m: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (h, w) = (m.len(), m[0].len());
    let mut rot = vec![vec![T::default(); w]; h];
    for i in 0..h {
        for j in 0..w {
            rot[j][w - 1 - i] = m[i][j].clone();
        }
    }
    rot
}
