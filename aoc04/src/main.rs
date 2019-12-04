use std::collections::HashSet;
use std::iter::FromIterator;


const RANGE: (i32, i32) = (134564, 585159);

fn main() {
    // part 1
    let result: Vec<Vec<i32>> = (RANGE.0..=RANGE.1)
        .map(|n| number_to_vec(n))
        .filter(|v| v.windows(2).all(|w| w[0] <= w[1])) // check is sorted
        .filter(|v| HashSet::<&i32>::from_iter(v.iter()).len() != v.len())
        .collect();
    println!("part 1: {}", result.len());
    // Much better solution:
    // https://www.reddit.com/r/adventofcode/comments/e5u5fv/2019_day_4_solutions/f9mkj4v?utm_source=share&utm_medium=web2x
    let part2 = result
                    .iter()
                    .filter(|v| { 
                        let mut last_char = -1;
                        let mut seq = 1;
                                for (i,d) in v.iter().enumerate() {
                                    if last_char == *d {
                                        seq += 1;
                                        if i == v.len()-1 && seq == 2{
                                            return true;
                                        }
                                    }
                                    else {
                                        if seq == 2 {
                                            return true;
                                        }
                                        seq = 1
                                    }
                                    last_char = *d;
                                }
                    return false}).count();
    println!("part 2: {}", part2);
}

// copied from https://users.rust-lang.org/t/how-to-convert-a-number-to-numeric-vec/10404
fn number_to_vec(n: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}
