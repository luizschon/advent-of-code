use std::{collections::HashSet, fmt::Display};

type Map = Vec<Vec<char>>; // 2D char grid
type Pos = (usize, usize); // (i, j)

struct Direction(isize, isize); // (delta_i, delta_j)

impl Direction {
    #[inline]
    pub fn rotate_right(&mut self) {
        let temp = self.0;
        self.0 = self.1;
        self.1 = -1 * temp;
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self(-1, 0)
    }
}

struct Guard {
    pos: Pos,
    direction: Direction,
}

struct GuardPosError;

impl Guard {
    #[inline]
    pub fn curr_pos(&self) -> Pos {
        self.pos
    }

    pub fn step(&mut self, map: &Map) -> Result<Pos, GuardPosError> {
        let (rows, cols) = (map.len(), map[0].len());
        let next_pos = |(i, j): Pos, dir: &Direction| {
            Ok((
                i.checked_add_signed(dir.0)
                    .and_then(|r| (r < rows).then_some(r))
                    .ok_or(GuardPosError)?,
                j.checked_add_signed(dir.1)
                    .and_then(|c| (c < cols).then_some(c))
                    .ok_or(GuardPosError)?,
            ))
        };

        let next = next_pos(self.pos, &self.direction)?;

        if map[next.0][next.1] == '#' {
            self.direction.rotate_right();
        }
        self.pos = next_pos(self.pos, &self.direction)?;
        Ok(self.pos)
    }
}

impl From<&Map> for Guard {
    fn from(map: &Map) -> Self {
        let mut pos = (0, 0);
        for (i, row) in map.iter().enumerate() {
            if let Some(j) = row.iter().position(|&c| c == '^') {
                pos = (i, j);
                break;
            }
        }
        Self {
            pos,
            direction: Direction::default(),
        }
    }
}

pub fn part_1(input: &str) -> Box<dyn Display> {
    let (map, mut guard) = parse(input);
    let mut visited_pos = HashSet::<Pos>::new();
    visited_pos.insert(guard.curr_pos());

    while let Ok(pos) = guard.step(&map) {
        visited_pos.insert(pos);
    }
    Box::new(visited_pos.len())
}

pub fn part_2(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}

fn parse(input: &str) -> (Map, Guard) {
    let map = input.lines().map(|s| s.chars().collect()).collect();
    let guard = Guard::from(&map);
    (map, guard)
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
        assert_eq!(&res.to_string(), "6");
    }
}
