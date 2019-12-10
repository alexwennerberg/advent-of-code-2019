#[macro_use] extern crate itertools;

use intcode;
use itertools::Itertools;
use std::io::{self, Read};

fn main() {
    let mut program_string = String::new();
    io::stdin().read_to_string(&mut program_string).unwrap();

    // Part 1
    //let result = (0..=4)
    //    .permutations(5)
    //    .inspect(|x| println!("Running permutation {:?}", x))
    //    .map(|x| x.iter().fold(0, |amplitude, ps| {
    //        let mut program = intcode::Program::from_string(program_string.clone());
    //        program.input = vec![amplitude, *ps];
    //        program.run_until_output().unwrap()
    //    }))
    //    .max();
    //println!("part 1: {:?}", result);
    //run_program(program.clone(), input);
    let mut max = 0;
    let mut best = vec![];
    for permutation in (5..=9).permutations(5) {
        let mut programs: Vec<intcode::Program> = vec![];
        for signal in permutation.iter() {
            let mut program = intcode::Program::from_string(program_string.clone());
            program.input = vec![*signal];
            program.step(); // read input
            programs.push(program);
        }
        let mut amplitude = 0;
        let mut doabreak = false;
        loop {
            for i in 0..programs.len() {
                println!("{:?}", programs[i]);
                programs[i].input = vec![amplitude];
                match programs[i].run_until_output() {
                    Some(a) => amplitude = a,
                    None => {
                        doabreak = true;
                        break
                    }
                }
            }
            if doabreak {
                break
            }
        }
        if amplitude > max {
            max = amplitude;
            best = permutation.clone();
        }
    }
    println!("part 2: {:?} {:?}", max, best);
    // part 2
}

