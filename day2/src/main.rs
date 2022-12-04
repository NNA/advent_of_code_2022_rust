use anyhow::{Error, Result};
use std::{fs, str::FromStr};

fn main() -> Result<()> {
    let res = part1("input.txt")?;
    println!("Solution for part1 is {res}");
    let res = part2("input.txt")?;
    println!("Solution for part2 is {res}");
    Ok(())
}

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone)]
enum RoundEnd {
    PlayerWin,
    Draw,
    PlayerLost,
}

#[derive(Debug)]
enum DayPart {
    Part1,
    Part2,
}

#[derive(Debug, Clone)]
struct Round {
    player_shape: Shape,
    opponent_shape: Shape,
}

fn part1(input: &str) -> Result<u32> {
    let content = fs::read_to_string(input)?;
    total_score(content, DayPart::Part1)
}

fn part2(input: &str) -> Result<u32> {
    let content = fs::read_to_string(input)?;
    total_score(content, DayPart::Part2)
}

fn total_score(content: String, part: DayPart) -> Result<u32> {
    let score: u32 = content
        .split("\n")
        .map(|line| match part {
            DayPart::Part1 => round_score(line.try_into().unwrap()).unwrap(),
            DayPart::Part2 => round_score(line.parse::<Round>().unwrap()).unwrap(),
        })
        .sum();
    Ok(score)
}

fn shape_needed_to_end(opponent_shape: Shape, round_end: RoundEnd) -> Shape {
    let player_shape = match (opponent_shape, round_end) {
        (os, RoundEnd::Draw) => os,
        (Shape::Rock, RoundEnd::PlayerWin) => Shape::Paper,
        (Shape::Paper, RoundEnd::PlayerWin) => Shape::Scissors,
        (Shape::Scissors, RoundEnd::PlayerWin) => Shape::Rock,
        (Shape::Rock, RoundEnd::PlayerLost) => Shape::Scissors,
        (Shape::Paper, RoundEnd::PlayerLost) => Shape::Rock,
        (Shape::Scissors, RoundEnd::PlayerLost) => Shape::Paper,
    };
    player_shape
}

impl TryFrom<&str> for Round {
    type Error = Error;
    fn try_from(r: &str) -> Result<Self> {
        let v = r.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(Round {
            opponent_shape: Shape::try_from(v[0])?,
            player_shape: Shape::try_from(v[1])?,
        })
    }
}

impl FromStr for Round {
    type Err = Error;
    fn from_str(r: &str) -> Result<Self> {
        let v = r.split_ascii_whitespace().collect::<Vec<_>>();
        let opponent_shape = Shape::try_from(v[0])?;
        let round_end = RoundEnd::try_from(v[1])?;
        let player_shape = shape_needed_to_end(opponent_shape.clone(), round_end);
        Ok(Round {
            opponent_shape,
            player_shape,
        })
    }
}

impl TryFrom<&str> for Shape {
    type Error = Error;
    fn try_from(s: &str) -> Result<Shape> {
        match s {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => panic!("invalid shape"),
        }
    }
}

impl TryFrom<&str> for RoundEnd {
    type Error = Error;
    fn try_from(s: &str) -> Result<RoundEnd> {
        match s {
            "X" => Ok(RoundEnd::PlayerLost),
            "Y" => Ok(RoundEnd::Draw),
            "Z" => Ok(RoundEnd::PlayerWin),
            _ => panic!("invalid round_end"),
        }
    }
}

fn round_score(round: Round) -> Result<u32> {
    let res = match round {
        // Player chooses Rock
        Round {
            player_shape: Shape::Rock,
            opponent_shape: Shape::Rock,
        } => 1 + 3,
        Round {
            player_shape: Shape::Rock,
            opponent_shape: Shape::Paper,
        } => 1 + 0,
        Round {
            player_shape: Shape::Rock,
            opponent_shape: Shape::Scissors,
        } => 1 + 6,
        // Player chooses Paper
        Round {
            player_shape: Shape::Paper,
            opponent_shape: Shape::Rock,
        } => 2 + 6,
        Round {
            player_shape: Shape::Paper,
            opponent_shape: Shape::Paper,
        } => 2 + 3,
        Round {
            player_shape: Shape::Paper,
            opponent_shape: Shape::Scissors,
        } => 2 + 0,
        // Player chooses Scissors
        Round {
            player_shape: Shape::Scissors,
            opponent_shape: Shape::Rock,
        } => 3 + 0,
        Round {
            player_shape: Shape::Scissors,
            opponent_shape: Shape::Paper,
        } => 3 + 6,
        Round {
            player_shape: Shape::Scissors,
            opponent_shape: Shape::Scissors,
        } => 3 + 3,
    };
    Ok(res)
}

#[test]
fn test_part1_sample() {
    let res = part1("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 15);
}

#[test]
fn test_part2_sample() {
    let res = part2("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 12);
}
