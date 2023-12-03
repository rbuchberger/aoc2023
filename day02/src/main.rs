// Defined by problem
fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one input: {}", part_one("input"));

    println!("Part two example: {}", part_two("example"));
    println!("Part two example: {}", part_two("input"));
}

struct Game {
    id: u32,
    rounds: Vec<Vec<Color>>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let mut iter = line.split(": ");
        let (id, rounds) = (iter.next().unwrap(), iter.next().unwrap());
        if iter.next().is_some() {
            panic!("Invalid game");
        }

        let id = id.split(" ").last().unwrap().parse::<u32>().unwrap();

        let rounds = rounds
            .split("; ")
            .map(|round| round.split(", ").map(Color::parse).collect())
            .collect();

        Game { id, rounds }
    }

    fn check(&self) -> bool {
        self.rounds
            .iter()
            .all(|round| round.iter().all(Color::check))
    }

    fn max_values(&self) -> ColorSet {
        let mut max_values = ColorSet::new();

        self.rounds.iter().for_each(|round| {
            round.iter().for_each(|color| {
                max_values.update_if_greater(color);
            })
        });

        return max_values;
    }
}

#[derive(Debug)]
struct ColorSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl ColorSet {
    fn new() -> Self {
        ColorSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn update_if_greater(&mut self, color: &Color) {
        match color {
            Color::Red(n) => {
                if *n > self.red {
                    self.red = *n
                }
            }
            Color::Green(n) => {
                if *n > self.green {
                    self.green = *n
                }
            }
            Color::Blue(n) => {
                if *n > self.blue {
                    self.blue = *n
                }
            }
        }
    }

    fn powers(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Color {
    const RED_LIMIT: u32 = 12;
    const GREEN_LIMI: u32 = 13;
    const BLUE_LIMI: u32 = 14;

    fn parse(line: &str) -> Self {
        let mut iter = line.split(" ");

        let (n, color) = (iter.next().unwrap(), iter.next().unwrap());
        if iter.next().is_some() {
            panic!("Invalid color");
        }

        match (n.parse::<u32>().unwrap(), color) {
            (n, "red") => Color::Red(n),
            (n, "green") => Color::Green(n),
            (n, "blue") => Color::Blue(n),
            _ => panic!("Invalid color"),
        }
    }

    fn check(&self) -> bool {
        match self {
            Color::Red(n) => n <= &Color::RED_LIMIT,
            Color::Green(n) => n <= &Color::GREEN_LIMI,
            Color::Blue(n) => n <= &Color::BLUE_LIMI,
        }
    }
}

fn part_one(filename: &str) -> u32 {
    let text = std::fs::read_to_string(filename).unwrap();
    let games = text.trim().lines().map(Game::parse).collect::<Vec<_>>();

    let sum = games
        .iter()
        .filter(|game| game.check())
        .map(|game| game.id)
        .sum::<u32>();

    return sum;
}

fn part_two(filename: &str) -> u32 {
    let text = std::fs::read_to_string(filename).unwrap();
    let games = text.trim().lines().map(Game::parse).collect::<Vec<_>>();

    let max_values = games
        .iter()
        .map(|game| game.max_values())
        .collect::<Vec<_>>();

    let power_sum = max_values.iter().map(|values| values.powers()).sum::<u32>();

    return power_sum;
}
