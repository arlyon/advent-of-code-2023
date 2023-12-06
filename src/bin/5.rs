use std::collections::HashSet;

use advent_of_code_2023 as lib;
use itertools::Itertools;

/// vector of disjoint ranges, with destination, source, and range length
#[derive(Debug, PartialEq, Clone)]
struct Mapping(Vec<(u64, u64, u64)>);

impl Mapping {
    fn apply(&self, n: u64) -> u64 {
        for (dst, src, range) in &self.0 {
            // println!("{} {} {}", dst, src, range);
            if n >= *src && n < src + range {
                return dst + (n - src);
            }
        }
        return n;
    }

    fn is_affected(&self, other: &Self, n: u64) -> (bool, bool) {
        let n_1 = self.apply(n);
        let first_apply = n_1 != n;
        let n_2 = other.apply(n_1);
        let second_apply = n_2 != n_1;
        (first_apply, second_apply)
    }

    fn zones<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = (u64, u64, u64)> + 'a {
        // get the ranges of A after application
        let mut sources = self.0.iter().sorted_by_key(|s| s.1);
        let mut destinations = other.0.iter().sorted_by_key(|s| s.1);

        let mut source = sources.next();
        let mut destination = destinations.next();

        let mut indices = HashSet::from([0]);

        loop {
            match (source, destination) {
                (Some((dst_a, src_a, range_a)), Some((dst_b, src_b, range_b))) => {
                    let start_a = *src_a;
                    let end_a = src_a + range_a;
                    let start_b = *src_b;
                    let end_b = src_b + range_b;

                    indices.extend([start_a, end_a, start_b, end_b].iter().copied());

                    let mapped_b_start = (src_b + src_a).saturating_sub(*dst_a);
                    let mapped_b_end = mapped_b_start + range_b;

                    // check if the mapped ranges overlap and add them if they do
                    println!(
                        "checking overlap: {} < {} < {} or {} < {} < {}",
                        start_a, mapped_b_start, end_a, start_a, mapped_b_end, end_a
                    );
                    if start_a < mapped_b_start && mapped_b_start < end_a
                        || start_a < mapped_b_end && mapped_b_end < end_a
                    {
                        println!("yielding {}..{}", mapped_b_start, mapped_b_end);
                        indices.insert(mapped_b_start);
                        indices.insert(mapped_b_end);
                    }

                    // increment the iterator that is behind
                    if end_a < end_b {
                        source = sources.next();
                    } else if end_b < end_a {
                        destination = destinations.next();
                    } else {
                        source = sources.next();
                        destination = destinations.next();
                    }
                }
                (Some(x), None) | (None, Some(x)) => {
                    indices.insert(x.1);
                    indices.insert(x.1 + x.2);
                    source = sources.next();
                    destination = destinations.next();
                }
                (None, None) => break,
            }
        }

        println!("{:?}", indices);

        indices
            .into_iter()
            .sorted()
            .map(|x| (x, self.is_affected(other, x)))
            .tuple_windows()
            .filter_map(move |((idx_1, calls), (idx_2, _))| {
                println!("{} {} {:?}", idx_1, idx_2, calls);
                let dst = match calls {
                    (true, true) => other.apply(self.apply(idx_1)),
                    (true, false) => self.apply(idx_1),
                    (false, true) => other.apply(idx_1),
                    (false, false) => return None,
                };

                if dst == idx_1 {
                    return None;
                }

                Some((dst, idx_1, idx_2 - idx_1))
            })
    }

    // get a mapping that contains all the ranges in A that affect B
    fn merge(&self, other: &Self) -> Self {
        // get the ranges of A after application
        let mut sources = self.0.iter().sorted_by_key(|s| s.1);
        let mut destinations = other.0.iter().sorted_by_key(|s| s.1);

        let mut source = sources.next();
        let mut destination = destinations.next();

        let mut pieces = vec![];

        println!("merging {:?} {:?}", self, other);

        loop {
            match (source, destination) {
                (Some((dst_a, src_a, range_a)), Some((dst_b, src_b, range_b))) => {
                    // get min and max coords of B in A space
                    let delta_a = *dst_a as i64 - *src_a as i64;
                    let delta_b = *dst_b as i64 - *src_b as i64;

                    // gets the ranges of A and B in A space
                    let start_a_mapped = *src_a;
                    let start_b_mapped = (src_b + src_a).saturating_sub(*dst_a);
                    let end_a_mapped = src_a + range_a;
                    let end_b_mapped = start_b_mapped + range_b;

                    // get overlap between start and end. if they don't overlap, set it to None
                    let overlap = if start_a_mapped > end_b_mapped || start_b_mapped > end_a_mapped
                    {
                        None
                    } else {
                        let start = start_a_mapped.max(start_b_mapped);
                        let end = end_a_mapped.min(end_b_mapped);

                        let src = start;
                        let dst = src + dst_a - src_a + dst_b - src_b;
                        let range = end - start;
                        if src == dst || range == 0 {
                            None
                        } else {
                            Some((dst, src, range))
                        }
                    };

                    println!(
                        "({} {} {} Δ {}) ({} {} {} Δ {}) -> {}..{} {}..{} {:?}",
                        dst_a,
                        src_a,
                        range_a,
                        delta_a,
                        dst_b,
                        src_b,
                        range_b,
                        delta_b,
                        start_a_mapped,
                        end_a_mapped,
                        start_b_mapped,
                        end_b_mapped,
                        overlap
                    );

                    let overlap_start = overlap.map(|(_, src, _)| src);
                    let overlap_end = overlap.map(|(_, src, range)| src + range);

                    let FUCK = overlap_start.unwrap_or(src_a + range_a);
                    if *src_a < FUCK {
                        println!("PREFIX A {} {}", src_a, FUCK);
                    }

                    let FUCK = overlap_start.unwrap_or(src_b + range_b);
                    if *src_b < FUCK {
                        println!("PREFIX B");
                    }

                    // overlap
                    if let Some((dst, src, range)) = overlap {
                        println!("overlap = ({} {} {})", dst, src, range);
                        pieces.push((dst, src, range));
                    }

                    let start_suffix_a = overlap_end.unwrap_or(*src_a);
                    if *dst_a + range_a > start_suffix_a {
                        println!("SUFFIX A: {} - {}", start_suffix_a, src_a);
                        let start = start_suffix_a;
                        pieces.push((start, start, range_a + src_a - start)); // (12, 9, 1)
                    }

                    let start_suffix_b = overlap_end.unwrap_or(*src_b);
                    if *dst_b + range_b > end_b_mapped {
                        println!("SUFFIX B: {} - {} {}", start_suffix_b, src_b, end_b_mapped);
                        let start = start_suffix_b;
                        // 9, 10, 2
                        pieces.push((start + dst_b - src_b, start, range_b + src_b - start))
                    }

                    // advance the relevant iterator
                    println!("ADVANCING {} {} {:?}", end_a_mapped, end_b_mapped, overlap);
                    if end_a_mapped < start_b_mapped {
                        source = sources.next();
                    } else if end_b_mapped < start_a_mapped {
                        destination = destinations.next();
                    } else {
                        source = sources.next();
                        destination = destinations.next();
                    }
                }
                (Some(x), None) | (None, Some(x)) => {
                    pieces.push(x.clone());
                    source = sources.next();
                    destination = destinations.next();
                }
                (None, None) => break,
            }
        }
        Mapping(pieces)
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

#[cfg(test)]
mod test {
    use std::{collections::HashSet, hash::Hash};

    use super::{lib, Mapping};
    use test_case::test_case;

    //    0 1 2 3 4 5 6 7  8 9
    // A  2 3 4 5 6 5 6 7  8 9
    //    +--------
    // B  0 1 2 3 4 5 8 9 10 9
    //                +-----
    // AB 2 3 4 5 8 5 8 9 10 9
    //    +------ +   +-----
    //    2       4   2
    fn onto(a: Mapping, b: Mapping, expected: &[(u64, u64, u64)]) {
        assert_eq!(&a.merge(&b).0, expected);
    }

    #[test_case(Mapping(vec![(2,0,3)]), Mapping(vec![(6,5,3)]), &[(2, 0, 3), (6, 5, 3)] ; "disjoint")]
    #[test_case(Mapping(vec![(2, 0, 5)]), Mapping(vec![(4, 2, 5)]), &[(4, 0, 2), (6,2,3), (7,5,2)] ; "overlap complete")]
    #[test_case(Mapping(vec![(52, 50, 48), (50, 98, 2)]), Mapping(vec![(39, 0, 15), (0, 15, 37), (37, 52, 2)]), &[(39, 0, 15), (0, 15, 35), (37, 50, 2), (54, 52, 2), (56, 54, 44), (35, 98, 2)] ; "merge")]
    #[test_case(Mapping(vec![]), Mapping(vec![(5, 0, 5)]), &[(5, 0, 5)] ; "a only")]
    #[test_case(Mapping(vec![(5, 0, 5)]), Mapping(vec![]), &[(5, 0, 5)] ; "b only")]
    #[test_case(Mapping(vec![(2,0,5)]), Mapping(vec![(8,6,3)]), &[(2, 0, 4), (8, 4, 1), (8, 6, 1), (9,7,2)] ; "overlap 1")]
    #[test_case(Mapping(vec![(1, 0, 5)]), Mapping(vec![(2, 1, 3)]), &[(2, 0, 1), (3,1,2), (4, 3, 1), (5,4,1)] ; "subset 2")]
    #[test_case(Mapping(vec![(8, 5, 5)]), Mapping(vec![(7, 8, 4)]), &[(7, 5, 3), (10, 8, 1), (12, 9, 1), (9, 10, 2)] ; "subset 3")]
    #[test_case(Mapping(vec![(8, 5, 3)]), Mapping(vec![(4,7,5)]), &[(5, 8, 4)] ; "cancel out")]
    ///    0 1 2 3 4 5 6  7 8 9 10 11 12
    /// A  0 1 2 3 4 8 9 10 8 9 10 11 12
    ///              |      |
    /// B  0 1 2 3 4 5 6  4 5 6  7  8 12
    ///                   |            |
    /// AB 0 1 2 3 4 5 6  7 5 6  7  8 12
    ///                     +--------
    ///              |      |          |
    ///    NONE      O    O B          NONE
    fn zones(a: Mapping, b: Mapping, expected: &[(u64, u64, u64)]) {
        assert_eq!(&a.zones(&b).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_merge() {
        let test = lib::load_test!();
        let test = test.strip_prefix("seeds: ").unwrap();
        let (_, rest) = test.split_once("\n\n").unwrap();

        let maps: Vec<_> = rest.split("\n\n").map(Mapping::from).collect();
        for map_count in 1..=maps.len() {
            println!("merging {} maps", map_count);
            let map = maps
                .iter()
                .take(map_count)
                .fold(Mapping(Vec::new()), |a, b| a.merge(b));

            for x in 1..100 {
                let res = maps.iter().take(map_count).fold(x, |x, map| {
                    println!("{} -> {}", x, map.apply(x));
                    map.apply(x)
                });

                assert_eq!(map.apply(x), res, "{} + {} {:#?}", x, map_count, map);
            }
        }

        let expected = [82, 84, 84, 84, 77, 45, 46, 46];
        for x in 1..=maps.len() {
            let map = maps
                .iter()
                .take(x)
                .fold(Mapping(Vec::new()), |a, b| a.merge(b));

            assert_eq!(maps[x - 1].apply(expected[x - 1]), expected[x]);
            println!("{:?}", map);
            assert_eq!(map.apply(82), expected[x], "{} / {}", x, expected[x]);
        }
    }

    // #[test]
    // fn identity() {
    //     let a = Mapping(vec![(0, 0, 100)]);
    //     let identity = Mapping(vec![]);
    //     assert_eq!(a.clone().merge(&identity), a);
    // }

    #[test]
    fn test_convert() {
        let m = Mapping(vec![(39, 0, 15)]);
        assert_eq!(m.apply(0), 39);
    }

    // #[test]
    // fn test_a() {
    //     let input = lib::load_test!();

    //     let input = input.strip_prefix("seeds: ").unwrap();
    //     let (_, rest) = input.split_once("\n\n").unwrap();

    //     let map = rest
    //         .split("\n\n")
    //         .map(Mapping::from)
    //         .fold(Mapping(Vec::new()), |a, b| a.merge(&b));

    //     assert_eq!(map.apply(82), 46);
    // }
}

fn main() {
    let input = lib::load_input!();

    let input = input.strip_prefix("seeds: ").unwrap();
    let (numbers, rest) = input.split_once("\n\n").unwrap();
    let numbers = lib::parse_number_list(numbers).collect::<Vec<_>>();

    let maps = rest.split("\n\n").map(Mapping::from).collect::<Vec<_>>();

    let map = maps.iter().fold(Mapping(Vec::new()), |a, b| a.merge(b));

    println!("{:?}", map);

    println!(
        "{}",
        numbers.iter().copied().map(|n| map.apply(n)).min().unwrap()
    );

    println!("{}", map.apply(82));

    println!(
        "{}",
        numbers
            .iter()
            .copied()
            .tuples()
            .flat_map(|(low, range)| low..(low + range))
            .map(|n| map.apply(n))
            .min()
            .unwrap()
    )
}
