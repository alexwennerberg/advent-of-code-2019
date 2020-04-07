use std::io::{self, Read};

// Refactored based on https://github.com/sethetter/aoc-2019/blob/master/day-08/src/main.rs after
// completing
fn main() {
    let height = 6;
    let width = 25;
    let layer_size = height * width;
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

    println!("part 1: {}", result);
    let layers: Vec<&[u32]> = nums
        .chunks(layer_size)
        .collect::<Vec<&[u32]>>();

    let mut image: Vec<Vec<u32>> = vec![];
    for layer in layers {
        // initialize
        for i in 0..height {
            match image.get(i) {
                Some(_) => (),
                None => image.push(vec![])
            }
            for j in 0..width {
                let mychar = layer[i * width + j];
                let result = image[i].get(j);
                if result == None {
                    image[i].push(mychar)
                }
                else {
                    let newchar = match image[i][j] {
                        0 => 0,
                        1 => 1,
                        2 => mychar,
                        _ => panic!()
                    };
                    image[i as usize][j] = newchar;
                }
            }
        }
    }


    println!("part 2:");
    for line in image {
        for a in line {
            match a {
                1 => print!("O"),
                0 => print!("_"),
                _ => panic!()
            }
        }
        print!("\n");
    }
}
