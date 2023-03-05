use std::time::Instant;

use clap::{Parser, Subcommand};

mod problem;
mod solution;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run the solution to the specified problem")]
    Run {
        day: u32,
        part: u32,
        year: Option<u32>,
    },
    #[command(about = "List all the available solutions for the given year")]
    List { year: Option<u32> },
}

const DEFAULT_YEAR: u32 = 2022;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Run { year, day, part } => {
            let y = year.unwrap_or(DEFAULT_YEAR);
            let day = day.saturating_sub(1);

            let solutions = solution::get_year(y);
            let sol = match solutions.get(day as usize) {
                Some(s) => s,
                None => {
                    println!("No solution for day {} in year {}", day + 1, y);
                    return;
                }
            };

            println!(
                "Running: {}, Part {} (Day {} of {})",
                sol.name(),
                part,
                day + 1,
                y
            );

            let start = Instant::now();
            let output = match part {
                1 => sol.part_1(),
                2 => sol.part_2(),
                _ => return println!("[-] Invalid Part {}", part),
            };
            let elapsed = start.elapsed();

            println!("[+] OUT: {} ({:.2?})", output, elapsed);
        }
        Commands::List { year } => {
            println!(
                "Called List with this parma {}",
                year.unwrap_or(DEFAULT_YEAR)
            );
        }
    };
}
