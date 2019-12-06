use std::io::{self, Read};
use std::collections::{HashMap, HashSet};
use std::iter::{successors, FromIterator};


fn main() {
    // read input to hashmap
    let mut input_data = String::new();
    io::stdin().read_to_string(&mut input_data).unwrap();
    let orbits: HashMap<&str, &str> = input_data
        .lines()
        .map(|line| {
            let a: Vec<&str> = line.split(")").collect();
            (a[1], a[0])}).collect();

    // part 1
    let orbit_count = orbits
        .iter()
        .map(|(_,b)| successors(Some(b), |suborbit| orbits.get(*suborbit)))
        .flatten() // same as flatmap
        .count();

    println!("part 1: {}", orbit_count);

    let suborbits = |orbit| successors(Some(orbit), |suborbit| orbits.get(*suborbit));
    let you: Vec<&&str> = suborbits(&"YOU").collect();
    let santa: Vec<&&str> = suborbits(&"santa").collect();
    let shared_descendents: HashSet<&&&&str> = HashSet::<Vec<&&&str>>::from_iter(you.iter()).intersection(&HashSet::from_iter(santa.iter()));
    for (i, item) in suborbits(&"YOU").enumerate() {
        if shared_descendents.contains(&&item) {
            let result = i + suborbits(&"SANTA").position(|r| r == item).unwrap();
            println!("part 2: {}", result);
        }
    }
        
}
