use std::{fmt::Display, ops::Index};

type Point = (usize, usize); // Point = (i, j)
type Direction = (isize, isize); // Direction = (delta i, delta j)

#[derive(Debug)]
struct CharGrid {
    data: Vec<char>,
    cols: usize,
}

impl CharGrid {
    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.data.len() / self.cols
    }
}

impl Index<usize> for CharGrid {
    type Output = [char];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}

fn matches_pattern(grid: &CharGrid, (i, j): Point, (di, dj): Direction, pattern: &str) -> bool {
    pattern.char_indices().all(|(k, ch)| {
        let row = i.checked_add_signed(di * k as isize);
        let col = j.checked_add_signed(dj * k as isize);

        matches!((row, col), (Some(r), Some(c)) if r < grid.rows() && c < grid.cols() && grid[r][c] == ch)
    })
}

pub fn part_1(input: &str) -> Box<dyn Display> {
    let grid = parse(input);
    #[rustfmt::skip]
    let directions = [
        (-1, -1), (-1,  0), (-1,  1),
        ( 0, -1),           ( 0,  1),
        ( 1, -1), ( 1,  0), ( 1,  1),
    ];

    Box::new(
        itertools::iproduct!(0..grid.rows(), 0..grid.cols(), directions)
            .filter(|&(i, j, d)| grid[i][j] == 'X' && matches_pattern(&grid, (i, j), d, "XMAS"))
            .count(),
    )
}

pub fn part_2(input: &str) -> Box<dyn Display> {
    let grid = parse(input);
    let cross_directions = ((1, 1), (1, -1));
    let patterns = ["MAS", "SAM"];

    Box::new(
        itertools::iproduct!(1..grid.rows() - 1, 1..grid.cols() - 1, patterns, patterns)
            .filter(|&(i, j, p1, p2)| {
                grid[i][j] == 'A'
                    && matches_pattern(&grid, (i - 1, j - 1), cross_directions.0, p1)
                    && matches_pattern(&grid, (i - 1, j + 1), cross_directions.1, p2)
            })
            .count(),
    )
}

fn parse(input: &str) -> CharGrid {
    let mut cols = 0;
    let data = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let data = l.trim().chars().collect::<Vec<_>>();
            cols = data.len();
            data
        })
        .flatten()
        .collect();

    CharGrid { data, cols }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! { r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "# };

    #[test]
    fn test_part_1() {
        let res = part_1(TEST_INPUT);
        assert_eq!(&res.to_string(), "18");
    }

    #[test]
    fn test_part_2() {
        let res = part_2(TEST_INPUT);
        assert_eq!(&res.to_string(), "9");
    }
}
