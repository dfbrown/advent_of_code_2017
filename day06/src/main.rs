use std::fs::File;
use std::io::prelude::*;
use std::collections;
use std::collections::hash_map::Entry;

const MEMORY_BANK_COUNT: usize = 16;
type MemoryBanks = [usize; MEMORY_BANK_COUNT];

fn get_input() -> MemoryBanks {
    let mut f = File::open("input.txt")
        .expect("Could not open file");

    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    let input_iter = input_str.split_whitespace().map(|x| {
        x.parse::<usize>().expect("Non number found in input")
    });

    let mut result: MemoryBanks = Default::default();
    let mut count = 0;
    for (output, input) in (&mut result).into_iter().zip(input_iter) {
        *output = input;
        count += 1;
    }

    if count != MEMORY_BANK_COUNT {
        panic!("Incorrect number of values in input");
    }

    return result;
}

fn part12() -> (usize, usize) {
    let mut current_memory = get_input();
    let mut seen_configurations: collections::HashMap<MemoryBanks, usize> = collections::HashMap::new();

    let mut step: usize = 0;
    loop {
        match seen_configurations.entry(current_memory) {
            Entry::Occupied(entry) => return (step, step - entry.get()),
            Entry::Vacant(entry) => entry.insert(step),
        };

        let (max_index, &value) = current_memory.iter().enumerate().max_by_key(|&(i, v)| (v, -(i as i32))).unwrap();
        current_memory[max_index] = 0;
        let per_bank_value = value / MEMORY_BANK_COUNT;
        let remainder = value % MEMORY_BANK_COUNT;
        for offset in 1..MEMORY_BANK_COUNT + 1 {
            let index = (max_index + offset) % current_memory.len();
            current_memory[index] += per_bank_value;
            if offset <= remainder {
                current_memory[index] += 1;
            }
        }
        step += 1;
    }
}

fn main() {
    let (part1_answer, part2_answer) = part12();
    println!("part1: {}", part1_answer);
    println!("part2: {}", part2_answer);
}
