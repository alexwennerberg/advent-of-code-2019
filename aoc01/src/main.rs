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
    // (Part 1)
    // Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
    // required for a module, take its mass, divide by three, round down, and subtract 2.
    return (mass / 3) - 2
}

fn sophisticated_calculate_fuel(mass: u32) -> u32 {
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
