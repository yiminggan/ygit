use std::{env, fs::File, io::{BufRead, BufReader}, process};
use ygit::diff_str::diff;

fn main() {
    // Collect command-line arguments into a vector.
    let args: Vec<String> = env::args().collect();

    // Expect exactly two additional arguments: a_file and b_file.
    if args.len() != 3 {
        eprintln!("Usage: {} <FILE> <FILE>", args[0]);
        process::exit(1);
    }

    let a_file: &String = &args[1];
    let b_file: &String = &args[2];

    // Read and split a_file into lines, trimming trailing whitespace.
    let a_lines: Vec<String> = match File::open(a_file) {
        Ok(file) => {
            let reader = BufReader::new(file);
            reader
                .lines()
                .map(|line| line.unwrap_or_default().trim_end().to_string())
                .collect()
        },
        Err(e) => {
            eprintln!("Error opening {}: {}", a_file, e);
            process::exit(1);
        }
    };
    // Read and split b_file into lines, trimming trailing whitespace.
    let b_lines: Vec<String> = match File::open(b_file) {
        Ok(file) => {
            let reader = BufReader::new(file);
            reader
                .lines()
                .map(|line| line.unwrap_or_default().trim_end().to_string())
                .collect()
        },
        Err(e) => {
            eprintln!("Error opening {}: {}", b_file, e);
            process::exit(1);
        }
    };
    let diff_trace = diff(&a_lines, &b_lines);
    print!("diff: {:?}\n", diff_trace);
}