use anyhow::Result;
use std::{collections::HashMap, fs};

#[derive(PartialEq)]
pub enum CrateMover {
    CM9000,
    CM9001,
}

fn main() -> Result<()> {
    let res = part1("input.txt")?;
    println!("Solution for part1 is {res}");
    let res = part2("input.txt")?;
    println!("Solution for part2 is {res}");
    Ok(())
}

fn part1(input: &str) -> Result<String> {
    let content = fs::read_to_string(input)?;
    let parts: Vec<&str> = content.split("\n\n").collect();

    let mut crates: HashMap<usize, Vec<char>> = HashMap::new();
    let crates_count = load_crates(&mut crates, parts[0]);
    move_crates(&mut crates, parts[1], CrateMover::CM9000);
    let res = top_crates(&mut crates, crates_count.try_into().unwrap());

    Ok(res)
}

fn part2(input: &str) -> Result<String> {
    let content = fs::read_to_string(input)?;
    let parts: Vec<&str> = content.split("\n\n").collect();

    let mut crates: HashMap<usize, Vec<char>> = HashMap::new();
    let crates_count = load_crates(&mut crates, parts[0]);
    move_crates(&mut crates, parts[1], CrateMover::CM9001);
    let res = top_crates(&mut crates, crates_count.try_into().unwrap());

    Ok(res)
}

fn load_crates(crates: &mut HashMap<usize, Vec<char>>, raw_crates: &str) -> u32 {
    let mut col_count: u32 = 0;

    for (line_index, line) in raw_crates.rsplit("\n").enumerate() {
        if line_index == 0 {
            col_count = line
                .trim_end()
                .chars()
                .rev()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap();
        } else {
            for (index, val) in line.chars().skip(1).step_by(4).enumerate() {
                let new_index = index + 1;
                if let Some(v) = crates.get_mut(&new_index) {
                    if val != ' ' {
                        v.push(val);
                    }
                } else {
                    crates.insert(new_index, vec![val]);
                }
            }
        }
    }
    col_count + 1
}

fn move_crates(crates: &mut HashMap<usize, Vec<char>>, raw_moves: &str, mover_version: CrateMover) {
    for line in raw_moves.split("\n") {
        let movements = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .collect::<Vec<&str>>();
        let moves_count: usize = movements[0].parse().unwrap();
        let src_col: usize = movements[1].parse().unwrap();
        let dst_col: usize = movements[2].parse().unwrap();

        let mut stack: Vec<char> = vec![];

        if let Some(col) = crates.get_mut(&src_col) {
            for _mov in 0..moves_count {
                stack.push(col.pop().unwrap());
            }
        }

        if mover_version == CrateMover::CM9000 {
            stack.reverse();
        }

        if let Some(col) = crates.get_mut(&dst_col) {
            for _mov in 0..moves_count {
                // println!("pushed dest");
                col.push(stack.pop().unwrap());
            }
        }
    }
}

fn top_crates(crates: &mut HashMap<usize, Vec<char>>, col_count: usize) -> String {
    let mut res = String::new();

    for col_index in 1..col_count {
        if let Some(v) = crates.get_mut(&col_index) {
            res.push(v.pop().unwrap());
        }
    }

    res
}

#[test]
fn test_part1_sample() {
    let res = part1("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "CMZ");
}

#[test]
fn test_part2_sample() {
    let res = part2("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "MCD");
}
