use std::collections::{HashMap, HashSet};

use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    test: bool,
}

lazy_static! {
    static ref ITEM_PRIORITY: HashMap<char, usize> = HashMap::from_iter(
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

    let repeated_priorities = input.lines().chunks(3);
    let repeated_priorities = repeated_priorities.into_iter().map(|mut groups| {
        let mut repeated: HashSet<char> = HashSet::from_iter(groups.next().unwrap().chars());

        for group in groups {
            let mut next_repeated: HashSet<char> = HashSet::new();
            for c in group.chars() {
                if repeated.contains(&c) {
                    next_repeated.insert(c);
                }
            }
            repeated = next_repeated;
        }

        let n = repeated.len();

        if n != 1 {
            panic!("expected 1 unique item, found {n}");
        }

        let c = repeated.iter().next().unwrap();

        return ITEM_PRIORITY.get(c).unwrap().clone();
    });

    let total_priorities = repeated_priorities.sum::<usize>();

    println!("total priorities: {total_priorities}");
}
