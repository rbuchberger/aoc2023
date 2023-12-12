use std::cmp::Ordering;
use std::collections::hash_map::HashMap;

fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one actual:  {}", part_one("input"));
}

fn part_one(filename: &str) -> usize {
    let mut hands = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| Hand::from(l))
        .collect::<Vec<_>>();

    hands.sort();

    let score = &hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1))
        .sum::<usize>();

    return *score;
}

#[test]
fn test_example() {
    assert!(part_one("example") == 6440);
}

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const HANDTYPES: [fn(&Vec<char>) -> bool; 7] = [
    (|_| true),                                                              // high card
    (|chars| group_cards(chars).values().any(|&v| v == 2)),                  // one pair
    (|chars| group_cards(chars).values().filter(|&v| v == &2).count() == 2), // two pair
    (|chars| group_cards(chars).values().any(|&v| v == 3)),                  // three of a kind
    // Full house
    (|chars| {
        let groups = group_cards(chars);
        let groups = groups.values().collect::<Vec<_>>();

        groups.contains(&&2) && groups.contains(&&3)
    }),
    (|chars| group_cards(chars).values().any(|&v| v == 4)), // four of a kind
    (|chars| group_cards(chars).values().any(|&v| v == 5)), // five of a kind
];

fn group_cards(cards: &Vec<char>) -> HashMap<char, u8> {
    cards.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(*c).or_insert(0) += 1;
        acc
    })
}

fn rank_card(card: &char) -> u8 {
    CARDS.iter().position(|&c| c == *card).unwrap() as u8
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

impl Hand {
    fn base_rank(&self) -> u8 {
        HANDTYPES
            .iter()
            .enumerate()
            .rev()
            .find(|(_, f)| f(&self.cards))
            .unwrap()
            .0 as u8
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let s = s.split(' ').collect::<Vec<_>>();
        let cards = s[0].chars().collect::<Vec<_>>();
        let bid = s[1].parse::<usize>().unwrap();

        Self { cards, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.base_rank().cmp(&other.base_rank()) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(a, b)| {
                    if a != b {
                        Some(rank_card(a).cmp(&rank_card(b)))
                    } else {
                        None
                    }
                })
                .unwrap(),

            result => result,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
