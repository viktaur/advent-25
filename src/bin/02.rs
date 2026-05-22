use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input.lines().join("").split(',')
            .map(|segment| segment.split_once("-").unwrap())
            .map(|(start, end)| invalid_in_range(
                start.parse().unwrap(), end.parse().unwrap(), id_is_valid
            ))
            .flatten()
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input.lines().join("").split(',')
            .map(|segment| segment.split_once("-").unwrap())
            .map(|(start, end)| invalid_in_range(
                start.parse().unwrap(), end.parse().unwrap(), id_is_valid_2
            ))
            .flatten()
            .sum()
    )
}

fn invalid_in_range(start: u64, end: u64, valid_fn: impl Fn(u64) -> bool) -> Vec<u64> {
    (start..=end)
        .filter(|&n| !valid_fn(n))
        .collect()
}

fn id_is_valid(digit: u64) -> bool {
    let s = digit.to_string();
    let mut chars = s.chars();
    let len = s.len();

    // Always valid if odd
    if len % 2 == 1 {
        return true;
    }

    let mut a = vec![];
    let mut b = vec![];
    let mut i = 0;

    while let Some(c) = chars.next() {
        if i >= len / 2 {
            b.push(c);
        } else {
            a.push(c);
        }

        i += 1;
    }

    a != b
}

// Uses factor decomposition for window sizes (if len is 12, try with window sizes
// of 6 (at least twice, 6 + 6 = 12), 4, 3, 2, 1). Basically all factors of n except n.
fn id_is_valid_2(digit: u64) -> bool {
    let s = digit.to_string().chars().collect_vec();
    let len = s.len();

    let window_sizes = factors(len);
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut diff = false;

    for n in window_sizes {
        while i < len {
            if s[i] != s[j] {
                // Difference found, so this window size won't give us an invalid number.
                // We can stop here and try with next one.
                diff = true;
                break;
            }

            i += 1;
            j = (j + 1) % n;
        }

        if !diff { // We have found an invalid num (e.g. 123123123)
            return false;
        }

        // Otherwise we reset the flags and try again with the next smaller window size
        diff = false;
        i = 0;
        j = 0;
    }

    // If nothing has been found the number is valid
    true
}

fn factors(n: usize) -> Vec<usize> {
    let mut factors: HashSet<usize> = HashSet::new();
    let mut i: usize = 1;

    while i.pow(2) <= n {
        if n % i == 0 {
            factors.insert(i);
            factors.insert(n / i);
        }
        i += 1;
    }

    // Ensure the number itself is not present (window size needs to be at least half in
    // order to appear twice).
    factors.remove(&n);
    factors.into_iter().sorted_by(|a, b| b.cmp(a)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
