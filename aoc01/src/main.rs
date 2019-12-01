use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    partone(&input);
    parttwo(&input);

}

fn partone_calculate_fuel(mass: i32) -> i32 {
    return (mass / 3) - 2
}

fn parttwo_calculate_fuel(mass: i32) -> i32 {
    let mut total = 0;
    let mut current = mass / 3 - 2;
    while current > 0 {
        total += current;
        current = current / 3 -2;
    }
    return total
}

fn partone(input: &String) {
    let mut total_fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse().unwrap();
        total_fuel += partone_calculate_fuel(mass);
    }
    println!("part 1: {}", total_fuel);
}

fn parttwo(input: &String) {
    let mut total_fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse().unwrap();
        total_fuel += parttwo_calculate_fuel(mass);
    }
    println!("part 2: {}", total_fuel);
}
