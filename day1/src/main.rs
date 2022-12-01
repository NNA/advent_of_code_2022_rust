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
    max_calories_per_elf(content)
}

fn part2(input: &str) -> Result<u32> {
    let content = fs::read_to_string(input)?;
    top_3_max_calories_per_elf(content)
}

fn max_calories_per_elf(content: String) -> Result<u32> {
    let max = content
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|e| e.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    Ok(max)
}

fn top_3_max_calories_per_elf(content: String) -> Result<u32> {
    let mut v = content
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|e| e.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    v.sort();
    v.reverse();
    v.truncate(3);

    let top3 = v.iter().sum::<u32>();

    Ok(top3)
}

#[test]
fn test_part1_sample() {
    let res = part1("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 24_000);
}

#[test]
fn test_part2_sample() {
    let res = part2("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 45_000);
}
