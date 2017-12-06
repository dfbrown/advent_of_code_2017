use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut used_word2_letter = vec![false; word2.chars().count()];
    for word1_char in word1.chars() {
        let mut found_letter = false;
        for (i, word2_char) in word2.chars().enumerate() {
            if !used_word2_letter[i] && word1_char == word2_char {
                used_word2_letter[i] = true;
                found_letter = true;
                break;
            }
        }
        if !found_letter {
            return false;
        }
    }
    return true;
}

fn valid_password_count<F>(predicate: F) -> u32
    where
        F: Fn(&str, &str) -> bool
{
    let f = File::open("./input.txt").expect("Could not open file");
    let reader = BufReader::new(&f);
    let mut num_valid = 0;
    for line in reader.lines() {
        let line = line.expect("Hmm");
        let words: Vec<_> = line.split(" ").collect();
        let mut valid = true;
        for (i, word1) in words.iter().enumerate() {
            valid &= words.iter().skip(i + 1).all(|word2| predicate(word1, word2));
        }

        if valid {
            num_valid += 1;
        }
    }
    return num_valid;
}

fn part1() -> u32 {
    return valid_password_count(|w1, w2| w1 != w2);
}

fn part2() -> u32 {
    return valid_password_count(|w1, w2| !is_anagram(w1, w2));
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
