use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
struct MulOperation(i64, i64);

impl MulOperation {
    pub fn run(&self) -> i64 {
        self.0.checked_mul(self.1).unwrap()
    }
}

#[derive(Debug)]
struct ParseMulOperationError;

impl FromStr for MulOperation {
    type Err = ParseMulOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .strip_prefix("mul(")
            .and_then(|s| s.strip_suffix(")"))
            .and_then(|s| s.split_once(","))
            .ok_or(ParseMulOperationError)?;

        let a = a.parse::<i64>().map_err(|_| ParseMulOperationError)?;
        let b = b.parse::<i64>().map_err(|_| ParseMulOperationError)?;
        Ok(Self(a, b))
    }
}

#[derive(Debug)]
enum Operation {
    Mul(MulOperation),
    Do,
    Dont,
}

#[derive(Debug)]
struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;

        Ok(match s {
            "do()" => Do,
            "don't()" => Dont,
            s => Mul(s.parse().map_err(|_| ParseOperationError)?),
        })
    }
}

fn find_closest(s: &str, pats: &[&str]) -> Option<usize> {
    pats.iter()
        .filter_map(|p| s.find(p))
        .fold(None, |acc, i| Some(acc.unwrap_or(usize::MAX).min(i)))
}

pub fn part_1(input: &str) -> Box<dyn Display> {
    let mut input = input;
    let mut sum = 0;

    while let Some(start) = input.find("mul(") {
        if let Some(end) = input[start..].find(')') {
            let end = end + start;

            sum += &input[start..=end]
                .parse::<MulOperation>()
                .map_or(0, |m| m.run());

            input = &input[start + 3..];
        } else {
            break;
        }
    }
    Box::new(sum)
}

pub fn part_2(input: &str) -> Box<dyn Display> {
    let mut input = input;
    let mut sum = 0;
    let mut enable = true;

    while let Some(start) = find_closest(input, &["mul", "do", "don't"]) {
        if let Some(end) = input[start..].find(')') {
            let end = end + start;

            use Operation::*;
            match &input[start..=end].parse::<Operation>().ok() {
                Some(Do) => enable = true,
                Some(Dont) => enable = false,
                Some(Mul(m)) => {
                    if enable {
                        sum += m.run()
                    }
                }
                _ => (),
            }

            input = &input[start + 3..];
        } else {
            break;
        }
    }
    Box::new(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! { r#"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "# };

    #[test]
    fn test_part_1() {
        let res = part_1(TEST_INPUT);
        assert_eq!(&res.to_string(), "161");
    }

    #[test]
    fn test_part_2() {
        let res = part_2(TEST_INPUT);
        assert_eq!(&res.to_string(), "48");
    }
}
