use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut fresh_count = 0;

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        if let Some((s, e)) = line.split_once("-") {
            ranges.push((s.parse().ok()?, e.parse().ok()?));
        }
    }

    while let Some(line) = lines.next() {
        let n: u64 = line.parse().ok()?;

        // Check over all the ranges
        for (s, e) in ranges.iter() {
            if (s..=e).contains(&&n) {
                fresh_count += 1;
                break;
            }
        }
    }

    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    // let mut lines = input.lines();
    // let mut elements = HashSet::new();

    // while let Some(line) = lines.next() {
    //     if line.is_empty() {
    //         break;
    //     }

    //     if let Some((s, e)) = line.split_once("-") {
    //         let (s, e): (u64, u64) = (s.parse().ok()?, e.parse().ok()?);
    //         (s..=e).for_each(|n| { elements.insert(n); })
    //     }
    // }

    // Some(elements.len() as u64)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
