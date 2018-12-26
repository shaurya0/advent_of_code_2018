use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use std::error::Error;
extern crate regex;
use regex::Regex;


const NX: usize = 30;
const NY: usize = 20;
const X_OFFSET: i32 = 10;
const Y_OFFSET: i32 = 6;

#[derive(Debug)]
struct Light {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}



fn print_grid(grid: &[[char; NX]; NY])  {
    for i in 0..NY {
        for j in 0..NX {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
}

fn upate_grid(grid: &mut [[char; NX]; NY], lights: &mut Vec<Light>) {
    for light in lights.iter_mut() {

        let x = light.x as usize;
        let y = light.y as usize;
        grid[y][x] = '.';

        light.x += light.dx;
        light.y += light.dy;
        let x = light.x as usize;
        let y = light.y as usize;
        grid[y][x] = '#';


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

    // let mut lines: Vec<String> = Vec::new();
    // let re = Regex::new(r"position=<(.?\d), (.?\d)> velocity=<(.?\d), (.?\d)>").unwrap();
    let re = Regex::new(r"position=<(.?\d{5}), (.?\d{5})> velocity=<(.?\d{1}), (.?\d{1})>").unwrap();
    let mut lights: Vec<Light> = Vec::new();
    let mut min_light = Light{x:std::i32::MAX, y:std::i32::MAX, dx:0, dy:0};
    let mut max_light = Light{x:std::i32::MIN, y:std::i32::MIN, dx:0, dy:0};
    for line in BufReader::new(file).lines() {
        for cap in re.captures_iter(&line.unwrap()){
            let x :i32 = cap[1].trim().parse().expect("expected i32");
            let y :i32 = cap[2].trim().parse().expect("expected i32");
            let dx :i32 = cap[3].trim().parse().expect("expected i32");
            let dy :i32 = cap[4].trim().parse().expect("expected i32");

            if x < min_light.x{
                min_light.x = x;
            }
            if y < min_light.y{
                min_light.y = y;
            }
            if x > max_light.x{
                max_light.x = x;
            }
            if y > max_light.y{
                max_light.y = y;
            }
            // let x = x+X_OFFSET;
            // let y = y+Y_OFFSET;
            lights.push(Light{x, y, dx, dy});
        }
        // lines.push(line.unwrap());
    }
    println!("{:?}", lights);

    println!("{:?}", min_light);
    println!("{:?}", max_light);


    // let mut grid: [[char; NX]; NY] = [['.'; NX]; NY];
    // for light in lights.iter_mut(){
    //     let x = light.x as usize;
    //     let y = light.y as usize;
    //     grid[y][x] = '#';
    // }
    // print_grid(&grid);
    // println!("");

    // upate_grid(&mut grid, &mut lights);
    // print_grid(&grid);
    // println!("");
    // upate_grid(&mut grid, &mut lights);
    // print_grid(&grid);
    // println!("");
    // upate_grid(&mut grid, &mut lights);
    // print_grid(&grid);
    // println!("");
    // upate_grid(&mut grid, &mut lights);
    // print_grid(&grid);
    // println!("");
    // upate_grid(&mut grid, &mut lights);
    // print_grid(&grid);
    // println!("");


    // println!("{:?}", lights);
}


