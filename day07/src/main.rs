use std::cmp::Ordering;
use std::collections::hash_map::HashMap;

fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one actual:  {}", part_one("input"));

    println!("Part two example: {}", part_two("example"));
    println!("Part two actual:  {}", part_two("input"));
}

fn part_one(filename: &str) -> usize {
    let mut hands = parse_file(filename, &STD_RULES);
    return score(&mut hands);
}

fn part_two(filename: &str) -> usize {
    let mut hands = parse_file(filename, &JOKER_RULES);
    return score(&mut hands);
}

#[derive(Debug, Clone, Copy)]
struct Rules {
    cards: [char; 13],
    tests: [fn(&Vec<char>) -> bool; 7],
}

const STD_RULES: Rules = Rules {
    cards: [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ],
    tests: [
        (|_| true),
        (|h| of_a_kind_std(h) == 2),
        (|h| of_a_kind_std(h) == 2 && group_cards_std(h).len() == 3), // two pair
        (|h| of_a_kind_std(h) == 3),
        (|h| of_a_kind_std(h) == 3 && group_cards_std(h).len() == 2), // full  house
        (|h| of_a_kind_std(h) == 4),
        (|h| of_a_kind_std(h) == 5),
    ],
};

fn group_cards_std(cards: &Vec<char>) -> HashMap<char, u8> {
    cards.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(*c).or_insert(0) += 1;
        acc
    })
}

fn of_a_kind_std(cards: &Vec<char>) -> u8 {
    group_cards_std(cards).values().max().unwrap().clone()
}

const JOKER_RULES: Rules = Rules {
    cards: [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ],

    tests: [
        (|_| true),
        (|h| of_a_kind_j(h) == 2),
        (is_2pair_j),
        (|h| of_a_kind_j(h) == 3),
        (is_fullhouse_j),
        (|h| of_a_kind_j(h) == 4),
        (|h| of_a_kind_j(h) == 5),
    ],
};

fn group_cards_j(cards: &Vec<char>) -> (u8, HashMap<char, u8>) {
    let mut groups = group_cards_std(cards);
    let jokers = groups.remove_entry(&'J').unwrap_or(('J', 0)).1;

    return (jokers, groups);
}

fn of_a_kind_j(cards: &Vec<char>) -> u8 {
    let (jokers, groups) = group_cards_j(cards);

    groups.values().max().unwrap_or(&0) + jokers
}

fn is_2pair_j(chars: &Vec<char>) -> bool {
    let (jokers, groups) = group_cards_j(chars);

    let pairs = groups.values().filter(|&v| v == &2).count();
    jokers + pairs as u8 >= 2
}

fn is_fullhouse_j(chars: &Vec<char>) -> bool {
    let (jokers, groups) = group_cards_j(chars);
    let groups = groups.values().collect::<Vec<_>>();

    match jokers {
        0 => groups.contains(&&2) && groups.contains(&&3),
        _ => groups.len() <= 2,
    }
}

fn parse_file(filename: &str, rules: &Rules) -> Vec<Hand> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| Hand::parse(l, rules))
        .collect::<Vec<_>>()
}

fn score(hands: &mut Vec<Hand>) -> usize {
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1))
        .sum::<usize>()
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    rules: Rules,
}

impl Hand {
    fn base_rank(&self) -> u8 {
        self.rules
            .tests
            .iter()
            .enumerate()
            .rev()
            .find(|(_, f)| f(&self.cards))
            .unwrap()
            .0 as u8
    }

    fn parse(s: &str, rules: &Rules) -> Self {
        let mut s = s.split(' ');

        Self {
            rules: rules.clone(),
            cards: s.next().unwrap().chars().collect::<Vec<_>>(),
            bid: s.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        fn rank_card(card: &char, rules: &Rules) -> u8 {
            rules.cards.iter().position(|&c| c == *card).unwrap() as u8
        }

        match self.base_rank().cmp(&other.base_rank()) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(a, b)| {
                    if a != b {
                        Some(rank_card(a, &self.rules).cmp(&rank_card(b, &self.rules)))
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

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// Convenience method for testing
#[allow(dead_code)]
fn charvec(s: &str) -> Vec<char> {
    s.chars().collect::<Vec<_>>()
}

// What follows is the evidence of my struggle to deal with a logic error where the example
// problems and part one passed, but the actual test for part 2 did not pass.
//
// The problem was that I wan't handling a hand of all jokers correctly.

#[test]
fn test_part_one_example() {
    assert_eq!(part_one("example"), 6440);
}

#[test]
fn test_part_one_actual() {
    assert_eq!(part_one("input"), 251121738);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two("example"), 5905);
}

#[test]
fn test_part_two_actual() {
    assert_eq!(part_two("input"), 251421071);
}

#[test]
fn test_group_cards_j() {
    assert_eq!(group_cards_j(&charvec("12345")).0, 0);
    assert_eq!(group_cards_j(&charvec("J2345")).0, 1);
    assert_eq!(group_cards_j(&charvec("JJ345")).0, 2);
    assert_eq!(group_cards_j(&charvec("JJJ45")).0, 3);
    assert_eq!(group_cards_j(&charvec("JJJJJ")).0, 5);

    assert_eq!(group_cards_j(&charvec("22233")).1.get(&'2').unwrap(), &3);
    assert_eq!(group_cards_j(&charvec("22233")).1.get(&'3').unwrap(), &2);
    assert_eq!(
        group_cards_j(&charvec("22233")).1.values().max().unwrap(),
        &3
    );
    assert!(group_cards_j(&charvec("JJJJJ")).1.get(&'2').is_none());
}

#[test]
fn test_of_a_kind_j() {
    assert_eq!(of_a_kind_j(&charvec("12345")), 1);
    assert_eq!(of_a_kind_j(&charvec("22345")), 2);
    assert_eq!(of_a_kind_j(&charvec("J2345")), 2);
    assert_eq!(of_a_kind_j(&charvec("J2245")), 3);
    assert_eq!(of_a_kind_j(&charvec("JJ245")), 3);
    assert_eq!(of_a_kind_j(&charvec("JJJ45")), 4);
    assert_eq!(of_a_kind_j(&charvec("JJ445")), 4);
    assert_eq!(of_a_kind_j(&charvec("22222")), 5);
    assert_eq!(of_a_kind_j(&charvec("JJ222")), 5);
    assert_eq!(of_a_kind_j(&charvec("JJJJJ")), 5);
}

#[test]
fn test_is_2pair_j() {
    assert!(is_2pair_j(&charvec("22334")));
    assert!(is_2pair_j(&charvec("J2334")));
    assert!(is_2pair_j(&charvec("JJ345")));
    assert!(is_2pair_j(&charvec("JJJ45")));

    assert!(!is_2pair_j(&charvec("12345")));
    assert!(!is_2pair_j(&charvec("22345")));
    assert!(!is_2pair_j(&charvec("J2345")));
}

#[test]
fn test_is_fullhouse_j() {
    assert!(is_fullhouse_j(&charvec("22333")));
    assert!(is_fullhouse_j(&charvec("J2333")));
    assert!(is_fullhouse_j(&charvec("JJ333")));
    assert!(is_fullhouse_j(&charvec("22J33")));
    assert!(is_fullhouse_j(&charvec("J2J33")));
    assert!(is_fullhouse_j(&charvec("JJJ34")));

    assert!(!is_fullhouse_j(&charvec("12345")));
    assert!(!is_fullhouse_j(&charvec("12222")));
    assert!(!is_fullhouse_j(&charvec("22335")));
    assert!(!is_fullhouse_j(&charvec("J2235")));
    assert!(!is_fullhouse_j(&charvec("JJ245")));
    assert!(!is_fullhouse_j(&charvec("JJ245")));
}
