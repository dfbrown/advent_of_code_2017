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

    let next_digits = digit_array.chars().cycle().skip(1);
    let mut sum = 0;
    for (digit_char, next_digit_char) in digit_array.chars().zip(next_digits) {
        let digit = digit_char.to_digit(10).expect("Non-digit character found");
        let next_digit = next_digit_char.to_digit(10).expect("Non-digit character found");
        if digit == next_digit {
            sum = sum + digit;
        }
    }

    println!("Sum: {}", sum);
}
