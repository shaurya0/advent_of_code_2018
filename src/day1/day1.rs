use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;

use std::error::Error;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut numbers: Vec<i64> = Vec::new();
    for line in BufReader::new(file).lines() {
        let value: i64 = line.unwrap().trim().parse()
            .expect("Failed to parse line");
        numbers.push(value);
    }

    let sum: i64 = numbers.iter().sum();
    println!("Result frequency : {}", sum);


    let mut sum = 0;
    let mut frequency_occurences = HashMap::new();
    frequency_occurences.insert(sum, 1);
    let mut second_occurence_found = false;

    while !second_occurence_found {
        for num in &numbers {
            sum += num;
            let count = frequency_occurences.entry(sum).or_insert(0);
            *count += 1;
            if *count == 2 {
                second_occurence_found = true;
                println!("Second frequency found {}", sum);
                break;
            }
        }
    }



}
