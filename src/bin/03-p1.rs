use std::collections::HashMap;

use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    test: bool,
}

lazy_static! {
    static ref ITEM_3PRIORITY: HashMap<char, usize> = HashMap::from_iter(
        ('a'..='z')
            .chain('A'..='Z')
            .enumerate()
            .map(|(i, c)| (c, i + 1)),
    );
}

fn main() {
    let cli = Cli::parse();

    // load input
    let file_path = "data/03";
    let input_path = if cli.test { "test.txt" } else { "input.txt" };
    let input = std::fs::read_to_string(format!("{file_path}/{input_path}")).unwrap();

    let repeated_priorities = input.lines().map(|line| {
        let n = line.len() / 2;
        let first_compartment = &line[..n];
        let second_compartment = &line[n..];

        for a in first_compartment.chars() {
            for b in second_compartment.chars() {
                if a == b {
                    return ITEM_PRIORITY.get(&a).unwrap().clone();
                }
            }
        }
        unreachable!("no repeated item found");
    });

    let total_priorities = repeated_priorities.sum::<usize>();

    println!("total priorities: {total_priorities}");
}
