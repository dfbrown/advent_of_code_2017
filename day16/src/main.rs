#![feature(conservative_impl_trait)]

extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::mem;
use itertools::Itertools;

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn get_input_bytes() -> Vec<u8> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut result = Vec::new();
    f.read_to_end(&mut result).expect("Failed to read file");
    return result;
}

fn parse_ascii<T>(input: &[u8]) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    str::from_utf8(input)
        .expect("Not utf8?")
        .parse::<T>()
        .expect("Not a number")
}

fn dance_moves<'a>(input: &'a [u8]) -> impl Iterator<Item = DanceMove> + 'a {
    use DanceMove::*;
    return input.split(|&x| x == b',').map(|x| match x[0] {
        b's' => Spin(parse_ascii::<usize>(&x[1..])),
        b'x' => {
            let slash = x.iter()
                .position(|&x| x == b'/')
                .expect("No slash in exchange move");
            Exchange(
                parse_ascii::<usize>(&x[1..slash]),
                parse_ascii::<usize>(&x[slash + 1..]),
            )
        }
        b'p' => Partner(x[1], x[3]),
        c => panic!("Unexpected move {}", c as char),
    });
}

fn init_dance_line(dance_line: &mut [u8; 16]) {
    for (i, v) in dance_line.iter_mut().enumerate() {
        *v = b'a' + i as u8;
    }
}

fn spin(dance_line: &[u8; 16], spin_count: usize) -> [u8; 16] {
    let mut new_dance_line: [u8; 16];
    unsafe {
        new_dance_line = mem::uninitialized();
    }
    new_dance_line.iter_mut().set_from(dance_line.iter().cloned().cycle().skip(16 - spin_count as usize));
    return new_dance_line;
}

fn swap_partners(dance_line: &mut [u8; 16], p1: u8, p2: u8) {
    let p1_index = dance_line.iter().position(|&x| x == p1).expect("Partner 1 does not exist");
    let p2_index = dance_line.iter().position(|&x| x == p2).expect("Partner 2 does not exist");
    dance_line.swap(p1_index, p2_index);
}

fn part1(input_bytes: &[u8]) -> [u8; 16] {
    use DanceMove::*;
    let mut dance_line = [0u8; 16];
    init_dance_line(&mut dance_line);
    for dance_move in dance_moves(input_bytes) {
        match dance_move {
            Spin(count) => { dance_line = spin(&dance_line, count); }
            Exchange(index1, index2) => { dance_line.swap(index1, index2); }
            Partner(char1, char2) => { swap_partners(&mut dance_line, char1, char2); }
        }
    }
    return dance_line;
}

fn main() {
    let input_bytes = get_input_bytes();
    println!("Part 1: {}", str::from_utf8(&part1(&input_bytes)).expect("Not utf8?"));
}
