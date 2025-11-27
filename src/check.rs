use std::fs;

use clap::Parser;
use std::collections::HashMap;

mod util;

#[derive(Parser, Debug)]
struct Args {
    /// Input file
    #[arg(short, long, required = true)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");

    let lines = contents.lines();
    for coloring in lines {
        let v = vec_from_coloring(coloring);
        println!("{v:?}");
        match util::check_coloring(&v) {
            Ok(msg) => println!("{coloring} - {msg}"),
            Err(err) => println!("{coloring} - {err:?}"),
        }
    }
}

// translate the human readable short-form of "abba" to a [0,1,1,0]
fn vec_from_coloring(s: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(s.len());

    let mut seen: HashMap<char, u8> = HashMap::new();
    let mut color: u8 = 0;
    for ch in s.chars() {
        if !seen.contains_key(&ch) {
            seen.insert(ch, color);
            color += 1;
        }
        result.push(*seen.get(&ch).unwrap());
    }

    return result;
}
