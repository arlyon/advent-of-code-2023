use advent_of_code_2023 as lib;

const DIGITS: [(&str, u32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let input = lib::load_input!();

    println!(
        "{}",
        input
            .lines()
            .filter_map(|line| lib::first_last(line.chars(), |a: char| a.to_digit(10)))
            .map(|(a, b)| a * 10 + b)
            .sum::<u32>()
    );

    // now also check for written out numbers
    println!(
        "{}",
        input
            .lines()
            .filter_map(|line| lib::first_last(
                (0..line.len())
                    .into_iter()
                    .filter_map(|idx| DIGITS.iter().find(|d| line[idx..].starts_with(d.0))),
                |(_, val)| Some(val),
            ))
            .map(|(a, b)| a * 10 + b)
            .sum::<u32>()
    );
}
