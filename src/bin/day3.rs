use std::{collections::HashSet, io::{self, BufRead}};

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
    for line in stdin.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (a, b) = split_rucksack(&line);
        let common = common_items(a, b);
        total.extend(common.into_iter());
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