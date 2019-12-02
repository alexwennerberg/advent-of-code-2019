use std::io::{self, Read};

fn main() {
    let mut program_string = String::new();
    io::stdin().read_to_string(&mut program_string).unwrap();
    program_string.pop(); // Remove ending newline (?)
    let program: Vec<usize>= program_string
        .split(",")
        .map(|i| {
            i.parse::<usize>().unwrap()})
        .collect();
    // Part 1
    let noun = 12;
    let verb = 2;
    let result = run_program(program.clone(), noun, verb);
    println!("{}", result);
    // Part 2
    for noun in 0..100 {
        for verb in 0..100 {
            let result = run_program(program.clone(), noun, verb);
            if result == 19690720 {
                println!("part 2: {}", 100*noun + verb)
            }
        }
    }
    // part 2
}

fn run_program(mut program: Vec<usize>, noun: usize, verb: usize) -> usize {
    // Noun and verb bust be in range 0-99 inclusive
    program[1] = noun;
    program[2] = verb;
    let mut i = 0;
    let mut value = program[i];
    while value != 99 {
        let result = program[i+3];
        let op1 = program[i+1];
        let op2 = program[i+2];
        // println!("Running program {} operating on values {} {} into position {}", value, op1, op2, result);
        if value == 1 {
            program[result] = program[op1] + program[op2];
        }
        else if value == 2 {
            program[result] = program[op1] * program[op2];
        }
        i += 4;
        value = program[i];
    }
    return program[0]
}
