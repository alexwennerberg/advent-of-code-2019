use std::io::{self, Read};
use std::collections::HashMap;


fn main() {
    // read input to hashmap
    let mut input_data = String::new();
    io::stdin().read_to_string(&mut input_data).unwrap();
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    let pairs: Vec<Vec<&str>> = input_data
        .lines()
        .map(|line| line.split(")").collect()).collect();
    for pair in pairs {
        orbits.insert(pair[1], pair[0]);
    }

    // part 1
    let mut count = 0;
    for (_, mut v) in orbits.iter() {
        count += 1; // "direct orbits"
        loop {
            match orbits.get(v) {
                Some(orbit) => {
                    count += 1; // "indirect orbits"
                    v = orbit
                }
                None => break
            }
        }
    }
    println!("part 1: {}", count);

    let mut you_orbit = orbits.get("YOU").unwrap();
    let mut santa_orbits = orbits.get("SAN").unwrap();
    let mut you_descendents = Vec::new();
    let mut santa_descendents = Vec::new();
    you_descendents.push(you_orbit);
    loop {
        match orbits.get(you_orbit) {
            Some(orbit) => {
                you_descendents.push(orbit);
                you_orbit = orbit
            }
            None => break
        }
    }
    santa_descendents.push(santa_orbits);
    loop {
        match orbits.get(santa_orbits) {
            Some(orbit) => {
                santa_descendents.push(orbit);
                santa_orbits = orbit
            }
            None => break
        }
    }

    for (i, object) in santa_descendents.iter().enumerate() {
        if you_descendents.contains(object) {
            let you_transfers = you_descendents.iter().position(|r| r == object).unwrap();
            println!("part 2: {}", you_transfers + i);
            return;
        }
    }


}
