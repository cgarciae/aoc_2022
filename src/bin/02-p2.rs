use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    test: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<i32> for Play {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Play::Rock),
            2 => Ok(Play::Paper),
            3 => Ok(Play::Scissors),
            _ => Err(format!("invalid play: {}", value)),
        }
    }
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn result(play: Play, other: Play) -> Outcome {
    if play == other {
        return Outcome::Draw;
    }
    return if ((other as i32 - 1) + 1) % 3 == (play as i32 - 1) {
        Outcome::Win
    } else {
        Outcome::Lose
    };
}

fn score(play: Play, other: Play) -> i32 {
    let outcome = result(play, other);
    return outcome as i32 + play as i32;
}

fn main() {
    let cli = Cli::parse();

    // load input
    let file_path = "data/02";
    let input_path = if cli.test { "test.txt" } else { "input.txt" };
    let input = std::fs::read_to_string(format!("{file_path}/{input_path}")).unwrap();

    let game_scores = input.lines().filter_map(|s| {
        let [other, action] = s.split(' ').collect::<Vec<&str>>()[..] else {
            return None;
        };

        let other = match other {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("invalid other: {other}"),
        };

        let player = match action {
            // lose
            "X" => Play::try_from((other as i32 - 2).rem_euclid(3) + 1).unwrap(),
            // draw
            "Y" => other,
            // win
            "Z" => Play::try_from(((other as i32).rem_euclid(3)) + 1).unwrap(),
            _ => panic!("invalid action: {action}"),
        };

        return Some(score(player, other));
    });

    let total_score = game_scores.sum::<i32>();

    println!("total score: {total_score}")
}
