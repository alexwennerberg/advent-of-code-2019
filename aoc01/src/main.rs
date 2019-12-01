/// Solution to Day 1: https://adventofcode.com/2019/day/1

use std::io::{self};
use std::io::prelude::*;

fn main() {
    // Determine the total fuel requirements for a list of modules based on their mass
    // Given the "base" (Part 1) fuel calculation and "sophisticated" (Part 2) calculation
    // Input is a list of module masses (positive integers) separated by line breaks
    // Overflows if total mass >=  2^32
    let mut partone: u32 = 0;
    let mut parttwo: u32 = 0;
    for line in io::stdin().lock().lines() {
        let parsed = line.unwrap().parse::<u32>().unwrap();
        partone += base_calculate_fuel(parsed);
        parttwo += sophisticated_calculate_fuel(parsed);
    }
    println!("part 1: {}", partone);
    println!("part 2: {}", parttwo);
}

fn base_calculate_fuel(mass: u32) -> u32 {
    return (mass / 3) - 2
}

fn sophisticated_calculate_fuel(mass: u32) -> u32 {
    let mut total = 0;
    let mut current = base_calculate_fuel(mass);
    while current > 0 {
        total += current;
        current = base_calculate_fuel(current);
    }
    return total
}
