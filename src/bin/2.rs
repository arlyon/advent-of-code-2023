use advent_of_code_2023 as lib;
use itertools::Itertools;

struct Game<'a>(u32, Vec<Vec<(u32, &'a str)>>);

impl<'a> From<&'a str> for Game<'a> {
    fn from(s: &'a str) -> Self {
        let (_, rest) = s.split_once("Game ").unwrap();
        let (number, rest) = rest.split_once(": ").unwrap();
        Game(
            number.parse().unwrap(),
            rest.split("; ")
                .map(|s| {
                    s.split(", ")
                        .map(|s| s.split_once(' ').unwrap())
                        .map(|(a, b)| (a.parse().unwrap(), b))
                        .collect()
                })
                .collect(),
        )
    }
}

impl<'a> Game<'a> {
    /// check that a game can be played with a given collection
    fn validate(&self, collection: &[(u32, &str)]) -> bool {
        self.1.iter().all(|round| {
            round
                .iter()
                .all(|(i, c1)| collection.iter().any(|(j, c2)| c1 == c2 && j >= i))
        })
    }

    /// get the smallest collection that satisfies a game
    fn fewest(&'a self) -> impl Iterator<Item = &(u32, &'a str)> {
        self.1
            .iter()
            .flatten()
            .sorted_by(|(i, _), (j, _)| j.cmp(i))
            .unique_by(|(_, c)| *c)
    }
}

fn main() {
    let games = lib::load_input!()
        .lines()
        .map(Game::from)
        .collect::<Vec<_>>();

    println!(
        "{:?}",
        games
            .iter()
            .filter(|g| g.validate(&[(12, "red"), (13, "green"), (14, "blue")]))
            .map(|g| g.0)
            .sum::<u32>()
    );

    println!(
        "{:#?}",
        games
            .iter()
            .map(|g| g.fewest().map(|(i, _)| i).product::<u32>())
            .sum::<u32>()
    );
}
