use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::str::FromStr;

advent_of_code::solution!(2);

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Default, Debug)]
struct Set {
    pub red: Option<usize>,
    pub green: Option<usize>,
    pub blue: Option<usize>,
}

impl Set {
    fn new(input: Vec<(usize, &str)>) -> Self {
        let mut out = Self {
            red: None,
            green: None,
            blue: None,
        };

        for (number, color) in input {
            match color {
                "red" => out.red = Some(number),
                "green" => out.green = Some(number),
                "blue" => out.blue = Some(number),
                _ => panic!("Unknown color"),
            }
        }
        out
    }

    fn power(&self) -> usize {
        let mut out = 1;

        if let Some(red) = self.red {
            out *= red;
        }

        if let Some(green) = self.green {
            out *= green;
        }

        if let Some(blue) = self.blue {
            out *= blue;
        }

        out
    }

    pub fn is_impossible(&self) -> bool {
        if let Some(red) = self.red {
            if red > MAX_RED {
                dbg!(red);
                return true;
            }
        }

        if let Some(green) = self.green {
            if green > MAX_GREEN {
                dbg!(green);
                return true;
            }
        }

        if let Some(blue) = self.blue {
            if blue > MAX_BLUE {
                dbg!(blue);
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_valid(&self) -> bool {
        for set in self.sets.iter() {
            if set.is_impossible() {
                return false;
            }
        }
        dbg!(&self);
        true
    }

    pub fn fewest_possible_cubes(&self) -> Set {
        let mut out = Set::default();
        for set in self.sets.iter() {
            let red = set.red.unwrap_or(0);
            if red > out.red.unwrap_or(0) {
                out.red = Some(red);
            }

            let green = set.green.unwrap_or(0);
            if green > out.green.unwrap_or(0) {
                out.green = Some(green);
            }

            let blue = set.blue.unwrap_or(0);
            if blue > out.blue.unwrap_or(0) {
                out.blue = Some(blue);
            }
        }

        out
    }
}

fn parse_game(line: &str) -> IResult<&str, Game> {
    let (s, id) = parse_game_id(line)?;
    let (s, _) = tag(": ")(s)?;
    let (s, sets) = parse_sets(s)?;

    let game = Game { id, sets };

    Ok((s, game))
}

fn parse_sets(input: &str) -> IResult<&str, Vec<Set>> {
    let (s, sets) = separated_list1(tag("; "), parse_set)(input)?;
    Ok((s, sets))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let (s, colors) = separated_list1(tag(", "), parse_colour)(input)?;
    let set = Set::new(colors);

    Ok((s, set))
}

fn parse_colour(s: &str) -> IResult<&str, (usize, &str)> {
    separated_pair(
        map_res(digit1, usize::from_str),
        tag(" "),
        alt((tag("red"), tag("blue"), tag("green"))),
    )(s)
}

fn parse_game_id(line: &str) -> IResult<&str, usize> {
    preceded(tag("Game "), map_res(digit1, usize::from_str))(line)
}

pub fn part_one(input: &str) -> Option<u32> {
    let valid_games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .filter(|game| game.is_valid())
        .collect();

    valid_games
        .iter()
        .map(|game| game.id)
        .sum::<usize>()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Game> = input
        .lines()
        .map(|l| parse_game(l).expect("Failed to parse game").1)
        .collect();

    let sets = games
        .iter()
        .map(|game| game.fewest_possible_cubes())
        .collect::<Vec<_>>();

    let result = sets.iter().map(|set| set.power()).sum::<usize>();

    Some(result as u32)
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
