// Defined by problem
fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one input: {}", part_one("input"));
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
