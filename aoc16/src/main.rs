use std::iter;
use std::io::{Read, self};

fn main() {
    let mut input_string = String::new();
    std::io::stdin().lock().read_to_string(&mut input_string).unwrap();

    let input_vec: Vec<u32> = iter::repeat(input_string
        .chars()
        .map(|x| x.to_digit(10).unwrap() )
        )
        .cycle()
        .take(1000)
        .flatten()
        .collect();
    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];

    let calculate_step = |vec: &Vec<u32>, n| {
        let mut result = 0;
        let mut pattern = base_pattern
            .iter()
            .flat_map(|x| iter::repeat(x)
                      .take(n))
            .cycle();
        pattern.next(); // skip first value
        for (val, mult) in vec.iter().zip(pattern) {
            result += *val as i32 * *mult;
            result = result % 10
        }
        (result.abs() % 10) as u32
    };

    let mut result = input_vec.clone();
    for i in 0..100 {
        result = (1..=result.len())
            .into_iter()
            .map(|x| calculate_step(&result, x))
            .collect();
        println!("{}", i);
    };
}
