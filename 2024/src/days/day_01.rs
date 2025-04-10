use std::collections::HashMap;
use std::fmt::Display;

pub fn part_1(input: &str) -> Box<dyn Display> {
    let (mut first_list, mut second_list) = parse(input);
    first_list.sort();
    second_list.sort();

    Box::new(
        std::iter::zip(first_list.into_iter(), second_list.into_iter())
            .map(|(a, b)| a.abs_diff(b))
            .sum::<u64>(),
    )
}

pub fn part_2(input: &str) -> Box<dyn Display> {
    let (first, second) = parse(input);
    let mut occurency_map = HashMap::<u64, u64>::new();

    for v in second {
        occurency_map
            .entry(v)
            .and_modify(|o| {
                *o += 1;
            })
            .or_insert(1);
    }

    Box::new(
        first
            .into_iter()
            .map(|k| match occurency_map.get(&k) {
                Some(count) => count * k,
                None => 0,
            })
            .sum::<u64>(),
    )
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (mut first, mut second) = (vec![], vec![]);

    for line in input.lines().map(str::trim) {
        if line.is_empty() {
            continue;
        }
        let mut nums = line.split_whitespace().map(|n| n.parse().unwrap());
        first.push(nums.next().unwrap());
        second.push(nums.next().unwrap());
    }
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {
    "3   4
     4   3
     2   5
     1   3
     3   9
     3   3"
    };

    #[test]
    fn test_part_1() {
        let res = part_1(TEST_INPUT);
        assert_eq!(&res.to_string(), "11");
    }

    #[test]
    fn test_part_2() {
        let res = part_2(TEST_INPUT);
        assert_eq!(&res.to_string(), "31");
    }
}
