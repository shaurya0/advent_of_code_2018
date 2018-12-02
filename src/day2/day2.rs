use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;

use std::error::Error;

fn char_count(line: &String) -> [i32; 26] {
    let base_char: usize = b'a' as usize; // 'a'
    let mut array: [i32; 26] = [0; 26];
    for character in line.bytes(){
        let character = character as usize;
        let idx = character - base_char;
        array[idx] += 1;
    }
    return array;
}

fn part1(lines: &Vec<String>) {
    let mut twos = 0;
    let mut threes = 0;
    for line in lines.iter() {
        let array = char_count(line);
        let mut two_found = false;
        let mut three_found = false;
        for val in array.iter(){
            if *val == 2 && two_found == false {
                twos += 1;
                two_found = true;
            }
            if *val == 3 && three_found == false {
                threes += 1;
                three_found = true;
            }
        }
    }
    println!("Part 1 : {}", twos*threes);
}


fn part2(lines: &Vec<String>) {
    for line_o in lines.iter() {
        for line_i in lines.iter() {
            let mut diff = 0;
            for (a,b) in line_o.bytes().zip(line_i.bytes()){
                if a != b {
                    diff += 1;
                }
            }
            if diff == 1 {
                let mut result = String::new();
                for (a,b) in line_o.chars().zip(line_i.chars()){
                    if a != b {
                        continue;
                    }
                    result.push(a);
                }

                println!("Part 2 : {}", result);
                return;
            }
        }
    }
}



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

    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }

    part1(&lines);
    part2(&lines);


}


