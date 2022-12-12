use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let res = part1("input.txt")?;
    println!("Solution for part1 is {res}");
    let res = part2("input.txt")?;
    println!("Solution for part2 is {res}");
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let content = fs::read_to_string(input)?;
    let mut fully_contains_count = 0;
    for line in content.split("\n") {
        let groups = line.split(",").collect::<Vec<&str>>();
        let g1 = groups[0].split("-").collect::<Vec<&str>>();
        let g2 = groups[1].split("-").collect::<Vec<&str>>();
        if one_fully_contained(g1, g2) {
            fully_contains_count += 1;
        }
    }
    Ok(fully_contains_count)
}

fn part2(input: &str) -> Result<u32> {
    let content = fs::read_to_string(input)?;
    let mut overlap_count = 0;
    for line in content.split("\n") {
        let groups = line.split(",").collect::<Vec<&str>>();
        let g1 = groups[0].split("-").collect::<Vec<&str>>();
        let g2 = groups[1].split("-").collect::<Vec<&str>>();
        if one_overlaps(g1.clone(), g2.clone()) || one_fully_contained(g1.clone(), g2.clone()) {
            overlap_count += 1;
        }
    }
    Ok(overlap_count)
}

fn one_fully_contained(g1: Vec<&str>, g2: Vec<&str>) -> bool {
    let g2_contained = (g1[0].parse::<u32>().unwrap() <= g2[0].parse::<u32>().unwrap())
        && (g1[1].parse::<u32>().unwrap() >= g2[1].parse::<u32>().unwrap());
    let g1_contained = (g2[0].parse::<u32>().unwrap() <= g1[0].parse::<u32>().unwrap())
        && (g2[1].parse::<u32>().unwrap() >= g1[1].parse::<u32>().unwrap());
    g2_contained || g1_contained
}

fn one_overlaps(g1: Vec<&str>, g2: Vec<&str>) -> bool {
    let g2_start_overlaps = (g2[0].parse::<u32>().unwrap() >= g1[0].parse::<u32>().unwrap())
        && (g2[0].parse::<u32>().unwrap() <= g1[1].parse::<u32>().unwrap());
    let g2_end_overlaps = (g2[1].parse::<u32>().unwrap() >= g1[0].parse::<u32>().unwrap())
        && (g2[1].parse::<u32>().unwrap() <= g1[1].parse::<u32>().unwrap());
    g2_start_overlaps || g2_end_overlaps
}

#[test]
fn test_part1_sample() {
    let res = part1("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn test_part2_sample() {
    let res = part2("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 4);
}
