use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    test: bool,
}

type Play = i32;
const ROCK: Play = 0;
const PAPER: Play = 1;
const SCISSORS: Play = 2;

type Outcome = i32;
const LOSE: Outcome = 0;
const DRAW: Outcome = 3;
const WIN: Outcome = 6;

fn result(play: Play, other: Play) -> Outcome {
    if play == other {
        return DRAW;
    }
    return if (other + 1) % 3 == play { WIN } else { LOSE };
}

fn score(play: Play, other: Play) -> i32 {
    assert!(play >= 0 && play <= 2);
    assert!(other >= 0 && other <= 2);

    let outcome = result(play, other);
    return outcome + (play + 1);
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

        let other: Play = match other {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            _ => panic!("invalid play"),
        };

        let player = match action {
            // lose
            "X" => (other - 1).rem_euclid(3),
            // draw
            "Y" => other,
            // win
            "Z" => (other + 1).rem_euclid(3),
            _ => panic!("invalid action: {action}"),
        };

        return Some(score(player, other));
    });

    let total_score = game_scores.sum::<i32>();

    println!("total score: {total_score}")
}
