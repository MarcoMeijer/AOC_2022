#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

use Move::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Round {
    Win,
    Draw,
    Lose,
}

use Round::*;

trait Points {
    fn get_points(&self) -> u32;
}

impl Points for Move {
    fn get_points(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl Points for Round {
    fn get_points(&self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

fn strategy(m: Move, r: Round) -> Move {
    match (m, r) {
        (Rock, Win) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Win) => Rock,
        (m, Draw) => m,
        (Rock, Lose) => Scissors,
        (Paper, Lose) => Rock,
        (Scissors, Lose) => Paper,
    }
}

fn get_winner(op: Move, re: Move) -> Round {
    match (op, re) {
        (Rock, Paper) => Win,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (x, y) if x == y => Draw,
        _ => Lose,
    }
}

pub fn day2_silver(input: &str) -> String {
    let res: u32 = input
        .lines()
        .map(|line| {
            let opponent = match &line[0..1] {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!(),
            };
            let response = match &line[2..3] {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                _ => panic!(),
            };
            let result = get_winner(opponent, response);
            response.get_points() + result.get_points()
        })
        .sum();
    res.to_string()
}

pub fn day2_gold(input: &str) -> String {
    let res: u32 = input
        .lines()
        .map(|line| {
            let opponent = match &line[0..1] {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!(),
            };
            let result = match &line[2..3] {
                "X" => Lose,
                "Y" => Draw,
                "Z" => Win,
                _ => panic!(),
            };
            let response = strategy(opponent, result);
            response.get_points() + result.get_points()
        })
        .sum();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn sample_silver() {
        let expected = "15".to_string();
        assert_eq!(day2_silver(INPUT), expected);
    }

    #[test]
    fn sample_gold() {
        let expected = "12".to_string();
        assert_eq!(day2_gold(INPUT), expected);
    }
}
