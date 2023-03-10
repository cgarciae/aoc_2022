use clap::Parser;

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
        lines.map(|s| s.parse::<i32>().unwrap()).sum()
    });

    let max_calories: i32 = elf_calories.max().unwrap();

    println!("max calories: {max_calories}")
}
