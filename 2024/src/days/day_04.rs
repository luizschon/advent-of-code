use std::{fmt::Display, iter, ops::Index};

#[derive(Debug)]
struct CharTable {
    data: Vec<char>,
    cols: usize,
}

impl CharTable {
    pub fn new(data: Vec<char>, cols: usize) -> Self {
        assert!(data.len() % cols == 0, "Data len must be divisible by cols");
        Self { data, cols }
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.data.len() / self.cols
    }
}

impl Index<usize> for CharTable {
    type Output = [char];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}

fn count_horizontal(table: &CharTable, pos: (usize, usize)) -> u32 {
    let (i, j) = pos;
    let mut total = 0;

    if table.cols() - j >= 4 {
        total += (&table[i][j..j + 4].iter().collect::<String>() == "XMAS") as u32;
    }
    if j >= 3 {
        total += (&table[i][j - 3..=j].iter().rev().collect::<String>() == "XMAS") as u32;
    }
    total
}

fn count_vertical(table: &CharTable, pos: (usize, usize)) -> u32 {
    let (i, j) = pos;
    let mut total = 0;

    if table.rows() - i >= 4 {
        total += ((i..i + 4).map(|idx| table[idx][j]).collect::<String>() == "XMAS") as u32;
    }
    if i >= 3 {
        total += ((i - 3..=i)
            .map(|idx| table[idx][j])
            .rev()
            .collect::<String>()
            == "XMAS") as u32;
    }
    total
}

fn count_diagonal(table: &CharTable, pos: (usize, usize)) -> u32 {
    let (i, j) = pos;
    let mut total = 0;

    if table.rows() - i >= 4 {
        if table.cols() - j >= 4 {
            total += ((0..4).map(|c| table[i + c][j + c]).collect::<String>() == "XMAS") as u32;
        }
        if j >= 3 {
            total += ((0..4).map(|c| table[i + c][j - c]).collect::<String>() == "XMAS") as u32;
        }
    }
    if i >= 3 {
        if table.cols() - j >= 4 {
            total += ((0..4).map(|c| table[i - c][j + c]).collect::<String>() == "XMAS") as u32;
        }
        if j >= 3 {
            total += ((0..4).map(|c| table[i - c][j - c]).collect::<String>() == "XMAS") as u32;
        }
    }
    total
}

pub fn part_1(input: &str) -> Box<dyn Display> {
    let strings = parse(input);

    let pos: Vec<(usize, usize)> = strings
        .iter()
        .enumerate()
        .map(|(i, s)| {
            iter::zip(
                iter::repeat(i),
                s.char_indices().filter(|(_, c)| *c == 'X').map(|(j, _)| j),
            )
            .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let table = CharTable::new(
        strings
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .flatten()
            .collect(),
        strings[0].len(),
    );

    Box::new(
        pos.iter()
            .map(|&p| {
                count_horizontal(&table, p) + count_vertical(&table, p) + count_diagonal(&table, p)
            })
            .sum::<u32>(),
    )
}

pub fn part_2(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}

fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect()
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
        assert_eq!(&res.to_string(), "-1");
    }
}
