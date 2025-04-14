use std::{collections::HashSet, fmt::Display, ops::Index};

type Direction = (isize, isize); // (delta_i, delta_j)

struct Map {
    grid: Vec<char>,
    cols: usize,
    guard_pos: usize,
    guard_dir: Direction,
}

impl Map {
    pub fn from(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let cols = lines.peek().unwrap().len();
        let grid: Vec<char> = lines
            .map(|l| l.chars().collect::<Vec<_>>())
            .flatten()
            .collect();

        let Some(guard_pos) = grid.iter().position(|&c| c == '^') else {
            panic!("Guard not found!")
        };

        Self {
            grid,
            cols,
            guard_pos,
            guard_dir: (-1, 0),
        }
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.grid.len() / self.cols
    }
}

impl Index<usize> for Map {
    type Output = [char];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.cols;
        let end = start + self.cols;
        &self.grid[start..end]
    }
}

fn simulate_guard(map: &mut Map) -> usize {
    let (rows, cols) = (map.rows(), map.cols);
    let (mut gi, mut gj, mut gdir) = (
        (map.guard_pos / cols) as isize,
        (map.guard_pos % cols) as isize,
        map.guard_dir,
    );

    let is_inbounds =
        |i: isize, j: isize| i >= 0 && i < rows as isize && j >= 0 && j < cols as isize;

    // Save the guard's visited positions in a hashset to deny repeats
    let mut visited = HashSet::<(isize, isize)>::new();

    while is_inbounds(gi, gj) {
        if map[gi as usize][gj as usize] == '#' {
            gi -= gdir.0;
            gj -= gdir.1;
            // 90 deg rotation of a 2D vector.
            gdir = (gdir.1, -1 * gdir.0);
            continue;
        }
        visited.insert((gi, gj));
        gi += gdir.0;
        gj += gdir.1;
    }
    visited.len()
}

pub fn part_1(input: &str) -> Box<dyn Display> {
    Box::new(simulate_guard(&mut Map::from(input)))
}

pub fn part_2(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! { r#"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "# };

    #[test]
    fn test_part_1() {
        let res = part_1(TEST_INPUT);
        assert_eq!(&res.to_string(), "41");
    }

    #[test]
    fn test_part_2() {
        let res = part_2(TEST_INPUT);
        assert_eq!(&res.to_string(), "-1");
    }
}
