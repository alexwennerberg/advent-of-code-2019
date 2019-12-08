#[macro_use] extern crate itertools;

use itertools::Itertools;
use std::io::{self, Read};

fn main() {
    let mut program_string = String::new();
    io::stdin().read_to_string(&mut program_string).unwrap();
    program_string.pop(); // Remove ending newline (?)
    let program: Vec<i32>= program_string
        .split(",")
        .map(|i| {
            i.parse::<i32>().unwrap()})
        .collect();
    // Part 1
    let result = (0..=4)
        .permutations(5)
        .inspect(|x| println!("Running permutation {:?}", x))
        .map(|x| x.iter().fold(0, |amplitude, ps| run_program(program.clone(), amplitude, *ps)))
        .max();
    println!("part 1: {:?}", result);
    //run_program(program.clone(), input);
    println!("part 2: {:?}", result);
    // part 2
}


fn run_program(mut program: Vec<i32>, input: i32, phase_setting: i32) -> i32{
    let mut instruction_pointer = 0;
    let mut result = -1;
    let mut input_set = false;
    loop {
        let current_instruction = program[instruction_pointer];
        let opcode = current_instruction % 100;
        let get_values = |parameters| {
            let mut out = Vec::new();
            for i in 1..=parameters {
                let mut value = program[(instruction_pointer+i) as usize];
                let mode = (current_instruction / (i32::pow(10, i as u32)*10)) % 10;
                if mode == 0 { // position mode
                    // value used as pointer
                    value = program[value as usize];
                }
                out.push(value);
            }
            out
        };
        if opcode == 1 {
            let ptr = program[instruction_pointer + 3];
            let params = get_values(2);
            program[ptr as usize] = params[0] + params[1];
            instruction_pointer += 4
        }
        else if opcode == 2 {
            let ptr = program[instruction_pointer + 3];
            let params = get_values(2);
            program[ptr as usize] = params[0] * params[1];
            instruction_pointer += 4
        }
        else if opcode == 3 {
            let ptr = program[instruction_pointer + 1];
            if !input_set {
                program[ptr as usize] = phase_setting;
                input_set = true;
            }
            else {
                program[ptr as usize] = input;
            }
            instruction_pointer += 2
            // input
        }
        else if opcode == 4 {
            let params = get_values(1);
            result = params[0];
            println!("Result {}", params[0]);
            instruction_pointer += 2
            // output
        }
        else if opcode == 5 {
            // jump if true
            let params = get_values(2);
            if params[0] != 0 {
                instruction_pointer = params[1] as usize;
            }
            else {
                instruction_pointer += 3
            }
        }
        else if opcode  == 6 {
        // jump if false
            let params = get_values(2);
            if params[0] == 0 {
                instruction_pointer = params[1] as usize;
            }
            else {
                instruction_pointer += 3
            }
        }
        else if opcode == 7 {
            // less than 
            let ptr = program[instruction_pointer + 3];
            let params = get_values(2);
            if params[0] < params[1] { // hm
                program[ptr as usize] = 1;
            }
            else {
                program[ptr as usize] = 0;
            }
            instruction_pointer += 4
        }
        else if opcode == 8 {
            let ptr = program[instruction_pointer + 3];
            let params = get_values(2);
            if params[0] == params[1] {
                program[ptr as usize] = 1;
            }
            else {
                program[ptr as usize] = 0;
            }
            instruction_pointer += 4
        }
        else if opcode == 99 {
            return result;
        }
    }
}
