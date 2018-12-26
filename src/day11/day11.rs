use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use std::error::Error;


const NI32: i32 = 300;
const NSS: usize = NI32 as usize;

fn nth_digit(x: i32, n:i32) -> i32{
    let mut x = x;
    let mut n = n;
    while n > 0 {
        n = n-1;
        x = x/10;
    }
    x = x%10;
    x
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let x = x + 1;
    let y = y + 1;
    let rack_id = x+10;
    let mut pl: i32 = rack_id*y;
    pl += serial_number;
    pl *= rack_id;
    if pl < 100 {
        pl = 0;
    }
    else {
        pl = nth_digit(pl, 2);
    }
    pl -= 5;
    pl
}

fn init_grid(grid: &mut [[i32; NSS]; NSS], serial_number: i32) {
    for y in 0..NI32 {
        for x in 0..NI32 {
            let p = power_level(x,y, serial_number);
            grid[y as usize][x as usize] = p;
        }
    }

}

fn get_grid_val(grid: &[[i32; NSS]; NSS], x: usize, y: usize) -> i32 {
    let x = x-1;
    let y = y-1;
    grid[y][x]

}

fn main() {

    let mut grid: [[i32; NSS]; NSS] = [[0; NSS]; NSS];
    let serial_number: i32 = 7672;
    init_grid(&mut grid, serial_number);


    let mut max_sum: i32 = std::i32::MIN;
    let mut ii = 0;
    let mut jj = 0;
    let mut nn = 0;
    for n in 0..200 {
        for i in 0..NSS-n {
            for j in 0..NSS-n {
                let mut sum: i32 = 0;
                for k in 0..n {
                    for l in 0..n {
                        sum += grid[i+k][j+l];
                    }
                }
                if sum > max_sum {
                    max_sum = sum;
                    ii = i + 1;
                    jj = j + 1;
                    nn = n;
                }
            }
        }
        println!("n : {}", n);
    }
    println!("{},{},{}",jj,ii,nn);
}


