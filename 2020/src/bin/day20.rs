use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

// (0, 0) as the first # in second line
const MONSTER_DELTAS: [(isize, isize); 15] = [
    (-1, 18),
    (0, 0),
    (0, 5),
    (0, 6),
    (0, 11),
    (0, 12),
    (0, 17),
    (0, 18),
    (0, 19),
    (1, 1),
    (1, 4),
    (1, 7),
    (1, 10),
    (1, 13),
    (1, 16),
];

#[derive(Clone, Default)]
struct Tile {
    id: usize,
    char_map: Vec<Vec<char>>,
}

impl Tile {
    fn rotate_90(&mut self) {
        self.char_map = rotate_90_matrix(&self.char_map);
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
        self.char_map[0][..].to_vec()
    }

    fn get_right(&self) -> Vec<char> {
        (0..self.char_map.len())
            .map(|i| self.char_map[i][self.char_map[i].len() - 1])
            .collect()
    }

    fn get_bot(&self) -> Vec<char> {
        self.char_map[self.char_map.len() - 1][..].to_vec()
    }

    fn get_left(&self) -> Vec<char> {
        (0..self.char_map.len())
            .map(|i| self.char_map[i][0])
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("input/day20").expect("failure opening input file");

    let mut tiles = HashMap::new();
    let mut borders_to_tiles = HashMap::new();

    // save all tiles
    // build a map of border (and reverse) to tile IDs that have it
    for tile in input.split("\n\n") {
        let mut lines = tile.lines();
        let id = lines.next().unwrap();
        let id = id[5..id.len() - 1].parse::<usize>().unwrap();
        let char_map = lines
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let tile = Tile { id, char_map };
        tiles.insert(id, tile.clone());
        for edge in tile.get_edges() {
            borders_to_tiles
                .entry(edge.clone())
                .or_insert_with(Vec::new)
                .push(id);
            borders_to_tiles
                .entry(edge.into_iter().rev().collect())
                .or_insert_with(Vec::new)
                .push(id);
        }
    }

    let corners = find_corners(&borders_to_tiles);
    println!("Part1: {}", corners.iter().product::<usize>());

    let image = build_image(&tiles, &borders_to_tiles, corners[0]);
    println!("Part2: {}", compute_roughness(image));
}

fn find_corners(borders_to_tiles: &HashMap<Vec<char>, Vec<usize>>) -> Vec<usize> {
    let unique_edge_count = borders_to_tiles
        .values()
        .filter(|tids| tids.len() == 1)
        .fold(HashMap::new(), |mut map, tids| {
            *map.entry(tids[0]).or_insert(0) += 1;
            map
        });
    unique_edge_count
        .iter()
        .filter(|(_, v)| **v == 4)
        .map(|(k, _)| *k)
        .collect()
}

fn build_image(
    tiles: &HashMap<usize, Tile>,
    borders_to_tiles: &HashMap<Vec<char>, Vec<usize>>,
    top_left_corner: usize,
) -> Vec<Vec<char>> {
    let size = f32::sqrt(tiles.len() as f32) as usize;
    let mut image = vec![vec![Tile::default(); size]; size];

    // place top-left
    let mut corner = tiles[&top_left_corner].clone();
    while find_match(&corner, &borders_to_tiles, "top").is_some()
        || find_match(&corner, &borders_to_tiles, "left").is_some()
    {
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
        if match_tile.char_map[0] != above_tile.char_map[above_tile.char_map.len() - 1] {
            for s in &mut match_tile.char_map {
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
            let len = left_tile.char_map.len();
            if (0..len).any(|i| left_tile.char_map[i][len - 1] != match_tile.char_map[i][0]) {
                for i in 0..len / 2 {
                    match_tile.char_map.swap(i, len - 1 - i)
                }
            }

            image[i][j] = match_tile;
        }
    }

    // build image by replacing tile IDs by char maps
    let width = image[0][0].char_map.len();
    let trimmed_width = width - 2;
    let mut final_image = vec![Vec::new(); trimmed_width * size];
    for i in 0..size {
        for j in 0..size {
            let tile = &image[i][j];
            for k in 1..width - 1 {
                final_image[i * trimmed_width + (k - 1)].extend(&tile.char_map[k][1..width - 1]);
            }
        }
    }

    final_image
}

fn compute_roughness(mut image: Vec<Vec<char>>) -> usize {
    let total_hashtags = image.iter().flatten().filter(|&&c| c == '#').count();
    let monster_coords = HashSet::from_iter(MONSTER_DELTAS.iter().cloned());
    for r in 0..8 {
        let num_monsters = count_monsters(&image, &monster_coords);
        if num_monsters != 0 {
            return total_hashtags - num_monsters * monster_coords.len();
        }
        image = rotate_90_matrix(&image);
        if r == 3 {
            image = flip_y(&image);
        }
    }
    unreachable!()
}

fn count_monsters(image: &[Vec<char>], monster_coords: &HashSet<(isize, isize)>) -> usize {
    let hashtags = image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == '#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .collect::<HashSet<(isize, isize)>>();

    hashtags
        .iter()
        .filter(|(i, j)| {
            monster_coords
                .iter()
                .map(|(di, dj)| (i + di, j + dj))
                .all(|pos| hashtags.contains(&pos))
        })
        .count()
}

fn find_match(
    tile: &Tile,
    borders_to_tiles: &HashMap<Vec<char>, Vec<usize>>,
    dir: &str,
) -> Option<usize> {
    match dir {
        "top" => &borders_to_tiles[&tile.char_map[0]],
        "right" => &borders_to_tiles[&tile.get_right()],
        "bot" => &borders_to_tiles[&tile.char_map[tile.char_map.len() - 1]],
        "left" => &borders_to_tiles[&tile.get_left()],
        _ => unreachable!(),
    }
    .iter()
    .find(|&&id| id != tile.id)
    .copied()
}

fn rotate_90_matrix<T: Clone + Default>(m: &[Vec<T>]) -> Vec<Vec<T>> {
    let (h, w) = (m.len(), m[0].len());
    let mut rot = vec![vec![T::default(); w]; h];

    #[allow(clippy::needless_range_loop)]
    for i in 0..h {
        for j in 0..w {
            rot[j][w - 1 - i] = m[i][j].clone();
        }
    }

    rot
}

fn flip_y<T: Clone>(tile: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut tile = tile.to_vec();
    let size = tile[0].len();
    for row in &mut tile {
        for j in 0..size / 2 {
            row.swap(j, size - 1 - j);
        }
    }
    tile
}
