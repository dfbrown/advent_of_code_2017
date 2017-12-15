const A_START: u64 = 722;
const B_START: u64 = 354;
const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;
const A_DIVISOR: u64 = 4;
const B_DIVISOR: u64 = 8;
const MODULO: u64 = 2147483647;

fn next_value(value: u64, factor: u64) -> u64 {
    return (value * factor) % MODULO;
}

fn next_value_part2(mut value: u64, factor: u64, divisor: u64) -> u64 {
    loop {
        value = next_value(value, factor);
        if value % divisor == 0 {
            return value;
        }
    }
}

fn part1() -> u64 {
    let mut a = A_START;
    let mut b = B_START;
    let mut same_count = 0;
    for _ in 0..40_000_000 {
        a = next_value(a, A_FACTOR);
        b = next_value(b, B_FACTOR);
        if (a & 0xffff) == (b & 0xffff) {
            same_count += 1;
        }
    }
    return same_count;
}

fn part2() -> u64 {
    let mut a = A_START;
    let mut b = B_START;
    let mut same_count = 0;
    for _ in 0..5_000_000 {
        a = next_value_part2(a, A_FACTOR, A_DIVISOR);
        b = next_value_part2(b, B_FACTOR, B_DIVISOR);
        if (a & 0xffff) == (b & 0xffff) {
            same_count += 1;
        }
    }
    return same_count;
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
