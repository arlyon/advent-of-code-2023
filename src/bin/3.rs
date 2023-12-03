use std::iter;

use advent_of_code_2023 as lib;
use itertools::{chain, Itertools};

#[derive(Debug)]
enum Entry<'a> {
    Number(&'a str, usize, usize),
    Symbol(usize),
}

#[derive(Debug)]
struct Line<'a>(Vec<Entry<'a>>);

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Self {
        let groups = s.chars().enumerate().group_by(|c| match c.1 {
            '.' => 0,
            '*' | '#' | '+' | '$' | '-' | '%' | '&' | '=' | '/' | '@' => 1,
            _ => 2,
        });
        Self(
            groups
                .into_iter()
                .flat_map(|(symbols, groups)| {
                    match symbols {
                        0 => vec![],
                        // individual symbols
                        1 => groups.map(|(idx, _)| Entry::Symbol(idx)).collect(),
                        2 => {
                            // contiguous numbers
                            let (first, last) = lib::first_last(groups, |a| Some(a.0)).unwrap();
                            vec![Entry::Number(s, first, last)]
                        }
                        _ => unreachable!(),
                    }
                    .into_iter()
                })
                .collect(),
        )
    }
}

impl<'a> Line<'a> {
    fn find_part_numbers(
        &'a self,
        above: &'a Line,
        below: &'a Line,
    ) -> impl Iterator<Item = u32> + 'a {
        self.0
            .iter()
            .filter_map(|e| match e {
                Entry::Number(s, first, last) => {
                    chain(above.0.iter(), chain(self.0.iter(), below.0.iter()))
                        .filter_map(|entry| match entry {
                            Entry::Symbol(s) => Some(s),
                            _ => None,
                        })
                        .any(|i| *i >= first.saturating_sub(1) && *i <= last.saturating_add(1))
                        .then_some(&s[*first..=*last])
                }
                _ => None,
            })
            .map(|s| s.parse().unwrap())
    }

    // a gear is a symbol with exactly two numbers adjacent to it
    fn find_gears(
        &'a self,
        above: &'a Line,
        below: &'a Line,
    ) -> impl Iterator<Item = (u32, u32)> + 'a {
        self.0.iter().filter_map(|e| match e {
            Entry::Symbol(idx) => {
                let mut iter = chain(above.0.iter(), chain(self.0.iter(), below.0.iter()))
                    .filter_map(|entry| match entry {
                        Entry::Number(s, first, last)
                            if *first <= idx.saturating_add(1)
                                && *last >= idx.saturating_sub(1) =>
                        {
                            Some(s[*first..=*last].parse().unwrap())
                        }
                        _ => None,
                    });
                let fst = iter.next().zip(iter.next());
                if iter.next().is_none() {
                    fst
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}

fn main() {
    let input = lib::load_input!();

    // add a line before and after the input for the window search
    let lines: Vec<_> = chain(
        iter::once(Line(vec![])),
        chain(input.lines().map(Line::from), iter::once(Line(vec![]))),
    )
    .collect();

    println!(
        "{:?}",
        lines
            .iter()
            .tuple_windows()
            .flat_map(|(fst, snd, thrd)| snd.find_part_numbers(fst, thrd))
            .sum::<u32>()
    );

    println!(
        "{:?}",
        lines
            .iter()
            .tuple_windows()
            .flat_map(|(fst, snd, thrd)| snd.find_gears(fst, thrd))
            .map(|(a, b)| a * b)
            .sum::<u32>()
    )
}
