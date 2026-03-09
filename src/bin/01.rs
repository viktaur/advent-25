advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let rotations = input.lines().map(|l| Rotation::from_str(l).unwrap());
    // Keeps track of how many times it's zero
    let mut n_zero = 0;
    // Keeps track of the current value;
    let mut curr_val = 50;
    for r in rotations {
        curr_val = r.apply(curr_val);
        if curr_val == 0 {
            n_zero += 1;
        }
    }

    Some(n_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations = input.lines().map(|l| Rotation::from_str(l).unwrap());
    // Keeps track of how many times it has pointed to zero.
    let mut n_zero = 0;
    // Keeps track of the current value.
    let mut curr_val = 50;
    for r in rotations {
        let (new_val, visits_zero) = r.apply_with_zero_count(curr_val);
        curr_val = new_val;
        n_zero += visits_zero;
    }

    Some(n_zero)
}

#[derive(Debug)]
enum Rotation {
    Left(u64),
    Right(u64)
}

impl Rotation {
    fn from_str(s: &str) -> Option<Self> {
        let val: u64 = s[1..].parse().ok()?;

        if s.starts_with("L") {
            Some(Rotation::Left(val))
        } else if s.starts_with("R") {
            Some(Rotation::Right(val))
        } else {
            None
        }
    }

    fn apply(&self, val: u64) -> u64 {
        match self {
            Rotation::Left(x) => (val + 100 - x % 100) % 100,
            Rotation::Right(x) => (val + x) % 100,
        }
    }

    fn apply_with_zero_count(&self, val: u64) -> (u64, u64) {
        let zero_count = match self {
            Rotation::Left(x) => ((100 - val) % 100 + x) / 100,
            Rotation::Right(x) => (val + x) / 100,
        };
        (self.apply(val), zero_count)
    }
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
        assert_eq!(result, Some(6));
    }
}
