use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn get_input() -> String {
    let mut f = File::open("input.txt")
        .expect("Could not open file");

    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");
    return input_str;
}

fn compute_register_values<'a>(input: &'a str) -> (HashMap<&'a str, i32>, i32) {
    let mut registers = HashMap::new();

    let mut max_value = 0;
    for line in input.lines() {
        let mut word_iter = line.split_whitespace();
        let register = word_iter.next().expect("Could not find register");
        let inc_type = word_iter.next().expect("Could not get inc/dec");
        let inc_amount_str = String::from(word_iter.next().expect("Could not get inc/dec value"));
        let increment = match inc_type {
            "inc" => inc_amount_str.parse::<i32>().expect("Failed to parse inc/dec amount"),
            "dec" => -inc_amount_str.parse::<i32>().expect("Failed to parse inc/dec amount"),
            _ => panic!("Unknown increment type {}", inc_type)
        };
        assert!(word_iter.next().expect("Missing if") == "if");
        let cmp_register = word_iter.next().expect("Missing condition register");
        let cmp_op = word_iter.next().expect("Missing comparison op");
        let cmp_rhs = word_iter.next()
            .expect("Missing comparison rhs")
            .parse::<i32>()
            .expect("Could not parse comparison rhs");

        let cmp_register_value = *registers.get(cmp_register).unwrap_or(&0);

        let cmp_success = match cmp_op {
            "==" => cmp_register_value == cmp_rhs,
            "!=" => cmp_register_value != cmp_rhs,
            "<=" => cmp_register_value <= cmp_rhs,
            "<"  => cmp_register_value <  cmp_rhs,
            ">=" => cmp_register_value >= cmp_rhs,
            ">"  => cmp_register_value >  cmp_rhs,
            _    => panic!("Unkown comparison {}", cmp_op)
        };

        if cmp_success {
            let value = match registers.entry(register) {
                Entry::Vacant(v) => {
                    *v.insert(increment)
                }
                Entry::Occupied(mut v) => {
                    let new_value = *v.get() + increment;
                    v.insert(new_value);
                    new_value
                }
            };
            max_value = max_value.max(value);
        }
    }

    return (registers, max_value);
}

fn main() {
    let input = get_input();
    let (registers, run_max) = compute_register_values(input.as_str());
    let end_max = *registers.iter().max_by_key(|x| x.1).expect("No registers?").1;
    println!("part1: {}", end_max);
    println!("part2: {}", run_max);
}
