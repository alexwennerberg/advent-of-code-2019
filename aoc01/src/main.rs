/// Solution to Day 1: https://adventofcode.com/2019/day/1

use std::io::{self, Read};

fn main() {
    // Determine the total fuel requirements for a list of modules based on their mass
    // Given the "base" (Part 1) fuel calculation and "sophisticated" (Part 2) calculation
    // Input is a list of module masses (positive integers) separated by line breaks
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let fuel_functions: Vec<fn(i32) -> i32> = vec![base_calculate_fuel, sophisticated_calculate_fuel];
    for (i, fuel_function) in fuel_functions.iter().enumerate() {
        let result: i32 = input.lines().map(|line| fuel_function(line.parse::<i32>().unwrap())).sum();
        println!("part {}: {}", i+1, result);
    }
}

fn base_calculate_fuel(mass: i32) -> i32 {
    // (Part 1)
    // Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
    // required for a module, take its mass, divide by three, round down, and subtract 2.
    return (mass / 3) - 2
}

fn sophisticated_calculate_fuel(mass: i32) -> i32 {
    // (Part 2)
    // Fuel itself requires fuel just like a module - take its mass, divide by three, round down,
    // and subtract 2. However, that fuel also requires fuel, and that fuel requires fuel, and so
    // on. Any mass that would require negative fuel should instead be treated as if it requires
    // zero fuel; the remaining mass, if any, is instead handled by wishing really hard, which has
    // no mass and is outside the scope of this calculation.
    let mut total = 0;
    let mut current = base_calculate_fuel(mass);
    while current > 0 {
        total += current;
        current = base_calculate_fuel(current);
    }
    return total
}
