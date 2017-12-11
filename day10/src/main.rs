use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

fn get_input_bytes() -> Vec<u8> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut result = Vec::new();
    f.read_to_end(&mut result).expect("Failed to read file");
    return result;
}

fn get_input_comma_separated() -> Vec<u8> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    return Vec::from_iter(
        input_str
            .split(',')
            .map(|x| x.parse::<u8>().expect("Malformed input")),
    );
}

fn init_start_knot(knot: &mut [u8; 256]) {
    for (i, v) in knot.iter_mut().enumerate() {
        *v = i as u8;
    }
}

fn knot_round<T>(input: T, knot: &mut [u8], position: &mut u8, skip: &mut u8)
where
    T: Iterator<Item = u8>
{
    assert!(knot.len() == 256, "knot len: {}", knot.len());
    for length in input {
        for i in 0..(length / 2) {
            let front = position.wrapping_add(i);
            // back = position - length - 1 - i
            let back = position.wrapping_add(length).wrapping_sub(1).wrapping_sub(i);
            knot.swap(front as usize, back as usize);
        }
        *position = position.wrapping_add(length).wrapping_add(*skip);
        *skip = skip.wrapping_add(1);
    }
}

fn knot_hash(input: &[u8]) -> [u8; 16]
{
    let mut knot = [0; 256];
    init_start_knot(&mut knot);
    let mut position = 0;
    let mut skip = 0;

    for _ in 0..64 {
        let extended_input = input.iter().chain([17, 31, 73, 47, 23].iter()).cloned();
        knot_round(extended_input, &mut knot, &mut position, &mut skip);
    }

    let mut result = [0; 16];
    let mut knot_iter = knot.iter();
    for v in result.iter_mut() {
        *v = knot_iter.by_ref().take(16).fold(0, |acc, x| acc ^ x);
    }

    return result;
}

fn main() {
    let input = get_input_comma_separated();
    let mut part1_knot = [0; 256];
    init_start_knot(&mut part1_knot);
    knot_round(input.iter().cloned(), &mut part1_knot, &mut 0, &mut 0);
    println!("Part 1: {}", part1_knot[0] as u32 * part1_knot[1] as u32);

    let part2_hash = knot_hash(&get_input_bytes());
    let hash_str = String::from_iter(part2_hash.iter().map(|b| format!("{:02X}", b)));
    println!("Part 2: {}", hash_str);
}
