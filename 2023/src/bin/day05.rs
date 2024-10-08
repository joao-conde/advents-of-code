struct MapRule {
    src: usize,
    dst: usize,
    len: usize,
}

fn main() {
    let input = std::fs::read_to_string("input/day05").unwrap();
    let (seeds, maps) = parse_input(&input);

    let seeds1: Vec<(usize, usize)> = seeds.iter().map(|s| (*s, *s)).collect();
    let p1 = find_lowest_location(&seeds1, &maps);
    println!("Part1: {p1}");

    let seeds2: Vec<(usize, usize)> = seeds.chunks(2).map(|s| (s[0], s[0] + s[1] - 1)).collect();
    let p2 = find_lowest_location(&seeds2, &maps);
    println!("Part2: {p2}");
}

fn find_lowest_location(seeds: &[(usize, usize)], maps: &[Vec<MapRule>]) -> usize {
    let mut location = 0;
    loop {
        let src = src_from_location(location, maps);
        let is_seed = seeds.iter().any(|(s, e)| (*s..=*e).contains(&src));
        if is_seed {
            return location;
        }
        location += 1;
    }
}

fn src_from_location(location: usize, maps: &[Vec<MapRule>]) -> usize {
    maps.iter()
        .rev()
        .fold(location, |dst, rules| src_from_dst(dst, rules))
}

fn src_from_dst(dst: usize, rules: &[MapRule]) -> usize {
    rules
        .iter()
        .find(|r| dst >= r.dst && dst <= r.dst + r.len)
        .map(|r| r.src + dst - r.dst)
        .unwrap_or(dst)
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Vec<MapRule>>) {
    let mut blocks = input.split("\n\n");

    let (_, seeds) = blocks.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<usize> = seeds.split_whitespace().flat_map(str::parse).collect();

    let mut maps: Vec<Vec<MapRule>> = vec![];
    for block in blocks {
        let (_, rules) = block.split_once(':').unwrap();
        let rules = rules
            .trim()
            .lines()
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

    (seeds, maps)
}
