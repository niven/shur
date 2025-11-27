use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

use clap::Parser;
use tera::{Context, Tera};

#[derive(Parser, Debug)]
struct Args {
    /// Input file
    #[arg(short, long, required = true)]
    source: String,

    /// Index in inputnfile
    #[arg(short, long, default_value = "0")]
    index: usize,

    /// Input file
    #[arg(short, long, required = true)]
    destination: String,
}

const COLOR_MAPPING: [&str; 5] = ["#fc17da", "#FFFF00", "#38befc", "#19de12", "#eb5c34"];

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.source).expect("Should have been able to read the file");
    let lines: Vec<_> = contents.lines().collect();
    let selected = lines[args.index];
    println!("Selected coloring: {}", selected);

    let tera = match Tera::new("templates/*.dot") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            process::exit(1);
        }
    };
    let mut context = Context::new();

    let colors: Vec<char> = selected.chars().collect();
    let mut uniq: HashSet<&char> = HashSet::new();
    let mapped: Vec<&str> = colors
        .iter()
        .map(|c| {
            uniq.insert(c);
            match c {
                'a' => COLOR_MAPPING[0],
                'b' => COLOR_MAPPING[1],
                'c' => COLOR_MAPPING[2],
                'd' => COLOR_MAPPING[3],
                'e' => COLOR_MAPPING[4],
                _ => {
                    panic!("Unsupported color: {}", c);
                }
            }
        })
        .collect();
    context.insert("colors", &mapped);

    let title = format!(
        "Coloring for {} colors and {} numbers",
        uniq.len(),
        mapped.len()
    );
    context.insert("title", &title);

    let graph = tera
        .render("graph.dot", &context)
        .expect("Should be able to render template");

    let mut file = File::create(args.destination).expect("Should be able to create file");
    file.write_all(graph.as_bytes())
        .expect("Should be able to write to file");
}
