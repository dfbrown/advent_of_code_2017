use std::fs::File;
use std::io::prelude::*;

fn skip_garbage<T>(char_iter: &mut std::iter::Peekable<T>) -> u32
where
    T: Iterator<Item = char>,
{
    // Skip the opening '<'
    if let Some(c) = char_iter.next() {
        assert!(c == '<', "Garbage must start with '<', instead got {}", c);
    } else {
        panic!("Garbage has end of input")
    }

    let mut garbage_count = 0;
    while let Some(c) = char_iter.next() {
        if c == '>' {
            break;
        } else if c == '!' {
            char_iter.next();
        } else {
            garbage_count += 1;
        }
    }
    assert!(
        char_iter.peek().is_some(),
        "Unexpected end of input in garbage"
    );

    return garbage_count;
}

fn count_groups<T>(char_iter: &mut std::iter::Peekable<T>, depth: u32) -> (u32, u32)
where
    T: Iterator<Item = char>,
{
    // Skip the opening '{'
    assert!(char_iter.peek() == Some(&'{'));
    char_iter.next();

    let mut group_count = 0;
    let mut garbage_count = 0;
    loop {
        // Count subgroup or garbage
        match char_iter.peek() {
            Some(&c) => {
                if c == '{' {
                    let (subgroup_count, subgroup_garbage) = count_groups(char_iter, depth + 1);
                    group_count += subgroup_count;
                    garbage_count += subgroup_garbage;
                } else if c == '<' {
                    garbage_count += skip_garbage(char_iter);
                }
            }
            None => {
                panic!("Unexpected end of input");
            }
        }

        // Check if we are continuing this group with ',', or ending this group with '}'
        match char_iter.next() {
            Some(c) => {
                if c == '}' {
                    // End of group
                    return (depth + group_count, garbage_count);
                } else {
                    assert!(c == ',', "expected \',\' or \'}}\'; got \'{}\'", c);
                }
            }
            None => {
                panic!("Unexpected end of input");
            }
        }
    }
}

fn get_input() -> String {
    let mut f = File::open("input.txt").expect("Could not open file");

    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");
    return input_str;
}

fn main() {
    let input = get_input();
    let (part1, part2) = count_groups(&mut input.chars().peekable(), 1);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
