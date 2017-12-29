#![feature(test)]

extern crate rand;
extern crate bit_vec;
extern crate test;

use std::fs::File;
use std::io::prelude::*;
use bit_vec::BitVec;

#[cfg(test)]
use rand::{Rng, StdRng, SeedableRng};
#[cfg(test)]
use test::Bencher;

const MAX_SOURCE_SIZE: usize = 9;
const MAX_DEST_SIZE: usize = 16;

const PATTERN2_SIZE: usize = 2;

const PATTERN3_SIZE: usize = 3;
type Pattern = u16;

fn rotate_90(pattern: Pattern, size: usize) -> Pattern {
    let mut result: Pattern = 0;
    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            let rot_index = (size - 1 - col) * size + row;
            result |= ((pattern >> index) & 0x1) << rot_index;
        }
    }
    return result;
}

#[test]
fn test_rotate_90() {
    let seed = [0xdeadbeef_usize];
    let mut rng: StdRng = SeedableRng::from_seed(&seed[..]);
    for size in 2..4 {
        for _ in 0..100 {
            let pattern: Pattern = rng.gen_range(0, (0x1 << (size * size)) - 1);
            assert_eq!(pattern,
                       rotate_90(rotate_90(rotate_90(rotate_90(pattern, size), size), size), size));
            assert_eq!(rotate_90(rotate_90(pattern, size), size), flip_horizontal(flip_vertical(pattern, size), size));

        }
    }
}

#[test]
fn test_flip_horizontal() {
    let seed = [0xdeadbeef_usize];
    let mut rng: StdRng = SeedableRng::from_seed(&seed[..]);

    for size in 2..4 {
        for _ in 0..100 {
            let pattern: Pattern = rng.gen_range(0, (0x1 << (size * size)) - 1);
            assert_eq!(pattern,
                       flip_horizontal(flip_horizontal(pattern, size), size));
        }
    }
}

#[test]
fn test_flip_vertical() {
    let seed = [0xdeadbeef_usize];
    let mut rng: StdRng = SeedableRng::from_seed(&seed[..]);
    for size in 2..4 {
        for _ in 0..100 {
            let pattern: Pattern = rng.gen_range(0, (0x1 << (size * size)) - 1);
            assert_eq!(pattern,
                    flip_vertical(flip_vertical(pattern, size), size));
        }
    }
}

fn flip_vertical(pattern: Pattern, size: usize) -> Pattern {
    let mut result: Pattern = 0;
    let row_mask = (0x1 << size) - 1;
    let mut source_shift: usize = 0;
    let mut dest_shift = size * (size - 1);
    for _ in 0..size {
        let row = (pattern >> source_shift) & row_mask;
        result |= row << dest_shift;
        source_shift = source_shift.wrapping_add(size);
        dest_shift = dest_shift.wrapping_sub(size);
    }
    return result;
}

fn flip_horizontal(pattern: Pattern, size: usize) -> Pattern {
    let mut result: Pattern = 0;
    let mut col_mask = 0;
    for col in 0..size {
        col_mask |= 0x1 << (size * col);
    }
    let mut source_shift: u8 = 0;
    let mut dest_shift = size - 1;
    for _ in 0..size {
        let col = (pattern >> source_shift) & col_mask;
        result |= col << dest_shift;
        source_shift = source_shift.wrapping_add(1);
        dest_shift = dest_shift.wrapping_sub(1);
    }
    return result;
}

fn pattern_permutations(pattern: Pattern, size: usize) -> [Pattern; 12] {
    let mut result: [Pattern; 12] = [0; 12];
    result[0] = pattern;
    result[1] = rotate_90(result[0], size);
    result[2] = rotate_90(result[1], size);
    result[3] = rotate_90(result[2], size);

    result[4] = flip_horizontal(result[0], size);
    result[5] = flip_horizontal(result[1], size);
    result[6] = flip_horizontal(result[2], size);
    result[7] = flip_horizontal(result[3], size);

    result[8]  = flip_vertical(result[0], size);
    result[9]  = flip_vertical(result[1], size);
    result[10] = flip_vertical(result[2], size);
    result[11] = flip_vertical(result[3], size);
    return result;
}

fn bits_to_pattern(bits: &[u8]) -> Pattern {
    let mut result: Pattern = 0;
    for (i, &bit) in bits.iter().enumerate() {
        result |= (bit as Pattern) << i;
    }
    return result;
}

fn get_input() -> (Vec<Pattern>, Vec<Pattern>) {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    let mut source: Vec<u8> = Vec::with_capacity(MAX_SOURCE_SIZE);
    let mut dest: Vec<u8> = Vec::with_capacity(MAX_DEST_SIZE);

    let mut pattern2_map: Vec<Pattern> = vec![0; 0x1 << (PATTERN2_SIZE * PATTERN2_SIZE)];
    let mut pattern3_map: Vec<Pattern> = vec![0; 0x1 << (PATTERN3_SIZE * PATTERN3_SIZE)];

    for line in input_str.lines() {
        source.clear();
        dest.clear();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '.' => source.push(0),
                '#' => source.push(1),
                '=' => break,
                '/' | ' ' => {},
                other => panic!("Unexpected character parsing pattern: {}", other),
            }
        }
        chars.next();
        while let Some(c) = chars.next() {
            match c {
                '.' => dest.push(0),
                '#' => dest.push(1),
                '/' | ' ' => {},
                other => panic!("Unexpected character parsing pattern: {}", other),
            }
        }

        let source_pattern = bits_to_pattern(&source);
        let dest_pattern = bits_to_pattern(&dest);
        let (ref mut map, pattern_size) = if source.len() == PATTERN2_SIZE * PATTERN2_SIZE {
            (&mut pattern2_map, PATTERN2_SIZE)
        } else if source.len() == PATTERN3_SIZE * PATTERN3_SIZE {
            (&mut pattern3_map, PATTERN3_SIZE)
        } else {
            panic!("Uh oh")
        };
        assert_eq!(dest.len(), (pattern_size + 1) * (pattern_size + 1));
        for &pattern in pattern_permutations(source_pattern, pattern_size).iter() {
            assert!(map[pattern as usize] == 0 || map[pattern as usize] == dest_pattern);
            map[pattern as usize] = dest_pattern;
        }
    }
    return (pattern2_map, pattern3_map);
}

fn get_pattern(drawing: &BitVec, drawing_size: usize, pattern_size: usize, start_index: usize) -> Pattern {
    let mut pattern: Pattern = 0;
    let mut i = 0;
    let mut i_pattern = start_index;
    for _ in 0..pattern_size {
        for _ in 0..pattern_size {
            let bit = drawing[i_pattern] as Pattern;
            pattern |= (0x1 & bit) << i;
            i += 1;
            i_pattern += 1;
        }
        i_pattern += drawing_size - pattern_size;
    }
    return pattern;
}

fn set_pattern(drawing: &mut BitVec, drawing_size: usize, pattern: Pattern, pattern_size: usize, start_index: usize) {

    let mut i = 0;
    let mut i_pattern = start_index;
    for _ in 0..pattern_size {
        for _ in 0..pattern_size {
            drawing.set(i_pattern, (pattern & (0x1 << i)) != 0);
            i += 1;
            i_pattern += 1;
        }
        i_pattern += drawing_size - pattern_size;
    }
}

fn count_ones(drawing: &BitVec) -> u32 {
    drawing.blocks().fold(0, |acc, block| acc + block.count_ones())
}

fn compute_result(pattern2_map: &Vec<Pattern>, pattern3_map: &Vec<Pattern>) -> (u32, u32) {
    let mut current_drawing_size = PATTERN3_SIZE;
    let initial_pattern = [0b01000111_u8, 0b10000000_u8];
    let mut drawing = BitVec::from_bytes(&initial_pattern);
    drawing.truncate(current_drawing_size * current_drawing_size);

    let mut drawing_buffer = BitVec::new();

    let mut part1_result = 0;
    for iteration in 0..18 {
        let (ref pattern_map, pattern_size) = if current_drawing_size % PATTERN2_SIZE == 0 {
            (&pattern2_map, PATTERN2_SIZE)
        } else if current_drawing_size % PATTERN3_SIZE == 0 {
            (&pattern3_map, PATTERN3_SIZE)
        } else {
            panic!("Uh oh")
        };

        let pattern_dim = current_drawing_size / pattern_size;
        let dest_pattern_size = pattern_size + 1;
        let new_drawing_size = pattern_dim * dest_pattern_size;


        let grow = new_drawing_size * new_drawing_size - drawing_buffer.len();
        drawing_buffer.grow(grow, false);

        let source_pattern_col_skip = pattern_size;
        let dest_pattern_col_skip = pattern_size + 1;
        let mut i_source = 0;
        let mut i_dest = 0;
        for _ in 0..pattern_dim {
            for _ in 0..pattern_dim {
                let source_pattern = get_pattern(&drawing, current_drawing_size, pattern_size, i_source);
                let dest_pattern = pattern_map[source_pattern as usize];
                set_pattern(&mut drawing_buffer, new_drawing_size, dest_pattern, dest_pattern_size, i_dest);
                i_source += source_pattern_col_skip;
                i_dest += dest_pattern_col_skip;
            }
            i_source += current_drawing_size * (pattern_size - 1);
            i_dest += new_drawing_size * (dest_pattern_size - 1);
        }
        std::mem::swap(&mut drawing_buffer, &mut drawing);
        current_drawing_size = new_drawing_size;

        if iteration == 4 {
            part1_result = count_ones(&drawing);
        }
    }

    let part2_result = drawing.blocks().fold(0, |acc, block| acc + block.count_ones());
    return (part1_result, part2_result);
}

#[bench]
fn result_bench(b: &mut Bencher) {
    let (pattern2_map, pattern3_map) = get_input();
    b.iter(|| {
        test::black_box(compute_result(&pattern2_map, &pattern3_map));
    });
}

fn main() {
    let (pattern2_map, pattern3_map) = get_input();
    let (part1_result, part2_result) = compute_result(&pattern2_map, &pattern3_map);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}
