use intcode;

fn main() {
    let mut program = intcode::Program::from_stdin();
    program.run_until_stop();
}
