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
    let outcome = result(play, other);
    return outcome + play + 1;
}

fn main() {
    let cli = Cli::parse();

    // load input
    let file_path = "data/02";
    let input_path = if cli.test { "test.txt" } else { "input.txt" };
    let input = std::fs::read_to_string(format!("{file_path}/{input_path}")).unwrap();

    let game_scores = input.lines().filter_map(|s| {
        let [other, player] = s.split(' ').collect::<Vec<&str>>()[..] else {
            return None;
        };

        let other: Play = match other {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            _ => panic!("invalid play"),
        };

        let player: Play = match player {
            "X" => ROCK,
            "Y" => PAPER,
            "Z" => SCISSORS,
            _ => panic!("invalid play"),
        };

        let game_score = score(player, other);

        return Some(game_score);
    });

    let total_score = game_scores.sum::<i32>();

    println!("total score: {total_score}")
}
