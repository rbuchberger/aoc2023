use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one actual:  {}", part_one("input"));

    println!("Part two example: {}", part_two("example"));
    println!("Part two actual:  {}", part_two("input"));
}

fn part_one(filename: &str) -> usize {
    let (seeds, maps) = parse_file(filename);

    return seeds
        .iter()
        .map(|s| {
            trace_down(
                Trace {
                    key: String::from("seed"),
                    value: *s,
                    bound_dist: None,
                },
                &maps,
            )
            .value
        })
        .min()
        .unwrap();
}

fn part_two(filename: &str) -> usize {
    let (seeds, maps) = parse_file(filename);
    let seeds = get_seed_ranges(seeds);

    let result = seeds
        .iter()
        .flat_map(|s| trace_range(s, &maps))
        .min_by_key(|s| s.value)
        .unwrap()
        .value;

    return result;
}

fn trace_range(range: &RangeInclusive<usize>, maps: &HashMap<String, CategoryMap>) -> Vec<Trace> {
    let mut values = Vec::new();
    let mut value = range.start().clone();

    let trace = Trace {
        key: String::from("seed"),
        value,
        bound_dist: None,
    };

    while range.contains(&value) {
        let trace = trace_down(trace.clone(), maps);

        value = match trace.bound_dist {
            Some(bound_dist) => value + bound_dist,
            None => *range.end(),
        };

        values.push(trace);
    }

    return values;
}

fn get_seed_ranges(seeds: Vec<usize>) -> Vec<RangeInclusive<usize>> {
    seeds
        .windows(2)
        .step_by(2)
        .map(|slice| {
            let start = slice[0];
            let end = start + slice[1];

            return start..=end;
        })
        .collect()
}

fn trace_down(trace: Trace, maps: &HashMap<String, CategoryMap>) -> Trace {
    match maps.get(&trace.key) {
        None => trace,
        Some(m) => trace_down(
            Trace {
                key: m.output_key.clone(),
                value: m.translate(trace.value),
                bound_dist: Trace::compare_bounds(trace.bound_dist, m.bound_dist(trace.value)),
            },
            maps,
        ),
    }
}

#[derive(Debug, Clone)]
struct Trace {
    key: String,
    value: usize,
    bound_dist: Option<usize>,
}

impl Trace {
    fn compare_bounds(a: Option<usize>, b: Option<usize>) -> Option<usize> {
        let bounds = vec![a, b];
        let mut bounds = bounds.iter().filter_map(|v| *v).collect::<Vec<usize>>();
        bounds.sort();

        return bounds.first().copied();
    }
}

#[derive(Debug)]
struct CategoryMap {
    input_key: String,
    output_key: String,
    ranges: Vec<RangeConversion>,
    boundaries: Vec<usize>,
}

impl CategoryMap {
    fn translate(&self, input: usize) -> usize {
        match self.ranges.iter().find_map(|range| range.translate(input)) {
            None => input,
            Some(v) => v,
        }
    }

    fn bound_dist(&self, input: usize) -> Option<usize> {
        let bound = self.boundaries.iter().find(|v| **v > input)?;

        return Some(bound - input);
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
        let mut boundaries: Vec<_> = ranges
            .iter()
            .flat_map(|r| vec![r.source, r.source + r.length])
            .collect();

        boundaries.sort();

        return Self {
            input_key,
            output_key,
            ranges,
            boundaries,
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
