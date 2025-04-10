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

    const TEST_INPUT: &str = indoc! { r#""# };

    #[test]
    fn test_part_1() {
        let res = part_1(TEST_INPUT);
        assert_eq!(&res.to_string(), "-1");
    }

    #[test]
    fn test_part_2() {
        let res = part_2(TEST_INPUT);
        assert_eq!(&res.to_string(), "-1");
    }
}
