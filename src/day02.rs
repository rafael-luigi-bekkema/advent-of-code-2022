use crate::aoc::load_lines;

#[derive(Clone)]
enum Rpc {
    Rock,
    Paper,
    Scissors,
}

impl Rpc {
    fn score(&self) -> u64 {
        match self {
            Rpc::Rock => 1,
            Rpc::Paper => 2,
            Rpc::Scissors => 3,
        }
    }

    fn points(&self, against: &Rpc) -> u64 {
        match self {
            Rpc::Rock => match against {
                Rpc::Rock => 3,
                Rpc::Paper => 0,
                Rpc::Scissors => 6,
            },
            Rpc::Paper => match against {
                Rpc::Rock => 6,
                Rpc::Paper => 3,
                Rpc::Scissors => 0,
            },
            Rpc::Scissors => match against {
                Rpc::Rock => 0,
                Rpc::Paper => 6,
                Rpc::Scissors => 3,
            },
        }
    }

    fn counter(&self, outcome: Outcome) -> Rpc {
        match outcome {
            Outcome::Win => match self {
                Rpc::Rock => Rpc::Paper,
                Rpc::Paper => Rpc::Scissors,
                Rpc::Scissors => Rpc::Rock,
            },
            Outcome::Draw => self.clone(),
            Outcome::Lose => match self {
                Rpc::Rock => Rpc::Scissors,
                Rpc::Paper => Rpc::Rock,
                Rpc::Scissors => Rpc::Paper,
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
    opponent: Rpc,
    you: Rpc,
}

struct RoundB {
    opponent: Rpc,
    outcome: Outcome,
}

fn parse_round(line: String) -> Round {
    let (opp, you) = line.split_once(' ').unwrap();
    Round {
        opponent: match opp {
            "A" => Rpc::Rock,
            "B" => Rpc::Paper,
            _ => Rpc::Scissors,
        },
        you: match you {
            "X" => Rpc::Rock,
            "Y" => Rpc::Paper,
            _ => Rpc::Scissors,
        },
    }
}

fn parse_round_b(line: String) -> RoundB {
    let (opp, out) = line.split_once(' ').unwrap();
    RoundB {
        opponent: match opp {
            "A" => Rpc::Rock,
            "B" => Rpc::Paper,
            _ => Rpc::Scissors,
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
        .map(parse_round)
        .collect()
}

fn load_from_file_b() -> Vec<RoundB> {
    load_lines(2)
        .into_iter()
        .map(parse_round_b)
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
