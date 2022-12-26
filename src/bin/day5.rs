use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

struct Op {
    from: usize,
    to: usize,
    n: usize,
}

impl Op {
    fn from_str(s: &str) -> Self {
        let s = s.split_whitespace().collect_vec();
        let n = s[1].parse::<usize>().unwrap();
        let from = s[3].parse::<usize>().unwrap();
        let to = s[5].parse::<usize>().unwrap();
        Op { from, to, n }
    }
}

struct Stacks {
    stacks: HashMap<usize, VecDeque<Crate>>,
}

impl Stacks {
    fn apply(&mut self, op: Op) {
        let from_stack = self.stacks.get_mut(&op.from).unwrap();
        let removed: VecDeque<_> = (0..op.n).map(|_| from_stack.pop_back().unwrap()).collect();
        let to_stack = self.stacks.get_mut(&op.to).unwrap();
        removed
            .into_iter()
            .rev()
            .for_each(|c| to_stack.push_back(c));
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Crate(char);

fn parse_stacks(s: &str) -> Stacks {
    let mut stacks = HashMap::new();
    let numeric: Vec<char> = ('0'..='9').collect();
    for line in s.lines() {
        if !(line.contains(numeric.as_slice())) {
            let stack = parse_stack(line);
            for (idx, cr) in stack.into_iter() {
                let ss: &mut VecDeque<_> = stacks.entry(idx).or_default();
                ss.push_front(cr);
            }
        }
    }

    Stacks { stacks }
}

fn parse_stack(line: &str) -> HashMap<usize, Crate> {
    line.char_indices()
        .filter(|x| x.1.is_alphabetic())
        .map(|(p, c)| (convert_idx(p), Crate(c)))
        .collect()
}

fn convert_idx(x: usize) -> usize {
    ((x - 1) / 4) + 1
}

fn main() -> anyhow::Result<()> {
    todo!()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_idx_conversion() {
        let cases = [(1, 1), (5, 2), (9, 3), (13, 4)];
        for (x, y) in cases {
            assert_eq!(convert_idx(x), y);
        }
    }

    #[test]
    fn test_parse_row() {
        let xs: Vec<(&str, HashMap<usize, Crate>)> = vec![
            ("    [D]    ", [(2, Crate('D'))].into_iter().collect()),
            (
                "[N] [C]    ",
                [(1, Crate('N')), (2, Crate('C'))].into_iter().collect(),
            ),
        ];

        for (input, answer) in xs.into_iter() {
            assert_eq!(parse_stack(input), answer);
        }
    }

    #[test]
    fn test_apply() {
        let xs: HashMap<usize, VecDeque<_>> = [
            (1, [Crate('D')].into()),
            (2, [Crate('A'), Crate('C')].into()),
        ]
        .into();
        let mut stacks = Stacks { stacks: xs };
        let op = Op {
            from: 2,
            to: 1,
            n: 1,
        };
        stacks.apply(op);
        assert_eq!(stacks.stacks, [
            (1, [Crate('D'), Crate('C')].into()),
            (2, [Crate('A')].into()),
        ].into())
    }
}
