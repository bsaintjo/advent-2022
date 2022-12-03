use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Elf(usize);

fn main() -> anyhow::Result<()> {
    let mut elf = Elf(0);
    let mut acc = Vec::new();

    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        let line = line?;
        if line.is_empty() {
            acc.push(elf);
            elf = Elf(0);
        } else {
            let calories: usize = line.parse()?;
            elf = Elf(elf.0 + calories);
        }
    }
    acc.push(elf);

    acc.sort();
    let top_three: usize = acc[acc.len() - 3..].iter().map(|e| e.0).sum();
    println!("{top_three}");
    
    Ok(())
}