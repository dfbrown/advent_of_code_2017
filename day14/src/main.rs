use std::fs::File;
use std::io::prelude::*;
use std::collections::vec_deque;

fn get_input_bytes() -> Vec<u8> {
    let mut f = File::open("input.bin").expect("Could not open file");
    let mut result = Vec::new();
    f.read_to_end(&mut result).expect("Failed to read file");
    return result;
}

fn init_start_knot(knot: &mut [u8; 256]) {
    for (i, v) in knot.iter_mut().enumerate() {
        *v = i as u8;
    }
}

fn knot_round<T>(input: T, knot: &mut [u8], position: &mut u8, skip: &mut u8)
where
    T: Iterator<Item = u8>,
{
    assert!(knot.len() == 256, "knot len: {}", knot.len());
    for length in input {
        for i in 0..(length / 2) {
            let front = position.wrapping_add(i);
            // back = position - length - 1 - i
            let back = position
                .wrapping_add(length)
                .wrapping_sub(1)
                .wrapping_sub(i);
            knot.swap(front as usize, back as usize);
        }
        *position = position.wrapping_add(length).wrapping_add(*skip);
        *skip = skip.wrapping_add(1);
    }
}

fn knot_hash(input: &[u8]) -> [u8; 16] {
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

fn count_bits(mut byte: u8) -> u32 {
    let mut set_bits = 0;
    for _ in 0..8 {
        if byte & 0x1 != 0 {
            set_bits += 1;
        }
        byte = byte >> 1;
    }
    return set_bits;
}

const GRID_SIZE: usize = 128;

fn part1(input: &[u8]) -> u32 {
    let mut set_bits = 0;
    for row_index in 0..GRID_SIZE {
        let mut hash_bytes = Vec::from(input);
        let end_str = format!("-{}", row_index);
        hash_bytes.append(&mut end_str.into_bytes());
        let row = knot_hash(&hash_bytes);
        for &byte in row.iter() {
            set_bits += count_bits(byte);
        }
    }
    return set_bits;
}

const GRID_BYTE_SIZE: usize = GRID_SIZE / 8;
type BitGrid = [[u8; GRID_BYTE_SIZE]; GRID_SIZE];

fn is_bit_set(grid: &BitGrid, row: usize, col: usize) -> bool {
    let byte = grid[row][col / 8];
    return ((byte << (col % 8)) & 0x80) != 0;
}

fn unset_bit(grid: &mut BitGrid, row: usize, col: usize) {
    grid[row][col / 8] &= !(0x80 >> (col % 8));
}

fn zero_region(disk: &mut BitGrid, row: usize, col: usize) {
    let mut region_neighbors = vec_deque::VecDeque::with_capacity(10);
    region_neighbors.push_back((row, col));
    while let Some((row, col)) = region_neighbors.pop_front() {
        unset_bit(disk, row, col);
        if row > 0 && is_bit_set(disk, row - 1, col) {
            region_neighbors.push_back((row - 1, col));
        }
        if row < GRID_SIZE - 1 && is_bit_set(disk, row + 1, col) {
            region_neighbors.push_back((row + 1, col));
        }
        if col > 0 && is_bit_set(disk, row, col - 1) {
            region_neighbors.push_back((row, col - 1));
        }
        if col < GRID_SIZE - 1 && is_bit_set(disk, row, col + 1) {
            region_neighbors.push_back((row, col + 1));
        }
    }
}

fn part2(input: &[u8]) -> u32 {
    let mut disk: BitGrid = [[0; GRID_BYTE_SIZE]; GRID_SIZE];
    for (row_index, row) in disk.iter_mut().enumerate() {
        let mut hash_bytes = Vec::from(input);
        let end_str = format!("-{}", row_index);
        hash_bytes.append(&mut end_str.into_bytes());
        *row = knot_hash(&hash_bytes);
    }

    let mut regions = 0;
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if is_bit_set(&disk, row, col) {
                zero_region(&mut disk, row, col);
                regions += 1;
            }
        }
    }
    return regions;
}

fn main() {
    let input = get_input_bytes();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
