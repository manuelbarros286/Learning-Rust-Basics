use std::env;
use std::fs;

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

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            println!("{}", line);
        }
    }
}
