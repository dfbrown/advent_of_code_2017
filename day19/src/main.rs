use std::fs::File;
use std::io::prelude::*;
use std::default::Default;

fn get_position(grid: &[u8], width: i32, position: (i32, i32)) -> Option<u8> {
    if position.0 < 0 || position.0 >= width - 1 {
        return None;
    }
    let index = (position.0 * width + position.1) as usize;
    if index < grid.len() - 1 {
        return Some(grid[index]);
    }
    return None;
}

fn next_direction(
    grid: &[u8],
    width: i32,
    position: (i32, i32),
    direction: (i32, i32),
) -> (i32, i32) {
    if direction.0 == 0 {
        if let Some(c) = get_position(grid, width, (position.0 + 1, position.1)) {
            if c != b' ' {
                return (1, 0)
            }
        }
        return (-1, 0)
    } else {
        if let Some(c) = get_position(grid, width, (position.0, position.1 + 1)) {
            if c != b' ' {
                return (0, 1);
            }
        }
        return (0, -1);
    }
}

fn next_position(
    grid: &[u8],
    width: i32,
    position: (i32, i32),
    direction: &mut (i32, i32),
) -> Option<(i32, i32)> {
    match get_position(grid, width, position) {
        Some(b' ') => return None,
        Some(b'+') => { *direction = next_direction(grid, width, position, *direction); },
        Some(_) =>  {}
        None => panic!("Invalid position {:?}", position)
    }
    return Some((position.0 + direction.0, position.1 + direction.1))
}

fn both_parts(grid: &[u8], width: i32) -> (Vec<u8>, usize) {
    let mut result: Vec<u8> = Default::default();
    let mut current_position = (0 as i32,
                                grid.iter().position(|&x| x == b'|').expect("Couldn't find start position") as i32);
    let mut direction = (1, 0);
    let mut steps = 0;
    while let Some(position) = next_position(grid, width, current_position, &mut direction) {
        let current_byte = get_position(grid, width, position).expect("Invalid position");
        if current_byte >= b'A' && current_byte <= b'Z' {
            result.push(current_byte);
        }
        current_position = position;
        steps += 1;
    }

    return (result, steps);
}

fn main() {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut grid: Vec<u8> = Vec::new();
    f.read_to_end(&mut grid).expect("Could not read file");
    let width = (grid.iter().position(|&x| x == b'\n').expect("No newline?") + 1) as i32;

    if grid.iter().filter(|&&x| x == b'\n').count() * width as usize != grid.len()
    {
        panic!("Input is not a square of text");
    }

    let (text, steps) = both_parts(&grid, width);
    println!("Part 1: {}", std::str::from_utf8(&text).expect("Not utf8???"));
    println!("Part 2: {}", steps);
}
