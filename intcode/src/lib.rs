use std::io::{self, Read};

#[derive(Debug)]
pub struct Program {
    memory: Vec<i64>,
    pointer: usize,
    relative_base: i64,
    pub input: Vec<i64>,
}

impl Program {
    pub fn from_stdin() -> Program {
        let mut program_string = String::new();
        io::stdin().read_to_string(&mut program_string).unwrap();
        Program::from_string(program_string)
    }

    pub fn from_string(program_string: String) -> Program {
        let mut memory: Vec<i64> = program_string
            .replace("\n", "") // Remove ending newline
            .split(",")
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        for i in 0..100000 {
            memory.push(0);
        }
        Program {
            memory: memory,
            pointer: 0,
            relative_base: 0,
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

    pub fn run_until_output(self: &mut Self) -> Option<i64> {
        loop {
            let (output, stop) = self.step();
            match output {
                Some(i) => return Some(i),
                None => {},
            }
            if stop {
                return None;
            }
        }
    }

    pub fn step(self: &mut Self) -> (Option<i64>, bool) {
        // returns output if there is some
        let operation = self.memory[self.pointer];
        let opcode = operation % 100;
        let mut output = None;
        let mut stop = false;
        if opcode == 1 {
            let mut ptr = self.memory[self.pointer + 3];
            if operation / 10000 == 2 {
                ptr = self.relative_base + ptr;
            }
            let params = self.get_values(operation, 2);
            self.memory[ptr as usize] = params[0] + params[1];
            self.pointer += 4
        }
        else if opcode == 2 {
            let mut ptr = self.memory[self.pointer + 3];
            if operation / 10000 == 2 {
                ptr = self.relative_base + ptr;
            }
            let params = self.get_values(operation, 2);
            self.memory[ptr as usize] = params[0] * params[1];
            self.pointer += 4
        }
        else if opcode == 3 {
            let mut ptr = self.memory[self.pointer + 1];
            if operation / 10000 == 2 {
                ptr = self.relative_base + ptr;
            }
            if operation == 203 {
                self.memory[(self.relative_base + ptr) as usize] = self.input.pop().unwrap();
            }
            else  {
                self.memory[ptr as usize] = self.input.pop().unwrap();
            }
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
            let mut ptr = self.memory[self.pointer + 3];
            if operation / 10000 == 2 {
                ptr = self.relative_base + ptr;
            }
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
            let mut ptr = self.memory[self.pointer + 3];
            if operation / 10000 == 2 {
                ptr = self.relative_base + ptr;
            }
            let params = self.get_values(operation, 3);
            if params[0] == params[1] {
                self.memory[ptr as usize] = 1;
            }
            else {
                self.memory[ptr as usize] = 0;
            }
            self.pointer += 4
        }
        else if opcode == 9 {
            let params = self.get_values(operation, 1);
            self.relative_base += params[0];
            self.pointer += 2
            //implement
        }
        else if opcode == 99 {
            stop = true;
            
        };
        (output, stop)
    }

    fn get_values(self: &Self, operation: i64, num_params: usize) -> Vec<i64>{
        let mut out = Vec::new();
        for i in 1..=num_params {
            let mut value = self.memory[(self.pointer+i) as usize];
            let mode = (operation / (i64::pow(10, i as u32)*10)) % 10;
            if mode == 0 { // position mode
                // value used as pointer
                value = self.memory[value as usize];
            }
            if mode == 2 { // relative mode
                value = self.memory[(self.relative_base as i64 + value) as usize] // usize?
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
    fn day5_test() {
        let mut program = Program::from_string("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string());
        program.input = vec![9];
        assert_eq!(program.run_until_output().unwrap(), 1001)
    }

    #[test]
    fn day7_test() {
        let mut program = Program::from_string("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string());
        let mut output = vec![];
        let mut stop = false;
        while !stop {
            match program.run_until_output() {
                Some(o) => output.push(o),
                None => stop = true
            }
        }
        assert_eq!(output, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    }

    #[test]
    fn day7_test2() {
        let mut program = Program::from_string("1102,34915192,34915192,7,4,7,99,0".to_string());
        assert_eq!(program.run_until_output().unwrap(), 1219070632396864)
    }

    #[test]
    fn day7_test3() {
        let mut program = Program::from_string("104,1125899906842624,99".to_string());
        assert_eq!(program.run_until_output().unwrap(), 1125899906842624)
    }

}
