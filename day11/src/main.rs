use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::cmp;

// The coordinates of a particular point is its location along the two diagonal axes, like this:
//       ____      ____
// \____/ 3 2\____/ 2 3\____/
// / 3 1\____/ 2 2\____/ 1 3\
// \____/ 2 1\____/ 1 2\____/
// / 2 0\____/ 1 1\____/ 0 2\
// \____/ 1 0\____/ 0 1\____/
// /1 -1\____/ 0 0\____/-1 1\
// \____/ 0-1\____/-1 0\____/
// / 0-2\____/-1-1\____/-2 0\
// \____/-1-2\____/-2-1\____/
// /-1-3\____/-2-2\____/-3-1\
// \____/-2-3\____/-3-2\____/
//
// Distance from zero to a particular point is the maximum distance along any of the 3 axes.  The
// two diagonal axes are given by the coordinates, and the third (vertical) axis is axis2 - axis1.

// Convert the input into a Vec of coordinate steps
fn get_input() -> Vec<(i32, i32)> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    return Vec::from_iter(
        input_str
            .split(',')
            .map(|x| match x {
                "n"  => (1, 1),
                "ne" => (0, 1),
                "se" => (-1, 0),
                "s"  => (-1, -1),
                "sw" => (0, -1),
                "nw" => (1, 0),
                text => panic!("Unexpected input {}", text)
            }));
}

fn distance_from_zero(location: (i32, i32)) -> i32 {
    let z = location.1 - location.0;
    return cmp::max(cmp::max(location.0.abs(), location.1.abs()), z.abs());
}

fn distances(steps: &[(i32, i32)]) -> (i32, i32) {
    let mut location = (0, 0);
    let mut max_dist = 0;
    for step in steps.iter() {
        location.0 += step.0;
        location.1 += step.1;
        max_dist = cmp::max(max_dist, distance_from_zero(location));
    }
    return (distance_from_zero(location), max_dist);
}

fn main() {
    let input = get_input();
    let (max_distance, end_distance) = distances(&input);
    println!("Part 1: {}", max_distance);
    println!("Part 2: {}", end_distance);
}
