struct MapRule {
    src: usize,
    dst: usize,
    len: usize,
}

fn main() {
    let input = std::fs::read_to_string("input/day05").unwrap();

    let mut blocks = input.split("\n\n");

    let seeds = blocks.next().unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds: Vec<usize> = seeds.split(' ').flat_map(str::parse).collect();

    let mut maps: Vec<Vec<MapRule>> = vec![];
    for block in blocks {
        let (_, rules) = block.split_once(':').unwrap();

        let rules = rules.trim().split('\n');
        let rules = rules
            .map(|rule| {
                let mut rule = rule.splitn(3, ' ');
                let dst = rule.next().unwrap().parse().unwrap();
                let src = rule.next().unwrap().parse().unwrap();
                let len = rule.next().unwrap().parse().unwrap();
                MapRule { dst, src, len }
            })
            .collect();
        maps.push(rules);
    }

    let p1 = seeds
        .iter()
        .map(|seed| find_location(*seed, &maps))
        .min()
        .unwrap();
    println!("Part1: {p1}");
}

fn find_location(seed: usize, maps: &[Vec<MapRule>]) -> usize {
    maps.iter().fold(seed, |value, rule| map_value(value, rule))
}

fn map_value(value: usize, rules: &[MapRule]) -> usize {
    let matches: Vec<usize> = rules
        .iter()
        .filter(|r| value >= r.src && value <= r.src + r.len)
        .map(|r| r.dst + value - r.src)
        .collect();
    matches.into_iter().next().unwrap_or(value)
}
