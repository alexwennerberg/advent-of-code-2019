use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let fuel_functions: Vec<fn(i32) -> i32> = vec![base_calculate_fuel, sophisticated_calculate_fuel];
    for (i, fuel_function) in fuel_functions.iter().enumerate() {
        let result: i32 = input.lines().map(|line| fuel_function(line.parse::<i32>().unwrap())).sum();
        println!("part {}: {}", i+1, result);
    }
}

fn base_calculate_fuel(mass: i32) -> i32 {
    return (mass / 3) - 2
}

fn sophisticated_calculate_fuel(mass: i32) -> i32 {
    let mut total = 0;
    let mut current = base_calculate_fuel(mass);
    while current > 0 {
        total += current;
        current = base_calculate_fuel(current);
    }
    return total
}
