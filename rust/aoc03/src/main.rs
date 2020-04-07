use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let wire1_path: Vec<&str> = lines[0].split(",").collect();
    let wire2_path: Vec<&str> = lines[1].split(",").collect();
    let points1 =  get_points(wire1_path.clone());
    let points2 = get_points(wire2_path.clone());
    let intersections =  points1.intersection(&points2);
    // part 1
    
    let nearest_intersection = intersections.clone()
        .map(|t| t.0.abs() + t.1.abs())
        .min()
        .unwrap();
    println!("part 1: {}", nearest_intersection);
    // part 2
    let nearest_intersection = intersections.clone()
        .map(|t| get_length_to_point(wire1_path.clone(), *t) + get_length_to_point(wire2_path.clone(), *t)) // horrible
        .min()
        .unwrap();
    println!("part 2: {}", nearest_intersection);
    // println!("part 2: {}", intersection);
}

fn get_length_to_point(path: Vec<&str>, point: (i32, i32)) -> i32 {
    // inefficient
    let mut current_x = 0;
    let mut current_y = 0;
    let mut wire_len = 0;
    for item in path {
        let direction = &item[0..1];
        let amount: i32 = item[1..].parse().unwrap();
        for _ in 0..amount {
            match direction {
                "U" => current_y += 1,
                "D" => current_y -= 1,
                "L" => current_x -= 1,
                "R" => current_x += 1,
                _ => ()
            }
            wire_len += 1;
            if current_x == point.0 && current_y == point.1 {
                return wire_len;
            }
        }
    }
    return -1; // bad

}

fn get_points(path: Vec<&str>) -> HashSet<(i32, i32)>{
    let mut path_points = HashSet::new();
    let mut current_x = 0;
    let mut current_y = 0;
    for item in path {
        let direction = &item[0..1];
        let amount: i32 = item[1..].parse().unwrap();
        for _ in 0..amount {
            match direction {
                "U" => current_y += 1,
                "D" => current_y -= 1,
                "L" => current_x -= 1,
                "R" => current_x += 1,
                _ => ()
            }
            path_points.insert((current_x, current_y));
        }
    }
    return path_points;
}
