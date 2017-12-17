use std::time;

const OFFSET: usize = 363;

fn insert_after(buffer: &mut [u32], index: usize, current_size: usize) {
    let replace_index = index + 1;
    let mut replaced_value = buffer[replace_index];
    buffer[replace_index] = current_size as u32;
    for i in (replace_index + 1)..(current_size + 1) {
        let v = buffer[i];
        buffer[i] = replaced_value;
        replaced_value = v;
    }
}

fn part1() -> u32 {
    // Just do the naive thing and build the full buffer
    const STEPS: usize = 2017;
    const BUFFER_SIZE: usize = STEPS + 1;

    let mut buffer = [u32::max_value(); BUFFER_SIZE];
    let mut index = 0;
    buffer[0] = 0;
    for current_size in 1..(STEPS + 1) {
        index = (index + OFFSET) % current_size;
        insert_after(&mut buffer, index, current_size);
        index += 1;
    }

    assert!(buffer[index] == STEPS as u32);
    return buffer[(index + 1) % BUFFER_SIZE];
}


fn part2() -> u32 {
    // Find the element after the value 0.  0 is always the first element so we can just keep track
    // of the last time we inserted after index 0.
    const STEPS: usize = 5_000_000;
    let mut index = 0;
    let mut value_after_zero = 0u32;
    for current_size in 1..(STEPS+1) {
        index = 1 + (index + OFFSET) % current_size;
        if index == 1 {
            value_after_zero = current_size as u32;
        }
    }

    return value_after_zero;
}

fn time_fn<F, T>(func: F) -> (T, f32)
where
    F: FnOnce() -> T,
{
    let start = time::Instant::now();
    let result = func();
    return (result, duration_to_seconds(start.elapsed()));
}

fn duration_to_seconds(t: time::Duration) -> f32 {
    t.as_secs() as f32 + t.subsec_nanos() as f32 * 1.0e-9f32
}

fn main() {
    let (result, time) = time_fn(|| part1());
    println!("Part 1: {} ({}s)", result, time);
    let (result, time) = time_fn(|| part2());
    println!("Part 2: {} ({}s)", result, time);
}
