advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .flat_map(find_max_joltage)
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .flat_map(find_max_joltage_12)
            .sum()
    )
}

fn find_max_joltage(line: &str) -> Option<u64> {
    let digits: Vec<u64> = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();

    let (pos, &first) = digits[0..digits.len()-1]
        .iter()
        .enumerate()
        .max_by_key(|(_, d)| *d)?;
    let &second = digits[pos+1..].iter().max()?;

    Some(first * 10 + second)
}

fn find_max_joltage_12(line: &str) -> Option<u64> {
    let digits: Vec<(usize, u64)> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .enumerate()
        .collect();
    let mut left = 12;
    let mut min_pos = 0;
    let mut output = Vec::new();

    while left > 0 {
        let selection = &digits[min_pos..=(digits.len()-left)];
        let (_, d) = selection.iter().max_by_key(|(_, d)| d)?;
        // Get the position of the first digit matching d.
        let (pos, _) = selection.iter().find(|(_, e)| e == d)?;

        output.push(d);
        min_pos = pos + 1;
        left -= 1;
    }

    Some(output.into_iter().fold(0, |acc, e: &u64| acc * 10 + e))
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
