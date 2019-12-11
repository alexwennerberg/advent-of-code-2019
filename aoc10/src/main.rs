use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let asteroids: HashSet<(i32,i32)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| line
             .chars()
             .enumerate()
             .filter(|(_, space)| *space == '#')
             .map(move |(j, _)| (j as i32 ,i as i32 )))
        .flatten()
        .collect();

    let best = asteroids
        .iter()
        .map(|a1| {
            let count = asteroids.iter().filter(
                |a2| {
                    *a2 != a1 && asteroids.iter().filter(|a3|{
                        let result = check_if_between(*a1,**a2,**a3);
                        result
                           }).count() == 0 // includes self
                }
                ).count();
            count
        }).max().unwrap();
    println!("{:?}", best);
}

fn num_steps(mut m: i32, mut n: i32) -> i32 {
    // gcd modified for 0
    if n == 0 {
        return m.abs()
    }
    else if m == 0 {
        return n.abs()
    }
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
        }
    n.abs()
    }

fn check_if_between(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> bool {
    if c == a || c == b {
        return false
    }
    let badiff = (b.0  -a.0  , b.1 -a.1 );
    let num_steps = num_steps(badiff.0, badiff.1);
    let step = (badiff.0 / num_steps, badiff.1 / num_steps);
    let mut tracer = (a.0 , a.1 );
    for _ in 0..=num_steps {
        tracer = (tracer.0  + step.0, tracer.1  + step.1);
        if tracer == (c.0 , c.1 ) {
            return true
        }
        else if tracer == (b.0 , b.1 ) {
            return false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_of_sight() {
        let a=(0,0);
        let b=(2,2);
        let c=(1,1);
        assert!(check_if_between(a, b, c));
    }
    #[test]
    fn test_not_inline_of_sight() {
        let a=(0,0);
        let b=(2,3);
        let c=(1,1);
        assert!(!check_if_between(a, b, c))
    }
    #[test]
    fn test_wonky() {
        let a=(0,0);
        let b=(4,6);
        let c=(2,3);
        assert!(check_if_between(a, b, c))
}
    #[test]
    fn test_inf_slope() {
    let a=(0,0);
    let b=(0,2);
    let c=(0,1);
    assert!(check_if_between(a, b, c))
    }
    #[test]
    fn test_0_slope() {
    let a=(0,0);
    let b=(2,0);
    let c=(1,0);
    assert!(check_if_between(a, b, c))
    }
    #[test]
    fn test_b_is_c() {
    let a=(0,0);
    let b=(0,2);
    let c=(0,2);
    assert!(!check_if_between(a, b, c));
    }

    #[test]
    fn test_a_is_c() {
    let a=(0,0);
    let b=(0,2);
    let c=(0,0);
    assert!(!check_if_between(a, b, c))
    }
    #[test]
    fn test_broken_case() {
    let a=(3,2);
    let b=(1,2);
    let c=(4,2);
    assert!(!check_if_between(a, b, c))
    }
    #[test]
    fn test_broken_case2() {
    let a=(1,2);
    let b=(4,2);
    let c=(0,2);
    assert!(!check_if_between(a, b, c))
    }
}
