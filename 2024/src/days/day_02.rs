use std::fmt::Display;

fn is_safe(report: &[u64]) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let increasing = report[0] > report[1];
    report.windows(2).all(|w| {
        let (a, b) = (w[0], w[1]);
        a != b && a.abs_diff(b) <= 3 && (a > b) == increasing
    })
}

fn is_safe_damped(report: &[u64]) -> bool {
    is_safe(report)
        || (0..report.len()).any(|i| {
            is_safe(
                &report[..i]
                    .iter()
                    .chain(report[i + 1..].iter())
                    .copied()
                    .collect::<Vec<_>>(),
            )
        })
}

pub fn part_1(input: &str) -> Box<dyn Display> {
    let reports = dbg!(parse(input));

    Box::new(
        reports
            .iter()
            .map(|r| is_safe(r))
            .fold(0, |acc, b| acc + b as u64),
    )
}

pub fn part_2(input: &str) -> Box<dyn Display> {
    let reports = dbg!(parse(input));

    Box::new(
        reports
            .iter()
            .map(|r| is_safe_damped(r))
            .fold(0, |acc, b| acc + b as u64),
    )
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! { r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "# };

    #[test]
    fn test_part_1() {
        let res = part_1(TEST_INPUT);
        assert_eq!(&res.to_string(), "2");
    }

    #[test]
    fn test_part_2() {
        let res = part_2(TEST_INPUT);
        assert_eq!(&res.to_string(), "4");
    }
}
