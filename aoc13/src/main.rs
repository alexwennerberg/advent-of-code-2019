use intcode;

fn main() {
    let mut program = intcode::Program::from_stdin();
    let mut program2 = program.clone();
    program.run_until_stop();
    let part1 = program.output
        .chunks(3)
        .filter(|x| x[2] == 2).count();
    println!("{}", part1);
    program2.memory[0] = 2;
    program2.input = vec![0];
}

