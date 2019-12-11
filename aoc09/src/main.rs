use intcode;

fn main() {
    let mut program = intcode::Program::from_stdin();
    program.input = vec![2];
    program.run_until_stop();
}
