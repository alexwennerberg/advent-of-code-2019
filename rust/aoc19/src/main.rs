use std::io::{Read, self};
use intcode;

fn main() {
    let mut input_string = String::new();
    io::stdin().read_to_string(&mut input_string).unwrap();

    let mut count = 0;
    for x in 0..=49 {
        for y in 0..=49 {
            let mut program = intcode::Program::from_string(input_string.clone());
            program.input = vec![x,y];
            println!("{} {}", x, y);
            program.run_until_stop();
            let output = program.output;
            println!("{:?}", output);
            if output == vec![1] {
                count += 1
            }
        }
    }

    println!("{}", count);
}
