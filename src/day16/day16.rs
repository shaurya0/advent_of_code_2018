use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::slice::Iter;
use std::{thread, time};
extern crate regex;
use regex::Regex;


#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Opcode {
    Addr, // (add register) stores into register C the result of adding register A and register B.
    Addi, // (add immediate) stores into register C the result of adding register A and value B.
    Mulr, // (multiply register) stores into register C the result of multiplying register A and register B.
    Muli, // (multiply immediate) stores into register C the result of multiplying register A and value B.
    Banr, // (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    Bani, // (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    Borr, // (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    Bori, // (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    Setr, // (set register) copies the contents of register A into register C. (Input B is ignored.)
    Seti, // (set immediate) stores value A into register C. (Input B is ignored.)
    Gtir, // (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtri, // (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtrr, // (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Eqir, // (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqri, // (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqrr, // (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
}

static OPCODES: [Opcode;  16] = [
Opcode::Addr,
Opcode::Addi,
Opcode::Mulr,
Opcode::Muli,
Opcode::Banr,
Opcode::Bani,
Opcode::Borr,
Opcode::Bori,
Opcode::Setr,
Opcode::Seti,
Opcode::Gtir,
Opcode::Gtri,
Opcode::Gtrr,
Opcode::Eqir,
Opcode::Eqri,
Opcode::Eqrr];


#[derive(Debug)]
struct CPU{
    regs: [i32; 4]
}

impl CPU {
    fn new(regs: [i32; 4]) -> CPU {
        CPU{regs}
    }

    fn set_regs(&mut self, regs: [i32; 4]) {
        self.regs = regs;
    }

    fn execute_instruction(&mut self, opcode: Opcode, a: usize, b: usize, c: usize) {
        match opcode {
            Opcode::Addr => {
                self.regs[c] = self.regs[a] + self.regs[b];
            },
            Opcode::Addi => {
                self.regs[c] = self.regs[a] + (b as i32);
            },
            Opcode::Mulr => {
                self.regs[c] = self.regs[a] * self.regs[b];
            },
            Opcode::Muli => {
                self.regs[c] = self.regs[a] * (b as i32);
            },
            Opcode::Banr => {
                self.regs[c] = self.regs[a] & self.regs[b];
            },
            Opcode::Bani => {
                self.regs[c] = self.regs[a] & (b as i32);
            },
            Opcode::Borr => {
                self.regs[c] = self.regs[a] | self.regs[b];
            },
            Opcode::Bori => {
                self.regs[c] = self.regs[a] | (b as i32);
            },
            Opcode::Setr => {
                self.regs[c] = self.regs[a];
            },
            Opcode::Seti => {
                self.regs[c] = (a as i32);
            },
            Opcode::Gtir => {
                self.regs[c] = 0;
                if (a as i32) > self.regs[b]{
                    self.regs[c] = 1;
                }
            },
            Opcode::Gtri => {
                self.regs[c] = 0;
                if self.regs[a] > (b as i32){
                    self.regs[c] = 1;
                }
            },
            Opcode::Gtrr => {
                self.regs[c] = 0;
                if self.regs[a] > self.regs[b]{
                    self.regs[c] = 1;
                }
            },
            Opcode::Eqir => {
                self.regs[c] = 0;
                if (a as i32) == self.regs[b]{
                    self.regs[c] = 1;
                }
            },
            Opcode::Eqri => {
                self.regs[c] = 0;
                if self.regs[a] == (b as i32){
                    self.regs[c] = 1;
                }
            },
            Opcode::Eqrr => {
                self.regs[c] = 0;
                if self.regs[a] == self.regs[b]{
                    self.regs[c] = 1;
                }
            },
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

    let before_re = Regex::new(r"Before: \[(\d), (\d), (\d), (\d)\]").unwrap();
    let instruction_re = Regex::new(r"(\d) (\d) (\d) (\d)").unwrap();
    let after_re = Regex::new(r"After:  \[(\d), (\d), (\d), (\d)\]").unwrap();


    let mut cpu = CPU::new([0,0,0,0]);
    let line_iter = lines.iter();
    let mut count = 0;
    let mut jj = 0;
    for i in (0..lines.len()).step_by(4) {
        let before = &lines[i];
        if !before_re.is_match(before){
            break;
        }

        let mut init_regs: [i32; 4] = [0; 4];
        for cap in before_re.captures_iter(before){
            let reg0: i32 = cap[1].trim().parse().expect("expected i32");
            let reg1: i32 = cap[2].trim().parse().expect("expected i32");
            let reg2: i32 = cap[3].trim().parse().expect("expected i32");
            let reg3: i32 = cap[4].trim().parse().expect("expected i32");
            init_regs = [reg0, reg1, reg2, reg3];
        }

        let instruction = &lines[i+1];
        let mut input_a: i32 = 0;
        let mut input_b: i32 = 0;
        let mut output: i32 = 0;
        for cap in instruction_re.captures_iter(instruction){
            input_a = cap[2].trim().parse().expect("expected i32");
            input_b = cap[3].trim().parse().expect("expected i32");
            output = cap[4].trim().parse().expect("expected i32");
        }

        let after = &lines[i+2];
        let mut after_regs: [i32; 4] = [0; 4];

        for cap in after_re.captures_iter(after) {
            let mut reg0: i32 = cap[1].trim().parse().expect("expected i32");
            let mut reg1: i32 = cap[2].trim().parse().expect("expected i32");
            let mut reg2: i32 = cap[3].trim().parse().expect("expected i32");
            let mut reg3: i32 = cap[4].trim().parse().expect("expected i32");
            after_regs = [reg0, reg1, reg2, reg3];
        }


        let mut local_count = 0;
        for i in 0..OPCODES.len(){
            let opcode: Opcode = OPCODES[i];
            cpu.set_regs(init_regs);
            cpu.execute_instruction(opcode, input_a as usize,
                input_b as usize, output as usize);
            if cpu.regs == after_regs{
                local_count +=1
            }

        }
        println!("{:?} {}", jj, local_count);
        if local_count >= 3 {
            count += 1;
        }
        jj+=1;

        // println!("{:?}", before);
        // println!("{:?}", instruction);
        // println!("{:?}", after);
        // let mut input = String::new();
        // let string = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    }

}


