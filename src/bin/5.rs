use itertools::Itertools;

use advent_of_code_2023 as lib;

#[derive(Debug, PartialEq, Clone)]
struct Mapping(Vec<(u64, u64, u64)>);

impl Mapping {
    fn apply(&self, n: u64) -> u64 {
        self.apply_inner(n).unwrap_or(n)
    }

    fn apply_inner(&self, n: u64) -> Option<u64> {
        self.0
            .iter()
            .find_map(|(dst, src, range)| (n >= *src && n < src + range).then(|| dst + (n - src)))
    }

    // calculate the input that produces the smallest output by
    // applying the mapping to all inflection points in the range
    fn apply_range(&self, r: std::ops::Range<u64>) -> impl Iterator<Item = (u64, u64)> + '_ {
        [r.start, r.end]
            .into_iter()
            .chain(
                self.0
                    .iter()
                    .filter_map(move |(_, src, _)| r.contains(src).then_some(*src)),
            )
            .map(|n| (n, self.apply(n)))
    }
}

impl std::ops::Add for Mapping {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let indices = self
            .0
            .iter()
            .chain(rhs.0.iter())
            .flat_map(|(_dst, src, range)| [*src, src + range]);

        // consider the possible overlap points between all A and B
        let overlap_points = self
            .0
            .iter()
            .cartesian_product(rhs.0.iter())
            .flat_map(|(a, b)| {
                let mapped_b_start = (b.1 + a.1).saturating_sub(a.0);
                let mapped_b_end = mapped_b_start + b.2;
                (a.1 < mapped_b_start && mapped_b_start < a.1 + a.2
                    || a.1 < mapped_b_end && mapped_b_end < a.1 + a.2)
                    .then_some([mapped_b_start, mapped_b_end])
            })
            .flatten();

        Mapping(
            indices
                .chain(overlap_points)
                .sorted()
                .tuple_windows()
                .filter_map(move |(idx_1, idx_2)| {
                    let first = self.apply_inner(idx_1);
                    let second = rhs.apply_inner(first.unwrap_or(idx_1));
                    second
                        .or(first)
                        .filter(|dst| *dst != idx_1 && idx_2 != idx_1)
                        .map(|dst| (dst, idx_1, idx_2 - idx_1))
                })
                .collect(),
        )
    }
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        let (_, rules) = value.split_once(" map:\n").unwrap();
        Mapping(
            rules
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    // get 3 values from each line
                    let (a, rest) = s.split_once(' ').unwrap();
                    let (b, c) = rest.split_once(' ').unwrap();
                    (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap())
                })
                .collect(),
        )
    }
}

fn main() {
    let input = lib::load_input!();

    let input = input.strip_prefix("seeds: ").unwrap();
    let (numbers, rest) = input.split_once("\n\n").unwrap();
    let numbers = lib::parse_number_list(numbers).collect::<Vec<_>>();

    let map = rest
        .split("\n\n")
        .map(Mapping::from)
        .fold(Mapping(vec![]), |a, b| a + b);

    println!("{}", numbers.iter().map(|n| map.apply(*n)).min().unwrap());

    println!(
        "{}",
        numbers
            .iter()
            .tuples()
            .map(|(low, range)| *low..(low + range))
            .flat_map(|n| map.apply_range(n))
            .min_by_key(|(_, dst)| *dst)
            .unwrap()
            .1
    );
}

#[cfg(test)]
mod test {

    use super::{lib, Mapping};
    use test_case::test_case;

    #[test_case(Mapping(vec![(2,0,3)]), Mapping(vec![(6,5,3)]), Mapping(vec![(2, 0, 3), (6, 5, 3)]) ; "disjoint")]
    #[test_case(Mapping(vec![(2, 0, 5)]), Mapping(vec![(4, 2, 5)]), Mapping(vec![(4, 0, 2), (6,2,3), (7,5,2)]) ; "overlap complete")]
    #[test_case(Mapping(vec![(52, 50, 48), (50, 98, 2)]), Mapping(vec![(39, 0, 15), (0, 15, 37), (37, 52, 2)]), Mapping(vec![(39, 0, 15), (0, 15, 35), (37, 50, 2), (54, 52, 2), (56, 54, 44), (35, 98, 2)]) ; "merge")]
    #[test_case(
        Mapping(vec![(39, 0, 15), (0, 15, 35), (37, 50, 2), (54, 52, 2), (56, 54, 44), (35, 98, 2)]),
        Mapping(vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]),
        Mapping(vec![(28, 0, 4), (32, 4, 3), (35, 7, 4), (39, 11, 3), (49, 14, 1), (42, 15, 7), (57, 22, 4), (0, 26, 24), (26, 50, 1), (27, 51, 1), (50, 52, 1), (51, 53, 1), (52, 54, 5), (61, 59, 2), (63, 61, 7), (70, 68, 30), (24, 98, 2)]))]
    #[test_case(Mapping(vec![]), Mapping(vec![(5, 0, 5)]), Mapping(vec![(5, 0, 5)]) ; "a only")]
    #[test_case(Mapping(vec![(5, 0, 5)]), Mapping(vec![]), Mapping(vec![(5, 0, 5)]) ; "b only")]
    #[test_case(Mapping(vec![(2,0,5)]), Mapping(vec![(8,6,3)]), Mapping(vec![(2, 0, 4), (8, 4, 1), (8, 6, 1), (9,7,2)]) ; "overlap 1")]
    #[test_case(Mapping(vec![(1, 0, 5)]), Mapping(vec![(2, 1, 3)]), Mapping(vec![(2, 0, 1), (3,1,2), (4, 3, 1), (5,4,1)]) ; "subset 2")]
    #[test_case(Mapping(vec![(8, 5, 5)]), Mapping(vec![(7, 8, 4)]), Mapping(vec![(7, 5, 3), (10, 8, 1), (12, 9, 1), (9, 10, 2)]) ; "subset 3")]
    #[test_case(Mapping(vec![(8, 5, 3)]), Mapping(vec![(4,7,5)]), Mapping(vec![(5, 8, 4)]) ; "cancel out")]
    ///    0 1 2 3 4 5 6  7 8 9 10 11 12
    /// A  0 1 2 3 4 8 9 10 8 9 10 11 12
    ///              |      |
    /// B  0 1 2 3 4 5 6  4 5 6  7  8 12
    ///                   |            |
    /// AB 0 1 2 3 4 5 6  7 5 6  7  8 12
    ///                     +--------
    ///              |      |          |
    ///    NONE      O    O B          NONE
    fn merge(a: Mapping, b: Mapping, expected: Mapping) {
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_merge() {
        let test = lib::load_test!();
        let test = test.strip_prefix("seeds: ").unwrap();
        let (_, rest) = test.split_once("\n\n").unwrap();

        let maps: Vec<_> = rest.split("\n\n").map(Mapping::from).collect();
        for map_count in 1..=maps.len() {
            // println!("merging {} maps", map_count);
            let map = maps
                .iter()
                .take(map_count)
                .cloned()
                .fold(Mapping(Vec::new()), |a, b| a + b);

            for x in 1..100 {
                let res = maps.iter().take(map_count).fold(x, |x, map| map.apply(x));
                println!("{} -> {}", x, res);

                if res != map.apply(x) {
                    panic!();
                }
            }
        }

        let expected = [82, 84, 84, 84, 77, 45, 46, 46];
        for x in 1..=maps.len() {
            let map = maps
                .iter()
                .take(x)
                .cloned()
                .fold(Mapping(Vec::new()), |a, b| a + b);

            assert_eq!(maps[x - 1].apply(expected[x - 1]), expected[x]);
            // println!("{:?}", map);
            assert_eq!(map.apply(82), expected[x], "{} / {}", x, expected[x]);
        }
    }

    #[test]
    fn test_convert() {
        let m = Mapping(vec![(39, 0, 15)]);
        assert_eq!(m.apply(0), 39);
    }
}
