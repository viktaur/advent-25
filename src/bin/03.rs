use itertools::Itertools;
use itertools::EitherOrBoth;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| find_max_joltage(l))
            .flatten()
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| find_max_joltage_2(l))
            .flatten()
            .sum()
    )
}

fn find_max_joltage(bank: &str) -> Option<u64> {
    let bank_chars: Vec<char> = bank.chars().collect();
    let mut max_joltage: Option<u64> = None;

    for i in 0..bank_chars.len() {
        for j in i+1..bank_chars.len() {
            let joltage =
                bank_chars.get(i)?.to_digit(10)? as u64 * 10 +
                bank_chars.get(j)?.to_digit(10)? as u64;

            max_joltage = Some(joltage.max(max_joltage.unwrap_or(0)))
        }
    }

    max_joltage
}

type DigitNodeMap = HashMap<usize, DigitNode>;

struct DigitNode {
    pos: usize,
    value: u32,
    left: Option<usize>,
    right: Option<usize>,
}

impl DigitNode {
    fn next
}

fn find_max_joltage_2(bank: &str) -> Option<u64> {
    let digits = bank.chars().map(|c| c.to_digit(10).unwrap_or(0));
    let sorted_stacks = construct_stack_tree(digits);
    let mut selected_digits = HashSet::new();
    find_next_digit(&sorted_stacks, &mut selected_digits, 12);

    Some(
        selected_digits
            .iter()
            .sorted_by(|(i_a, _), (i_b, _)| i_a.cmp(i_b)) // sort by original position
            .map(|(_, d)| *d)
            .fold(0, |acc, d| acc * 10 + d as u64)
    )
}

fn construct_stack_tree(digits: impl Iterator<Item = u32>) -> (DigitNodeMap, usize) {
    let mut stacks: Vec<Vec<(usize, u32)>> = Vec::new();

    for (i, d) in digits.enumerate() {
        match stacks.last_mut() {
            Some(stack) => match stack.last() {
                Some((_, e)) if d > *e => stacks.push(vec![(i, d)]), // create a new stack with d
                _ => stack.push((i, d)),                             // append to the existing one
            }
            _ => stacks.push(vec![(i, d)]),
        }
    }

    stacks.sort_by(|a, b| cmp_stack(b, a)); // Sort DESC
    stacks
}

fn cmp_stack(stack_a: &Vec<(usize, u32)>, stack_b: &Vec<(usize, u32)>) -> Ordering {
    for pair in stack_a.iter().zip_longest(stack_b.iter()) {
        let (va, vb) = match pair {
            EitherOrBoth::Both(a, b) => (a, b),
            EitherOrBoth::Left(_) => return Ordering::Less,
            EitherOrBoth::Right(_) => return Ordering::Greater,
        };

        if va != vb {
            return va.1.cmp(&vb.1);
        }
    }

    Ordering::Equal
}

fn find_next_digit(
    sorted_stacks: &Vec<Vec<(usize, u32)>>,
    selected_digits: &mut HashSet<(usize, u32)>,
    n: u32
) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
