use std::process;

use clap::Parser;

mod algorithm;
mod util;

#[derive(Parser, Debug)]
pub struct Args {
    /// Number of colors to use
    #[arg(long, required = true)]
    colors: u8,

    /// Target number you want a coloring for
    #[arg(long, required = true)]
    target: usize,

    /// Algorithm
    #[arg(long, required = true)]
    algorithm: String,

    /// Attempts
    #[arg(long, default_value = "100")]
    attempts: usize,
}

const COLOR_LETTERS: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

fn main() {
    let args = Args::parse();

    if args.colors as usize > COLOR_LETTERS.len() {
        panic!("Max colors is {}", COLOR_LETTERS.len());
    }

    let algorithm = match args.algorithm.as_str() {
        "random" => algorithm::random::random_assignment,
        "random_ban" => algorithm::random::random_with_bannings,
        "random_dfs" => algorithm::random::random_with_backtrack,
        "search_dfs" => algorithm::search::depth_first,
        _ => {
            println!("Unsupported algorithm: {}", args.algorithm);
            process::exit(1);
        }
    };

    let result = algorithm(&args);
    match result {
        Some(solution) => {
            println!("Result: {:?}", solution);
            println!("Short form: {:?}", util::short(&solution));
            match util::check_coloring(&solution) {
                Ok(check_ok) => {
                    println!("{check_ok}");
                }
                Err(invalid) => {
                    println!(
                        "Coloring failed at index {}: {}",
                        invalid.index, invalid.message
                    );
                }
            }
        }
        None => {
            println!("No coloring found.");
        }
    }

    process::exit(0);
}
