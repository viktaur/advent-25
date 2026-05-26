use std::ops::DerefMut;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        Grid::from_input(input).count_accessible_paper_rolls()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from_input(input);
    let mut count = 0;

    loop {
        let locs = grid.accessible_roll_locs();
        if locs.len() > 0 {
            grid.remove_locs(&locs);
            count += locs.len() as u64;
        } else {
            break;
        }
    }

    Some(count)
}

#[derive(Copy, Clone, Debug)]
enum CellKind {
    PaperRoll,
    Empty
}

impl CellKind {
    fn from_char(c: char) -> Self {
        match c {
            '@' => CellKind::PaperRoll,
            '.' => CellKind::Empty,
            _ => panic!("Invalid cell kind: {}", c),
        }
    }
}

#[derive(Clone, Debug)]
struct Grid(Vec<Vec<CellKind>>);

impl Grid {
    fn from_input(input: &str) -> Self {
        Self(
            input.lines().map(
                |l| l.chars().map(|c| CellKind::from_char(c)).collect()
            ).collect()
        )
    }

    fn count_accessible_paper_rolls(&self) -> u64 {
        self.0.iter().enumerate()
            .map(|(i, row)| row.iter().enumerate()
                .filter(|(j, _)| self.is_accessible(i, *j))
                .count() as u64
            )
            .sum()
    }

    fn accessible_roll_locs(&self) -> Vec<(usize, usize)> {
        let mut locs = vec![];

        for (i, row) in self.0.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if self.is_accessible(i, j) {
                    locs.push((i, j))
                }
            }
        }

        locs
    }

    fn remove_locs(&mut self, locs: &[(usize, usize)]) {
        let cells = self.0.clone();

        for (i, row) in cells.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if locs.contains(&(i, j)) {
                    self.0[i][j] = CellKind::Empty
                }
            }
        }
    }

    fn neighbour(&self, i: usize, j: usize, di: isize, dj: isize) -> Option<&CellKind> {
        let ni = i.checked_add_signed(di)?;
        let nj = j.checked_add_signed(dj)?;
        self.0.get(ni)?.get(nj)
    }

    fn is_accessible(&self, i: usize, j: usize) -> bool {
        if let CellKind::Empty = self.0[i][j] {
            return false;
        }
        const OFFSETS: [(isize, isize); 8] = [
            (-1, 0), (1, 0), (0, -1), (0, 1),
            (-1, -1), (-1, 1), (1, -1), (1, 1),
        ];
        let roll_neighbours = OFFSETS
            .iter()
            .filter(|&&(di, dj)| matches!(
                self.neighbour(i, j, di, dj), Some(CellKind::PaperRoll)
            ))
            .count();
        roll_neighbours < 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
