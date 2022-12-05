use crate::aoc::load_lines;

#[derive(Clone)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}

impl RPC {
    fn score(&self) -> u64 {
        match self {
            RPC::Rock => 1,
            RPC::Paper => 2,
            RPC::Scissors => 3,
        }
    }

    fn points(&self, against: &RPC) -> u64 {
        match self {
            RPC::Rock => match against {
                RPC::Rock => 3,
                RPC::Paper => 0,
                RPC::Scissors => 6,
            },
            RPC::Paper => match against {
                RPC::Rock => 6,
                RPC::Paper => 3,
                RPC::Scissors => 0,
            },
            RPC::Scissors => match against {
                RPC::Rock => 0,
                RPC::Paper => 6,
                RPC::Scissors => 3,
            },
        }
    }

    fn counter(&self, outcome: Outcome) -> RPC {
        match outcome {
            Outcome::Win => match self {
                RPC::Rock => RPC::Paper,
                RPC::Paper => RPC::Scissors,
                RPC::Scissors => RPC::Rock,
            },
            Outcome::Draw => self.clone(),
            Outcome::Lose => match self {
                RPC::Rock => RPC::Scissors,
                RPC::Paper => RPC::Rock,
                RPC::Scissors => RPC::Paper,
            },
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

struct Round {
    opponent: RPC,
    you: RPC,
}

struct RoundB {
    opponent: RPC,
    outcome: Outcome,
}

fn parse_round(line: String) -> Round {
    let (opp, you) = line.split_once(" ").unwrap();
    Round {
        opponent: match opp {
            "A" => RPC::Rock,
            "B" => RPC::Paper,
            _ => RPC::Scissors,
        },
        you: match you {
            "X" => RPC::Rock,
            "Y" => RPC::Paper,
            _ => RPC::Scissors,
        },
    }
}

fn parse_round_b(line: String) -> RoundB {
    let (opp, out) = line.split_once(" ").unwrap();
    RoundB {
        opponent: match opp {
            "A" => RPC::Rock,
            "B" => RPC::Paper,
            _ => RPC::Scissors,
        },
        outcome: match out {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            _ => Outcome::Win,
        },
    }
}

fn load_from_file_a() -> Vec<Round> {
    load_lines(2)
        .into_iter()
        .map(|line| parse_round(line))
        .collect()
}

fn load_from_file_b() -> Vec<RoundB> {
    load_lines(2)
        .into_iter()
        .map(|line| parse_round_b(line))
        .collect()
}

pub fn a() -> u64 {
    _a(load_from_file_a())
}

pub fn b() -> u64 {
    _b(load_from_file_b())
}

fn _a(rounds: Vec<Round>) -> u64 {
    let mut score = 0u64;
    for round in rounds {
        score += round.you.score() + round.you.points(&round.opponent);
    }

    score
}

fn _b(rounds: Vec<RoundB>) -> u64 {
    let mut score = 0u64;
    for round in rounds {
        let counter = round.opponent.counter(round.outcome);
        score += counter.score() + counter.points(&round.opponent);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            15,
            _a(
                vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string(),]
                    .into_iter()
                    .map(|line| parse_round(line))
                    .collect()
            )
        );
    }

    #[test]
    fn a2() {
        assert_eq!(15691, _a(load_from_file_a()));
    }

    #[test]
    fn b() {
        assert_eq!(
            12,
            _b(
                vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string(),]
                    .into_iter()
                    .map(|line| parse_round_b(line))
                    .collect()
            )
        );
    }

    #[test]
    fn b2() {
        assert_eq!(12989, _b(load_from_file_b()));
    }
}
