use anyhow::Result;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    let res = part1("input.txt")?;
    println!("Solution for part1 is {res}");
    let res = part2("input.txt")?;
    println!("Solution for part2 is {res}");
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let content = fs::read_to_string(input)?;
    sum_priorities_of_items_in_both_compartments(content)
}

fn part2(input: &str) -> Result<u32> {
    let content = fs::read_to_string(input)?;
    sum_priorities_of_items_in_3_groups(content)
}

fn sum_priorities_of_items_in_both_compartments(content: String) -> Result<u32> {
    let runsacks_content: Vec<_> = content.split("\n").collect();
    let mut duplicates = vec![];

    for runsack_content in runsacks_content.clone() {
        let (left_c_content, right_c_content) = runsack_content.split_at(runsack_content.len() / 2);

        'ext_loop: for lc in left_c_content.chars() {
            for rc in right_c_content.chars() {
                if lc == rc {
                    duplicates.push(lc);
                    break 'ext_loop;
                }
            }
        }
    }

    Ok(sum_priorities(duplicates))
}

fn sum_priorities_of_items_in_3_groups(content: String) -> Result<u32> {
    // We could also use array_chunks to split in groups of 3 (needs unstable toolchain)
    // .array_chunks::<3>()
    let mut temp = vec![];
    let groups: Vec<_> = content
        .split("\n")
        .enumerate()
        .filter_map(|(pos, content)| match pos % 3 {
            0 => {
                temp.clear();
                temp.push(content);
                None
            }
            2 => {
                temp.push(content);
                Some(temp.clone())
            }
            _ => {
                temp.push(content);
                None
            }
        })
        .collect();

    let mut duplicates = vec![];
    for group in groups {
        let mut group_dup: HashSet<char> = HashSet::from_iter(group[0].chars());
        for runsack in group[1..].to_vec() {
            let index: HashSet<char> = HashSet::from_iter(runsack.chars());
            group_dup = group_dup.intersection(&index).cloned().collect();
        }
        duplicates.push(group_dup.iter().cloned().last().unwrap());
    }

    Ok(sum_priorities(duplicates))
}

fn sum_priorities(v: Vec<char>) -> u32 {
    v.iter()
        .map(|d| {
            let priority = d.to_digit(36).unwrap() - 9;
            if d.is_lowercase() {
                priority
            } else {
                priority + 26
            }
        })
        .sum::<u32>()
}

#[test]
fn test_part1_sample() {
    let res = part1("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 157);
}

#[test]
fn test_part2_sample() {
    let res = part2("sample.txt");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 70);
}
