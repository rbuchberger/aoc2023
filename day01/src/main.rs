fn main() {
    println!["part one example: {}", part_one("example")];
    println!["part one: {}", part_one("input")];
}

fn part_one(filename: &str) -> u32 {
    let text = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();
    let lines = text.lines();
    let digit_lines = lines
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let items = digit_lines
        .iter()
        .map(|l| vec![*l.first().unwrap(), *l.last().unwrap()])
        .collect::<Vec<_>>();

    let sum = items
        .iter()
        .map(|item| item.iter().collect::<String>().parse::<u32>().unwrap())
        .sum::<u32>();

    return sum;
}
