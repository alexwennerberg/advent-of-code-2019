use std::io::{self, Read};

// Refactored based on https://github.com/sethetter/aoc-2019/blob/master/day-08/src/main.rs after
// completing
fn main() {
    let layer_size = 25 * 6;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.pop();
    let count_digit = |f: &[u32], digit| f.iter().filter(|x| *x == &digit).count();

    // part 1
    let nums = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let fewest_zeros = nums
        .chunks(layer_size)
        .min_by(|a, b|  count_digit(a, 0).cmp(&count_digit(b, 0)))
        .unwrap();

   let result = count_digit(fewest_zeros, 1) * count_digit(fewest_zeros, 2);

    println!("{}", result);
}
