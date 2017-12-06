use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Loading input string");

    let mut f = File::open("input.txt")
        .expect("Could not open file");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Could not read file");
    let digit_array = buffer.trim();

    let digits: Vec<_> = digit_array.chars()
        .map(|x| x.to_digit(10).expect("Non-digit_character found"))
        .collect();

    let digit1_iter = digits.iter();
    let digit2_iter = digits.iter().cycle().skip(digits.len() / 2);
    let mut sum = 0;
    for (digit1, digit2) in digit1_iter.zip(digit2_iter) {
        if digit1 == digit2 {
            sum += digit1;
        }
    }
    println!("{}", sum);
}
