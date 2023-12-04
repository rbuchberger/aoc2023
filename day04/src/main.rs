fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one example: {}", part_one("input"));
}

struct Card {
    winners: Vec<usize>,
    values: Vec<usize>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let mut numbers = line.split(": ").skip(1).next().unwrap().split(" | ");

        let (winners, values) = (numbers.next().unwrap(), numbers.next().unwrap());

        let winners = winners
            .trim()
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap())
            .collect();

        let values = values
            .trim()
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap())
            .collect();

        return Self { winners, values };
    }

    fn score(&self) -> usize {
        let winning_values: Vec<_> = self
            .values
            .iter()
            .filter(|v| self.winners.contains(v))
            .collect();

        return match winning_values.len() {
            0 => 0,
            v => {
                let r = v - 1;
                2usize.pow(r.try_into().unwrap())
            }
        };
    }
}

fn part_one(filename: &str) -> usize {
    let text = std::fs::read_to_string(filename).unwrap();
    let cards = text.trim().lines().map(Card::parse).collect::<Vec<_>>();

    return cards.iter().map(Card::score).sum();
}
