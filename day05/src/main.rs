use std::collections::HashMap;

fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one actual:  {}", part_one("input"));
}

fn part_one(filename: &str) -> usize {
    let (seeds, maps) = parse_file(filename);

    return seeds
        .iter()
        .map(|s| trace_down(*s, "seed", &maps).1)
        .min()
        .unwrap();
}

fn trace_down<'a>(
    value: usize,
    key: &'a str,
    maps: &'a HashMap<String, CategoryMap>,
) -> (&'a str, usize) {
    match maps.get(key) {
        None => (key, value),
        Some(m) => trace_down(m.translate(value), &m.output_key, maps),
    }
}

#[derive(Debug)]
struct CategoryMap {
    input_key: String,
    output_key: String,
    ranges: Vec<RangeConversion>,
}

impl CategoryMap {
    fn translate(&self, input: usize) -> usize {
        match self.ranges.iter().find_map(|range| range.translate(input)) {
            None => input,
            Some(v) => v,
        }
    }
}

#[derive(Debug)]
struct RangeConversion {
    source: usize,
    dest: usize,
    length: usize,
}

impl RangeConversion {
    fn translate(&self, input: usize) -> Option<usize> {
        let distance = input.checked_sub(self.source)?;

        if distance > self.length {
            return None;
        }

        return Some(self.dest + distance);
    }
}

fn parse_file(filename: &str) -> (Vec<usize>, HashMap<String, CategoryMap>) {
    let text = std::fs::read_to_string(filename).unwrap();
    let mut entries = text.split("\n\n");

    let seeds: Vec<usize> = entries
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect();

    let maps: HashMap<String, CategoryMap> =
        entries
            .map(CategoryMap::from)
            .fold(HashMap::new(), |mut acc, map| {
                acc.insert(String::from(map.input_key.clone()), map);

                return acc;
            });

    return (seeds, maps);
}

impl From<&str> for CategoryMap {
    fn from(s: &str) -> Self {
        let mut lines = s.trim().lines();
        let keys = lines.next().unwrap();
        let keys = keys.split(" ").next().unwrap();
        let mut keys = keys.split("-to-");
        let input_key = keys.next().unwrap().into();
        let output_key = keys.next().unwrap().into();

        let ranges: Vec<_> = lines.map(RangeConversion::from).collect();

        return Self {
            input_key,
            output_key,
            ranges,
        };
    }
}

impl From<&str> for RangeConversion {
    fn from(s: &str) -> Self {
        let mut vals = s.split(" ");

        // Ranges are given as 3 numbers.
        // Example: 50 98 20
        //                ^^ Length
        //             ^^ Source range start
        //          ^^ Destination range start
        let dest: usize = vals.next().unwrap().parse().unwrap();
        let source: usize = vals.next().unwrap().parse().unwrap();
        let length: usize = vals.next().unwrap().parse().unwrap();

        return Self {
            source,
            dest,
            length,
        };
    }
}
