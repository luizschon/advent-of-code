use std::{collections::HashMap, fmt::Display};

type OrderingRules = HashMap<u32, Vec<u32>>;
type Updates = Vec<Vec<u32>>;

fn is_ordered(ordering: &OrderingRules, update: &Vec<u32>) -> bool {
    let idx_map: HashMap<_, _> = update.iter().enumerate().map(|(i, v)| (*v, i)).collect();

    idx_map.iter().all(|(page, p_idx)| {
        ordering.get(page).is_none_or(|rules| {
            rules
                .iter()
                .all(|p| idx_map.get(p).is_none_or(|r_idx| r_idx > p_idx))
        })
    })
}

pub fn part_1(input: &str) -> Box<dyn Display> {
    let (ordering, updates) = parse(input);
    Box::new(
        updates
            .iter()
            .filter_map(|u| is_ordered(&ordering, u).then(|| u[u.len() / 2]))
            .sum::<u32>(),
    )
}

pub fn part_2(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}

fn parse(input: &str) -> (OrderingRules, Updates) {
    let mut blocks = input.split("\n\n");
    let mut ordering = HashMap::<u32, Vec<u32>>::new();

    blocks.next().unwrap().lines().for_each(|l| {
        let [a, b] = l
            .split('|')
            .map(|d| d.parse().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        ordering
            .entry(a)
            .and_modify(|v| v.push(b))
            .or_insert(vec![b]);
    });

    let updates = blocks
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|d| d.parse().unwrap()).collect())
        .collect();

    (ordering, updates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! { r#"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "# };

    #[test]
    fn test_part_1() {
        let res = part_1(TEST_INPUT);
        assert_eq!(&res.to_string(), "143");
    }

    #[test]
    fn test_part_2() {
        let res = part_2(TEST_INPUT);
        assert_eq!(&res.to_string(), "-1");
    }
}
