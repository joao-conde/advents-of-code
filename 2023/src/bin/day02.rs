struct Game {
    id: usize,
    max_reds: usize,
    max_greens: usize,
    max_blues: usize,
}

impl Game {
    pub fn possible(&self, reds: usize, greens: usize, blues: usize) -> bool {
        self.max_reds <= reds && self.max_greens <= greens && self.max_blues <= blues
    }

    pub fn power(&self) -> usize {
        self.max_reds * self.max_greens * self.max_blues
    }
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (meta, sets) = line.split_once(':').unwrap();

        let (_, id) = meta.split_once(' ').unwrap();
        let id: usize = id.parse().unwrap();

        let sets = sets.trim().split(';').map(|s| s.split(',').map(str::trim));

        let (mut max_reds, mut max_greens, mut max_blues) = (0, 0, 0);
        for set in sets {
            for draw in set {
                let (number, color) = draw.split_once(' ').unwrap();
                let number: usize = number.parse().unwrap();
                match color {
                    "red" => max_reds = number.max(max_reds),
                    "green" => max_greens = number.max(max_greens),
                    "blue" => max_blues = number.max(max_blues),
                    _ => unreachable!(),
                }
            }
        }

        Game {
            id,
            max_reds,
            max_greens,
            max_blues,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day02").unwrap();
    let games: Vec<Game> = input.lines().map(|l| l.into()).collect();

    let p1: usize = games
        .iter()
        .filter(|&g| g.possible(12, 13, 14))
        .map(|g| g.id)
        .sum();
    let p2: usize = games.iter().map(Game::power).sum();
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}
