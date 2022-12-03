use std::fs;

enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn outcome(opponent: RPS, player: RPS) -> u32 {
    match (opponent, player) {
        (RPS::Rock, RPS::Rock) => Outcome::Draw as u32,
        (RPS::Rock, RPS::Paper) => Outcome::Win as u32,
        (RPS::Rock, RPS::Scissors) => Outcome::Loss as u32,
        (RPS::Paper, RPS::Rock) => Outcome::Loss as u32,
        (RPS::Paper, RPS::Paper) => Outcome::Draw as u32,
        (RPS::Paper, RPS::Scissors) => Outcome::Win as u32,
        (RPS::Scissors, RPS::Rock) => Outcome::Win as u32,
        (RPS::Scissors, RPS::Paper) => Outcome::Loss as u32,
        (RPS::Scissors, RPS::Scissors) => Outcome::Draw as u32,
    }
}

fn score(opponent: RPS, player: RPS) -> u32 {
    player as u32 + outcome(opponent, player)
}

fn parse_round_one(line: &str) -> (RPS, RPS) {
    let round: Vec<&str> = line.split(" ").collect();
    match (round[0], round[1]) {
        ("A", "X") => (RPS::Rock, RPS::Rock),
        ("A", "Y") => (RPS::Rock, RPS::Paper),
        ("A", "Z") => (RPS::Rock, RPS::Scissors),
        ("B", "X") => (RPS::Paper, RPS::Rock),
        ("B", "Y") => (RPS::Paper, RPS::Paper),
        ("B", "Z") => (RPS::Paper, RPS::Scissors),
        ("C", "X") => (RPS::Scissors, RPS::Rock),
        ("C", "Y") => (RPS::Scissors, RPS::Paper),
        ("C", "Z") => (RPS::Scissors, RPS::Scissors),
        (_, _) => panic!("This shouldn't happen.")
    }
}

fn parse_round_two(line: &str) -> (RPS, RPS) {
    let round: Vec<&str> = line.split(" ").collect();
    match (round[0], round[1]) {
        ("A", "X") => (RPS::Rock, RPS::Scissors),
        ("A", "Y") => (RPS::Rock, RPS::Rock),
        ("A", "Z") => (RPS::Rock, RPS::Paper),
        ("B", "X") => (RPS::Paper, RPS::Rock),
        ("B", "Y") => (RPS::Paper, RPS::Paper),
        ("B", "Z") => (RPS::Paper, RPS::Scissors),
        ("C", "X") => (RPS::Scissors, RPS::Paper),
        ("C", "Y") => (RPS::Scissors, RPS::Scissors),
        ("C", "Z") => (RPS::Scissors, RPS::Rock),
        (_, _) => panic!("This shouldn't happen.")
    }
}

fn parse_input(filename: &str) -> Vec<(RPS, RPS)> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().map(|line| {
        parse_round_two(line)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paper_beats_rock_win() {
        assert_eq!(score(RPS::Rock, RPS::Paper), 8)
    }

    #[test]
    fn paper_beats_rock_loss() {
        assert_eq!(score(RPS::Paper, RPS::Rock), 1)
    }

    #[test]
    fn scissors_draw() {
        assert_eq!(score(RPS::Scissors, RPS::Scissors), 6)
    }
}

fn main() {
    let rounds: Vec<(RPS, RPS)> = parse_input("./src/input.txt");
    let scores = rounds.into_iter().map(|(o, p)| {
        score(o, p)
    });

    let total_score: u32 = scores.sum();
    println!("Total score: {}", total_score);
}
