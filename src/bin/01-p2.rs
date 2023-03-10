use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    test: bool,
}

fn main() {
    let cli = Cli::parse();

    // load input
    let file_path = "data/01";
    let input_path = if cli.test { "test.txt" } else { "input.txt" };
    let input = std::fs::read_to_string(format!("{file_path}/{input_path}")).unwrap();

    let elf_calories = input.split("\n\n").map(|ls| {
        let lines = ls.lines();
        lines.map(|s| s.parse::<i32>().unwrap()).sum::<i32>()
    });

    let sum_largest_3 = elf_calories.sorted_by(|a, b| b.cmp(a)).take(3).sum::<i32>();

    println!("sum of largest 3: {sum_largest_3}");
}
