use std::env;
use std::fs;
use std::io::{self, BufRead};
use colored::*;
fn main() {
    //collect command line argument into a vector of strings
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not enough arguments provided!");
        return;
    }

    let query = &args[1];
    let file_path = &args[2];

    println!("query is {:?} in file {:?}", query, file_path);

    let file = fs::File::open(file_path).expect("file not found");
    let reader = io::BufReader::new(file);


    let colored_query = query.red().bold().to_string();
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("something went wrong reading the file");
        if line.contains(query) {
            let highlighted = line.replace(query, &colored_query);
            println!("{}: {}", (index + 1).to_string().cyan(), highlighted);
        }
    }
}