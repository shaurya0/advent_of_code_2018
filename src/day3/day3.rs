use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;


#[derive(PartialEq, Debug, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Claim {
    left_offset: u32,
    top_offset: u32,
    width: u32,
    height: u32,
    id: u32
}


#[derive(Debug)]
struct Count {
    count: u32,
    id: u32,
}

fn to_claim(claim: &String) -> Claim {
    let mut alpha_pos = 0;
    let mut comma_pos = 0;
    let mut colon_pos = 0;
    let mut x_pos = 0;

    for (i, ch) in claim.chars().enumerate(){
        if ch == '@' {
            alpha_pos = i;
        } else if ch == ',' {
            comma_pos = i;
        } else if ch == ':' {
            colon_pos = i;
        } else if ch == 'x' {
            x_pos = i;
        }
    }
    let left_offset: u32 = claim[alpha_pos+2..comma_pos]
        .parse().expect("Expected u32");
    let top_offset: u32 = claim[comma_pos+1..colon_pos]
        .parse().expect("Expected u32");
    let width: u32 = claim[colon_pos+2..x_pos]
        .parse().expect("Expected u32");
    let height: u32 = claim[x_pos+1..claim.len()]
        .parse().expect("Expected u32");

    let id: u32 = claim[1..alpha_pos-1]
        .parse().expect("Expected u32");
// #123 @ 3,2: 5x4


    Claim{
        left_offset,
        top_offset,
        width,
        height,
        id
    }
}

fn check_claim(grid_description: &HashMap<Point, Count>, claim : &String) {
    let claim = to_claim(claim);
    let mut unclaimed = true;
    for i in 0..claim.height{
        let y = claim.top_offset + i;
        for j in 0..claim.width{
            let point = Point{
                x : claim.left_offset + j,
                y,
            };
            // print!("{:?}, ", point);
            // let count = grid_description.entry(point);
            // *count.count;
            match grid_description.get(&point) {
                Some(count) if count.count != 1 => unclaimed = false,
                Some(count) => unclaimed = true,
                _ => print!(".")
            }
            if unclaimed == false{
                return;
            }
            // count.count += 1;
        }
    }
    println!("unclaimed id : {:?}", claim.id);
}



fn update_grid(grid_description: &mut HashMap<Point, Count>, claim : &String) {
    let claim = to_claim(claim);
    for i in 0..claim.height{
        let y = claim.top_offset + i;
        for j in 0..claim.width{
            let point = Point{
                x : claim.left_offset + j,
                y,
            };
            // print!("{:?}, ", point);
            let count = grid_description.entry(point)
            .or_insert(Count{
                count: 0,
                id: claim.id
            });
            count.count += 1;
        }
    }
}

fn draw_grid(grid_description: &HashMap<Point, i32>, nx: u32, ny: u32){
    for y in 0..ny{
        for x in 0..nx{
            let point = Point{
                x,
                y,
            };
            match grid_description.get(&point) {
                Some(&count) if count > 1 => print!("X"),
                Some(&count) => print!("#"),
                _ => print!(".")
            }
        }
        println!("");
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

    let mut grid_description: HashMap<Point, Count>
        = HashMap::new();

    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }

    for line in lines.iter() {
        update_grid(&mut grid_description, line)
    }
    for line in lines.iter() {
        check_claim(&grid_description, line)
    }

    let mut sum = 0;
    for (_, count) in &grid_description {
        if count.count > 1 {
            sum += 1;
        } else if count.count == 1{
            // print!("{}", count.id);
        }
        // println!("{:?} {}", point, count);
        // println!("{:?}", cout);
        // println!("{}{}", &point, count);
    }
    println!("sum covered : {}", sum);
    // draw_grid(&grid_description, 8, 8);
}


