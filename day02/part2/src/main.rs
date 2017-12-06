use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("./input.txt").expect("Could not open file");
    let reader = BufReader::new(&f);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Uh...");
        let numbers: Vec<_> = line.split("\t").map(|v| v.parse::<u32>().expect("Non-number found")).collect();
        'outer: for (i1, v1) in numbers.iter().enumerate() {
            for v2 in numbers.iter().skip(i1 + 1) {
                if v1 % v2 == 0 {
                    sum += v1 / v2;
                    break 'outer;
                }
                if v2 % v1 == 0 {
                    sum += v2 / v1;
                    break 'outer;
                }
            }
        }
    }
    println!("{}", sum)
}
