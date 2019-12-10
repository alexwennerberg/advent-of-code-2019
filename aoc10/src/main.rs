use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let asteroids: HashSet<(usize,usize)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| line
             .chars()
             .enumerate()
             .filter(|(_, space)| *space == '#')
             .map(move |(j, _)| (j,i)))
        .flatten()
        .collect();

    let best = asteroids
        .iter()
        .max_by_key(|a1| {
            let count = asteroids.iter().filter(
                |a2| {
                    **a2 != **a1 && asteroids.iter().filter(|a3|{
                        check_if_between(**a1,**a2,**a3)
                           }).count() > 1 // includes self
                }
                ).count();
            println!("Count for {:?}: {}", a1, count);
            count
        });
    println!("{:?}", best);
}

fn check_if_between(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> bool {
    // checks if c is between a and b
    let xdiffab = b.0 as i32 - a.0 as i32 ;
    let ydiffab = b.1 as i32 - a.1 as i32;
    let xdiffac = c.0 as i32 - a.0 as i32;
    let ydiffac = c.1  as i32- a.1 as i32;
    // flat special case
    if xdiffab == 0 || xdiffac == 0 {
        return xdiffab == xdiffac
    }
    if ydiffab == 0 || ydiffac == 0 {
        return xdiffab == xdiffac
    }
    let slopeac = (ydiffac as f32/xdiffac as f32);
    let slopeab = (ydiffab as f32/xdiffab as f32);
    return slopeac == slopeab

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
}
