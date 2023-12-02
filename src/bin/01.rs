use nom::{branch::alt, multi::many1, IResult};

advent_of_code::solution!(1);

const NUMBER_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| parse_digits(line).expect("Failed to parse digits"))
        .sum();

    Some(sum)
}

pub fn parse_digits(line: &str) -> Option<u32> {
    let mut digits = vec![];
    for char in line.chars() {
        if is_digit(char) {
            digits.push(char);
        }
    }

    let kept = format!("{}{}", digits[0], digits[digits.len() - 1]);

    let result: u32 = kept.parse().ok()?;
    Some(result)
}

pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_line(mut line: &str) -> Result<u32, String> {
    let mut digits = vec![];
    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            digits.push(c)
        }

        if line[i..].starts_with("one") {
            digits.push('1');
        } else if line[i..].starts_with("two") {
            digits.push('2');
        } else if line[i..].starts_with("three") {
            digits.push('3');
        } else if line[i..].starts_with("four") {
            digits.push('4');
        } else if line[i..].starts_with("five") {
            digits.push('5');
        } else if line[i..].starts_with("six") {
            digits.push('6');
        } else if line[i..].starts_with("seven") {
            digits.push('7');
        } else if line[i..].starts_with("eight") {
            digits.push('8');
        } else if line[i..].starts_with("nine") {
            digits.push('9');
        }
    }

    let number = format!("{}{}", digits[0], digits[digits.len() - 1])
        .as_str()
        .parse::<u32>()
        .map_err(|e| e.to_string())?;

    Ok(number)
}

fn parse_digit_char(line: &str) -> IResult<&str, &str> {
    nom::character::complete::digit1(line)
}

fn parse_digit_word(line: &str) -> IResult<&str, &str> {
    nom::branch::alt((
        nom::bytes::complete::tag("one"),
        nom::bytes::complete::tag("two"),
        nom::bytes::complete::tag("three"),
        nom::bytes::complete::tag("four"),
        nom::bytes::complete::tag("five"),
        nom::bytes::complete::tag("six"),
        nom::bytes::complete::tag("seven"),
        nom::bytes::complete::tag("eight"),
        nom::bytes::complete::tag("nine"),
    ))(line)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| parse_line(line).expect("Failed to parse digits"))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
