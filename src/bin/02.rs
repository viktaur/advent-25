use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input.lines().join("").split(',')
            .map(|segment| segment.split_once("-").unwrap())
            .map(|(start, end)| invalid_in_range(start.parse().unwrap(), end.parse().unwrap()))
            .flatten()
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn invalid_in_range(start: u64, end: u64) -> Vec<u64> {
    (start..=end)
        .filter(|&n| !id_is_valid(n))
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
        assert_eq!(result, None);
    }
}
