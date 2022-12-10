use std::{io::{self, BufRead}, str::FromStr};

enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn to_value(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn outcome(&self, opponent: RPS) -> usize {
        use RPS::*;
        self.to_value() + match (self, opponent) {
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => { 6 }
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => { 0 },
            _ => { 3 },
        }
    }

    fn result(&self, oc: Outcome) -> usize {
        use RPS::*;
        use Outcome::*;
        match (self, oc) {
            (x, Draw) => x.outcome(*x),
            (Paper, Win) => Scissors.outcome(Paper),
            (Rock, Win) => Paper.outcome(Rock),
            (Scissors, Win) => Rock.outcome(Scissors),

            (Paper, Loss) => Rock.outcome(Paper),
            (Rock, Loss) => Scissors.outcome(Rock),
            (Scissors, Loss) => Paper.outcome(Scissors),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Unknown")]
struct RPSError;

impl FromStr for RPS {
    type Err = RPSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            _ => Err(RPSError),
        }
    }
}

impl FromStr for Outcome {
    type Err = RPSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(RPSError),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut acc = 0;
    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        let line = line?;
        let cmds = line.split(' ').collect::<Vec<_>>();
        let opp = RPS::from_str(cmds[0])?;
        let out = Outcome::from_str(cmds[1])?;
        let outcome = opp.result(out);
        acc += outcome;
    }
    println!("{acc}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_outcomes() {
        use RPS::*;
        assert_eq!(Paper.outcome(Rock), 8);
        assert_eq!(Rock.outcome(Paper), 1);
        assert_eq!(Scissors.outcome(Scissors), 6);
    }
}