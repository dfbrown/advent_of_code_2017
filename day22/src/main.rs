use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter::FromIterator;

enum State {
    Infected,
    Weakened,
    Flagged,
}

fn get_input() -> HashSet<(isize, isize)> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");
    let mut rows = 0;
    let mut cols = 0;
    let mut infected: Vec<(isize, isize)> = Vec::new();
    for line in input_str.lines() {
        let mut col = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                '#' => { infected.push((rows as isize, col as isize)); },
                _ => panic!("Uh oh"),
            }
            col += 1
        }
        assert!(cols == 0 || col == cols);
        cols = col;
        rows += 1
    }

    let mut grid = HashSet::new();
    let zero_row = rows / 2;
    let zero_col = cols / 2;
    for v in infected {
        grid.insert((v.0 - zero_row, v.1 - zero_col));
    }

    return grid;
}

fn turn_right(direction: (isize, isize)) -> (isize, isize) {
    (direction.1, -direction.0)
}
fn turn_left(direction: (isize, isize)) -> (isize, isize) {
    (-direction.1, direction.0)
}
fn reverse(direction: (isize, isize)) -> (isize, isize) {
    (-direction.0, -direction.1)
}

fn part1(mut grid: HashSet<(isize, isize)>) -> usize {
    let mut position = (0_isize, 0_isize);
    let mut direction = (-1_isize, 0_isize);
    let mut infect_count = 0;
    for _ in 0..10000 {
        let infected = grid.contains(&position);
        if infected {
            direction = turn_right(direction);
            grid.remove(&position);
        } else {
            direction = turn_left(direction);
            grid.insert(position);
            infect_count += 1;
        }
        position.0 += direction.0;
        position.1 += direction.1;
    }
    return infect_count;
}

fn part2(mut grid: HashMap<(isize, isize), State>) -> usize {
    let mut position = (0_isize, 0_isize);
    let mut direction = (-1_isize, 0_isize);
    let mut infect_count = 0;
    for _ in 0..10000000 {
        let entry = grid.entry(position);
        match entry {
            Entry::Occupied(occupied) => {
                match occupied.get() {
                    &State::Infected => {
                        direction = turn_right(direction);
                        *occupied.into_mut() = State::Flagged;
                    },
                    &State::Weakened => {
                        infect_count += 1;
                        *occupied.into_mut() = State::Infected;
                    },
                    &State::Flagged => {
                        direction = reverse(direction);
                        occupied.remove();
                    },
                }
            }
            Entry::Vacant(vacant) => {
                direction = turn_left(direction);
                vacant.insert(State::Weakened);
            }
        }
        position.0 += direction.0;
        position.1 += direction.1;
    }
    return infect_count;
}

fn main() {
    let grid = get_input();
    let part2_grid: HashMap<(isize, isize), State> = HashMap::from_iter(grid.iter().map(|&pos| (pos, State::Infected)));
    let part1_result = part1(grid);
    println!("Part 1: {}", part1_result);
    let part2_result = part2(part2_grid);
    println!("Part 2: {}", part2_result);
}
