use std::fs::File;
use std::io::prelude::*;

fn get_input() -> Vec<(u32, u32)> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    let mut result: Vec<(u32, u32)> = Vec::new();

    for line in input_str.lines() {
        let mut numbers_iter = line.split(|c: char| !c.is_digit(10))
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<u32>().expect("Not a number??"));
        let index = numbers_iter.next().expect("No index?");
        let height = numbers_iter.next().expect("No height?");
        result.push((index, height));
    }

    return result;
}

fn scan_position_from_time(range: u32, picoseconds: u32) -> u32 {
    if range == 1 {
        return 0;
    }
    let state_count = range * 2 - 2;
    let wrapped_state = picoseconds % state_count;
    if wrapped_state < range {
        return wrapped_state;
    }
    return state_count - wrapped_state;
}

fn compute_severity(firewall: &[(u32, u32)]) -> u32 {
    let mut position = 0;
    let mut severity = 0;
    for &(depth, range) in firewall.iter() {
        position += depth - position;
        let scan_height = scan_position_from_time(range, position);
        if scan_height == 0 {
            severity += range * depth;
        }
        position += 1;
    }
    return severity;
}

fn passes_firewall(firewall: &[(u32, u32)], delay: u32) -> bool {
    let mut position = 0;
    let mut picoseconds = delay;
    for &(depth, range) in firewall.iter() {
        let step = depth - position;
        picoseconds += step;
        let scan_height = scan_position_from_time(range, picoseconds);
        if scan_height == 0 {
            return false;
        }
        position += step + 1;
        picoseconds += 1;
    }
    return true;
}

fn compute_min_delay(firewall: &[(u32, u32)]) -> u32 {
    let mut delay = 0;
    loop {
        if passes_firewall(firewall, delay) {
            return delay;
        }
        delay += 1;
    }
}

fn main() {
    let firewall = get_input();
    println!("Part 1: {}", compute_severity(&firewall));
    println!("Part 2: {}", compute_min_delay(&firewall));
}
