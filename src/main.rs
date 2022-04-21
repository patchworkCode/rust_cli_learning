use std::io;
use regex::Regex;
use clap::{App, Arg};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn process_input_arg<T: BufRead + Sized>(reader: T, re: Regex) -> Option<Vec<(i16, String)>> {
    let mut matching_lines = vec![];
    let mut line_number = 0;
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if line.is_empty() {    
            match matching_lines.len() {
                0 => return None,
                _ => return Some(matching_lines),
            }
        }
        match re.find(&line) {
            Some(_) => matching_lines.push((line_number, line)),
            None => (),
        }
    line_number += 1;
    }

    match matching_lines.len() {
        0 => return None,
        _ => return Some(matching_lines),
    }
}


fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(
            Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    
    let matched_lines;
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        matched_lines = process_input_arg(reader, re);
    }
    else {
        let f = File::open(Path::new(input)).unwrap();
        let reader = BufReader::new(f);
        matched_lines = process_input_arg(reader, re);
    }

    match matched_lines {
        Some(_) => {
            println!("The following line(s) match the pattern:");
            for tuple in matched_lines.unwrap() {
                println!("{:?}: {:?}", tuple.0, tuple.1)
            }
        },
        None => println!("No matches found")
    }

}

