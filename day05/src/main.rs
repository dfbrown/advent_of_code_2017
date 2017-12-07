use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn get_instructions() -> Vec<i32> {
    let f = File::open("./input.txt").expect("Failed to open file");
    let reader = BufReader::new(&f);
    return reader
        .lines()
        .map(|x| {
            x.expect("Failed to read line")
                .parse::<i32>()
                .expect("Failed to parse number")
        })
        .collect();
}

fn count_instructions(update_instruction: &Fn(i32) -> i32) -> u32 {
    let mut instructions = get_instructions();

    let mut instruction: i32 = 0;
    let mut steps = 0;
    while instruction < instructions.len() as i32 && instruction >= 0 {
        let jump = instructions[instruction as usize];
        instructions[instruction as usize] = update_instruction(jump);
        instruction += jump;
        steps += 1;
    }
    return steps;
}

fn part1_instruction_update(jump: i32) -> i32 {
    jump + 1
}
 fn part2_instruction_update(jump : i32) -> i32 {
     return if jump < 3 { jump + 1 } else { jump - 1 }
 }

fn part1() -> u32 {
    return count_instructions(&part1_instruction_update);
}

fn part2() -> u32 {
    return count_instructions(&part2_instruction_update);
}

fn main() {
    println!("part1: {}", part1());
    println!("part1: {}", part2());
}
