#![feature(conservative_impl_trait)]

use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::mem;
use std::time;

type DancerIndex = u32;

const NUM_DANCERS: usize = 16;
const PART2_ITERATIONS: usize = 1_000_000_000;

type DanceLine = [DancerIndex; NUM_DANCERS];

// I'm going to work with the dancers indexes (0-15) instead of letters
#[derive(Clone)]
enum DanceMove {
    Spin(DancerIndex),
    Exchange(DancerIndex, DancerIndex),
    Partner(DancerIndex, DancerIndex),
}

fn get_input() -> String {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut result = String::new();
    f.read_to_string(&mut result).expect("Failed to read file");
    return result;
}

// Parse an ascii string into a DancerIndex
fn parse_ascii(
    input: &[u8],
) -> std::result::Result<DancerIndex, <DancerIndex as std::str::FromStr>::Err> {
    // Using unchecked is ok here because anything that isn't valid utf8 won't parse as a number
    // anyways
    unsafe { str::from_utf8_unchecked(input) }.parse::<DancerIndex>()
}

fn dancer_to_index(dancer: u8) -> DancerIndex {
    (dancer - b'a') as DancerIndex
}

fn index_to_dancer(index: DancerIndex) -> u8 {
    index as u8 + b'a'
}

fn parse_dance_move(input: &str) -> DanceMove {
    use DanceMove::*;
    let bytes = input.as_bytes();
    match bytes[0] {
        b's' => Spin(parse_ascii(&bytes[1..]).expect("Not a number")),
        b'x' => {
            let slash = bytes
                .iter()
                .position(|&x| x == b'/')
                .expect("No slash in Exchange move");
            Exchange(
                parse_ascii(&bytes[1..slash]).expect("Not a number"),
                parse_ascii(&bytes[slash + 1..]).expect("Not a number"),
            )
        }
        b'p' => Partner(dancer_to_index(bytes[1]), dancer_to_index(bytes[3])),
        c => panic!("Unexpected move {}", c as char),
    }
}

// Iterator of dance moves from an input string
fn dance_moves<'a>(input: &'a str) -> impl Iterator<Item = DanceMove> + 'a {
    return input.split(',').map(parse_dance_move);
}

// Get the initial dance line [0, 1, 2, ..., NUM_DANCERS - 1]
fn initial_dance_line() -> DanceLine {
    let mut dance_line: DanceLine = unsafe { mem::uninitialized() };
    for (i, v) in dance_line.iter_mut().enumerate() {
        *v = i as DancerIndex;
    }
    return dance_line;
}

// Reduce a sequence of dance moves into a permutation of the dancer positions (or in other words
// all the swaps combined) and a permutation of the dancer names (all the renames combined).  The
// shifts are incorporated into the position permutation.
fn reduce_dance<T>(dance_moves: T) -> (DanceLine, DanceLine)
where
    T: Iterator<Item = DanceMove>,
{
    use DanceMove::*;
    let mut current_spin = 0 as DancerIndex;
    let mut position_permutation = initial_dance_line();
    let mut inv_renames = initial_dance_line();
    for dance_move in dance_moves {
        match dance_move {
            Spin(count) => {
                current_spin = current_spin.wrapping_add(count);
            }
            Exchange(index1, index2) => {
                let shifted_index1 =
                    index1.wrapping_sub(current_spin) % (NUM_DANCERS as DancerIndex);
                let shifted_index2 =
                    index2.wrapping_sub(current_spin) % (NUM_DANCERS as DancerIndex);
                position_permutation.swap(shifted_index1 as usize, shifted_index2 as usize);
            }
            Partner(index1, index2) => {
                inv_renames.swap(index1 as usize, index2 as usize);
            }
        }
    }
    let mut name_permutation: DanceLine = unsafe { mem::uninitialized() };
    for (new_name, &old_name) in inv_renames.iter().enumerate() {
        name_permutation[old_name as usize] = new_name as DancerIndex;
    }
    let mut position_permutation_shifted: DanceLine = unsafe { mem::uninitialized() };
    for (i, &m) in position_permutation.iter().enumerate() {
        position_permutation_shifted[(i + current_spin as usize) % NUM_DANCERS] = m;
    }

    return (position_permutation_shifted, name_permutation);
}

fn apply_position_permutation(
    dance_line: &DanceLine,
    position_permutation: &DanceLine,
) -> DanceLine {
    let mut result: DanceLine = unsafe { mem::uninitialized() };
    for (i, v) in result.iter_mut().enumerate() {
        *v = dance_line[position_permutation[i] as usize];
    }
    return result;
}

fn apply_name_permutation(mut dance_line: DanceLine, name_permutation: &DanceLine) -> DanceLine {
    for v in dance_line.iter_mut() {
        *v = name_permutation[*v as usize];
    }
    return dance_line;
}


fn part1(position_permutation: &DanceLine, name_permutation: &DanceLine) -> DanceLine {
    return apply_name_permutation(*position_permutation, &name_permutation);
}

// Compute the size of a cycle of the given a permutation.  For 16 elements the maximum cycle
// length is 140, See https://oeis.org/A000793 and https://en.wikipedia.org/wiki/Landau%27s_function
// This works for position permutations and name permutations
fn cycle_size(permutation: &DanceLine) -> usize {
    let initial_line = initial_dance_line();
    let mut current_line = *permutation;
    let mut cycle_size = 1;
    loop {
        if current_line == initial_line {
            return cycle_size;
        }
        current_line = apply_position_permutation(&current_line, permutation);
        cycle_size += 1;
    }
}

fn part2(position_permutation: &DanceLine, name_permutation: &DanceLine) -> DanceLine {
    // Repeatedly applying a permutation to a sequence of elements will cycle back to the initial
    // order at some point.  So when applying that permutation N times, we can get the final result
    // by applying it only N % cycle_length times.  Position permutations and name permutations are
    // independent so we can apply all the position permutations followed by all the name
    // permutations instead of interleaving them.
    let mut dance_line = initial_dance_line();
    let swap_cycle_size = cycle_size(position_permutation);
    for _ in 0..(PART2_ITERATIONS % swap_cycle_size) {
        dance_line = apply_position_permutation(&dance_line, position_permutation);
    }
    let rename_cycle_size = cycle_size(&name_permutation);
    for _ in 0..(PART2_ITERATIONS % rename_cycle_size) {
        dance_line = apply_name_permutation(dance_line, name_permutation);
    }

    return dance_line;
}

// Compute the final dance line by doubling the permutations each iteration and applying to the
// current dance line as needed
fn part2_exp(position_permutation: &DanceLine, name_permutation: &DanceLine) -> DanceLine {
    let mut dance_line = initial_dance_line();
    let mut exp_positon_permutation = *position_permutation;
    let mut exp_name_permutation = *name_permutation;
    let mut pow = 0x1;
    loop {
        if (pow & PART2_ITERATIONS) != 0 {
            dance_line = apply_position_permutation(&dance_line, &exp_positon_permutation);
            dance_line = apply_name_permutation(dance_line, &exp_name_permutation);
        }
        pow = pow << 1;
        if pow > PART2_ITERATIONS {
            break;
        }
        exp_positon_permutation = apply_position_permutation(&exp_positon_permutation, &exp_positon_permutation);
        exp_name_permutation = apply_name_permutation(exp_name_permutation, &exp_name_permutation);
    }
    return dance_line;
}

fn indices_to_ascii(indices: DanceLine) -> [u8; NUM_DANCERS] {
    let mut result: [u8; NUM_DANCERS] = unsafe { mem::uninitialized() };
    for (ascii_char, &index) in result.iter_mut().zip(indices.iter()) {
        *ascii_char = index_to_dancer(index);
    }
    return result;
}

fn time_fn<F, T>(func: F) -> (T, f32)
where
    F: FnOnce() -> T,
{
    let start = time::Instant::now();
    let result = func();
    return (result, duration_to_milliseconds(start.elapsed()));
}

fn duration_to_milliseconds(t: time::Duration) -> f32 {
    t.as_secs() as f32 * 1000.0f32 + t.subsec_nanos() as f32 * 1.0e-6f32
}

fn main() {
    assert!(NUM_DANCERS <= <DancerIndex>::max_value() as usize);
    assert!(
        (NUM_DANCERS & (NUM_DANCERS - 1)) == 0,
        "NUM_DANCERS ({}) must be a power of 2",
        NUM_DANCERS
    );

    let start = time::Instant::now();

    let (input, read_duration) = time_fn(|| get_input());
    println!("Read: {}ms", read_duration);

    let (dance_moves, parse_duration) = time_fn(|| dance_moves(&input).collect::<Vec<_>>());
    println!("Parse: {}ms", parse_duration);

    let ((position_permutation, name_permutation), reduce_duration) =
        time_fn(|| reduce_dance(dance_moves.iter().cloned()));
    println!("Reduce: {}ms", reduce_duration);

    let (part1_result, part1_duration) =
        time_fn(|| part1(&position_permutation, &name_permutation));
    println!(
        "Part 1: {} ({}ms)",
        str::from_utf8(&indices_to_ascii(part1_result)).expect("Not utf8?"),
        part1_duration
    );
    let (part2_result, part2_duration) =
        time_fn(|| part2(&position_permutation, &name_permutation));
    println!(
        "Part 2: {} ({}ms)",
        str::from_utf8(&indices_to_ascii(part2_result)).expect("Not utf8?"),
        part2_duration
    );

    let (part2_exp_result, part2_exp_duration) =
        time_fn(|| part2_exp(&position_permutation, &name_permutation));
    println!(
        "Part 2 (Exponentiation method): {} ({}ms)",
        str::from_utf8(&indices_to_ascii(part2_exp_result)).expect("Not utf8?"),
        part2_exp_duration
    );

    println!("Total: {}ms", duration_to_milliseconds(start.elapsed()));
}
