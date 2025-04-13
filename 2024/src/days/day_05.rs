use std::fmt::Display;

pub fn part_1(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}

pub fn part_2(_input: &str) -> Box<dyn Display> {
    Box::new(0)
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
