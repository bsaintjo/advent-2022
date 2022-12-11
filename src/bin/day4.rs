use std::io::{self, BufRead};

struct Pairs {
    fst: AssignRange,
    snd: AssignRange,
}

impl Pairs {
    fn from_str(s: &str) -> Self {
        let res = s.split(",").collect::<Vec<_>>();
        let fst = AssignRange::from_str(res[0]);
        let snd = AssignRange::from_str(res[1]);
        Pairs { fst, snd }
    }

    fn contains_either(&self) -> bool {
        self.fst.contains(&self.snd) || self.snd.contains(&self.fst)
    }

    fn overlaps(&self) -> bool {
        self.fst.overlaps(&self.snd) || self.snd.overlaps(&self.fst)
    }
}

struct AssignRange(usize, usize);

impl AssignRange {
    fn from_str(s: &str) -> Self {
        let res = s.split('-').collect::<Vec<_>>();
        let x = res[0].parse::<usize>().unwrap();
        let y = res[1].parse::<usize>().unwrap();
        AssignRange(x, y)
    }

    fn contains(&self, other: &AssignRange) -> bool {
        (self.0 <= other.0) && (self.1 >= other.1)
    }

    fn overlaps(&self, other: &AssignRange) -> bool {
        ((self.1 >= other.0) && (self.1 <= other.1)) || ((self.0 >= other.0) && (self.1 <= other.1))
    }
}

fn main() -> anyhow::Result<()> {
    let mut acc = 0;
    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let p = Pairs::from_str(&line);
        if p.overlaps() {
            acc += 1;
        }
    }
    println!("{acc}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let x = AssignRange(6, 6);
        let y = AssignRange(4, 6);
        assert!(y.contains(&x))
    }

    #[test]
    fn test_contains_either() {
        let fst = AssignRange(6, 6);
        let snd = AssignRange(4, 6);
        let pair = Pairs { fst, snd };
        assert!(pair.contains_either())
    }

    #[test]
    fn test_overlaps() {
        let fst = AssignRange(5, 7);
        let snd = AssignRange(7, 9);
        assert!(fst.overlaps(&snd))
    }
}