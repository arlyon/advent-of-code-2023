use std::collections::HashSet;

use advent_of_code_2023 as lib;

struct Card(u32, HashSet<u64>, HashSet<u64>);

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let (number, rest) = s.trim_start_matches("Card ").split_once(": ").unwrap();
        let (winners, ours) = rest.split_once(" | ").unwrap();
        Self(
            number.trim().parse().unwrap(),
            lib::parse_number_list(winners).collect(),
            lib::parse_number_list(ours).collect(),
        )
    }
}

impl Card {
    fn wins(&self) -> u32 {
        self.1.intersection(&self.2).count() as u32
    }

    fn score(&self) -> u64 {
        self.wins().checked_sub(1).map(|x| 2u64.pow(x)).unwrap_or(0)
    }
}

fn main() {
    let cards = lib::load_input!()
        .lines()
        .map(Card::from)
        .collect::<Vec<_>>();

    println!("{:?}", cards.iter().map(Card::score).sum::<u64>());

    let mut counter = vec![1; cards.len()];
    for card in cards {
        for i in 0..card.wins() {
            counter[(card.0 + i) as usize] += counter[card.0 as usize - 1];
        }
    }

    println!("{:?}", counter.iter().sum::<u64>());
}
