use std::{collections::HashSet, io::{self, BufRead}};

use itertools::Itertools;

fn split_rucksack(s: &str) -> (&str, &str) {
    let len = s.len() / 2;
    s.split_at(len)
}

fn common_items(a: &str, b: &str) -> HashSet<char> {
    let ac: HashSet<char> = a.chars().collect();
    let bc = b.chars().collect();
    ac.intersection(&bc).copied().collect()
}

fn sum_priorities<I>(cs: I) -> u32
where
    I: Iterator<Item = char>,
{
    cs.map(to_priority).sum()
}

fn to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => u32::from(c) - 96,
        'A'..='Z' => u32::from(c) - 38,
        _ => panic!("Invalid char {c}"),
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin().lock();
    let mut total = Vec::new();
    for lines in stdin.lines().flatten().chunks(3).into_iter() {
        let lines = lines.collect::<Vec<_>>();
        if lines.len() < 3 {
            break;
        }
        let a = lines[0].chars().collect::<HashSet<_>>();
        let b = lines[1].chars().collect::<HashSet<_>>();
        let c = lines[2].chars().collect::<HashSet<_>>();
        let all = &(&a & &b) & &c;
        total.extend(all.into_iter());
    }
    println!("{}", sum_priorities(total.into_iter()));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_rucksack() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (a, b) = split_rucksack(s);
        assert_eq!(a, "vJrwpWtwJgWr");
        assert_eq!(b, "hcsFMMfFFhFp");
    }

    #[test]
    fn test_common() {
        let a = "vJrwpWtwJgWr";
        let b =  "hcsFMMfFFhFp";
        let common = common_items(a, b);
        let result = common.into_iter().collect::<Vec<_>>();
        assert_eq!(result, vec!['p']);

    }
}