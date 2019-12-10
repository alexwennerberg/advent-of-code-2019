use std::io::{self, Read};

#[derive(Debug)]
pub struct Program {
    memory: Vec<i32>,
    pointer: usize,
    input: Vec<i32>,
}

impl Program {
    pub fn from_stdin() -> Program {
        let mut program_string = String::new();
        io::stdin().read_to_string(&mut program_string).unwrap();
        Program::from_string(program_string)
    }

    pub fn from_string(program_string: String) -> Program {
        let memory: Vec<i32> = program_string
            .replace("\n", "") // Remove ending newline
            .split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        Program {
            memory: memory,
            pointer: 0,
            input: vec![]
        }
    }

    pub fn run_until_stop(self: &mut Self) {
        loop {
            let (_, stop) = self.step();
            if stop {
                return
            }
        }
    }

    pub fn run_until_output(self: &mut Self) -> i32 {
        loop {
            let (output, _) = self.step();
            match output {
                Some(i) => return i,
                None => {},
            }
        }
    }

    pub fn step(self: &mut Self) -> (Option<i32>, bool) {
        // returns output if there is some
        let operation = self.memory[self.pointer];
        let opcode = operation % 100;
        let mut output = None;
        let mut stop = false;
       if opcode == 1 {
            let ptr = self.memory[self.pointer + 3];
            let params = self.get_values(operation, 2);
            self.memory[ptr as usize] = params[0] + params[1];
            self.pointer += 4
        }
        else if opcode == 2 {
            let ptr = self.memory[self.pointer + 3];
            let params = self.get_values(operation, 2);
            self.memory[ptr as usize] = params[0] * params[1];
            self.pointer += 4
        }
        else if opcode == 3 {
            let ptr = self.memory[self.pointer + 1];
            self.memory[ptr as usize] = self.input.pop().unwrap();
            self.pointer += 2
            // input
        }
        else if opcode == 4 {
            let params = self.get_values(operation, 1);
            output = Some(params[0]);
            println!("Result {}", params[0]);
            self.pointer += 2
            // output
        }
        else if opcode == 5 {
            // jump if true
            let params = self.get_values(operation, 2);
            if params[0] != 0 {
                self.pointer = params[1] as usize;
            }
            else {
                self.pointer += 3
            }
        }
        else if opcode  == 6 {
        // jump if false
            let params = self.get_values(operation, 2);
            if params[0] == 0 {
                self.pointer = params[1] as usize;
            }
            else {
                self.pointer += 3
            }
        }
        else if opcode == 7 {
            // less than 
            let ptr = self.memory[self.pointer + 3];
            let params = self.get_values(operation, 2);
            if params[0] < params[1] { // hm
                self.memory[ptr as usize] = 1;
            }
            else {
                self.memory[ptr as usize] = 0;
            }
            self.pointer += 4
        }
        else if opcode == 8 {
            let ptr = self.memory[self.pointer + 3];
            let params = self.get_values(operation, 2);
            if params[0] == params[1] {
                self.memory[ptr as usize] = 1;
            }
            else {
                self.memory[ptr as usize] = 0;
            }
            self.pointer += 4
        }
        else if opcode == 99 {
            stop = true;
            
        };
        (output, stop)
    }

    fn get_values(self: &Self, operation: i32, num_params: usize) -> Vec<i32>{
        let mut out = Vec::new();
        for i in 1..=num_params {
            let mut value = self.memory[(self.pointer+i) as usize];
            let mode = (operation / (i32::pow(10, i as u32)*10)) % 10;
            if mode == 0 { // position mode
                // value used as pointer
                value = self.memory[value as usize];
            }
            out.push(value);
        }
        out
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_program_constructor() {
        let program = Program::from_string("1,2,3".to_string());
        assert_eq!(program.memory, vec![1,2,3]);
        assert_eq!(program.pointer, 0)
    }

    #[test]
    fn day2_test() {
        for test in [("1,0,0,0,99", vec![2,0,0,0,99]), ("2,3,0,3,99", vec![2,3,0,6,99]), ("2,4,4,5,99,0", vec![2,4,4,5,99,9801]), ("1,1,1,4,99,5,6,0,99", vec![30,1,1,4,2,5,6,0,99])].iter() {
            let mut program = Program::from_string(test.0.to_string());
            program.run_until_stop();
            assert_eq!(program.memory, test.1);
        }
    }

    #[test]

    fn day5_test() {
        let mut program = Program::from_string("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string());
        program.input = vec![9];
        assert_eq!(program.run_until_output(), 1001)
    }

}
