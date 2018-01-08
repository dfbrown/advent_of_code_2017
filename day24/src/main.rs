use std::fs::File;
use std::io::prelude::*;
use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing input")
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Error parsing input"
    }
}

fn get_input() -> Result<Vec<[u16; 2]>, ParseError> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str).map_err(|_| ParseError)?;

    let mut result = Vec::new();
    for line in input_str.lines() {
        let mut split = line.split('/');
        result.push([
            split
                .next()
                .ok_or(ParseError)?
                .parse::<u16>()
                .map_err(|_| ParseError)?,
            split
                .next()
                .ok_or(ParseError)?
                .parse::<u16>()
                .map_err(|_| ParseError)?,
        ]);
    }

    return Ok(result);
}

#[derive(Debug, Clone)]
struct Bridge {
    all_pieces: Vec<[u16; 2]>,
    end_index: usize,
    end_port: u16,
}

impl Bridge {
    fn from_pieces(pieces: &[[u16; 2]]) -> Bridge {
        Bridge {
            all_pieces: pieces.to_vec(),
            end_index: 0,
            end_port: 0,
        }
    }
    fn unused_pieces(&self) -> &[[u16; 2]] {
        &self.all_pieces[self.end_index..]
    }
    fn strength(&self) -> usize {
        self.all_pieces[0..self.end_index]
            .iter()
            .fold(0, |acc, &x| acc + (x[0] + x[1]) as usize)
    }
    fn add_piece(&self, remaining_pieces_index: usize) -> Option<Bridge> {
        let add_index = self.end_index + remaining_pieces_index;
        let add_piece = self.all_pieces[add_index];
        for (&port, &other_port) in add_piece.iter().zip(add_piece.iter().cycle().skip(1)) {
            if port == self.end_port {
                let mut new_bridge = self.clone();
                new_bridge.all_pieces.swap(add_index, self.end_index);
                new_bridge.end_index += 1;
                new_bridge.end_port = other_port;
                return Some(new_bridge);
            }
        }
        return None;
    }
}

fn strongest_bridge(bridge: Bridge) -> usize {
    let mut max_strength = bridge.strength();
    for i in 0..bridge.unused_pieces().len() {
        if let Some(new_bridge) = bridge.add_piece(i) {
            max_strength = std::cmp::max(max_strength, strongest_bridge(new_bridge));
        }
    }
    max_strength
}

fn longest_strongest_bridge(bridge: Bridge) -> (usize, usize) {
    let mut longest = (bridge.end_index, bridge.strength());
    for i in 0..bridge.unused_pieces().len() {
        if let Some(new_bridge) = bridge.add_piece(i) {
            longest = std::cmp::max(longest, longest_strongest_bridge(new_bridge));
        }
    }
    longest
}

fn part1(pieces: &[[u16; 2]]) -> usize {
    strongest_bridge(Bridge::from_pieces(pieces))
}

fn part2(pieces: &[[u16; 2]]) -> usize {
    longest_strongest_bridge(Bridge::from_pieces(pieces)).1
}

fn main() {
    let input = match get_input() {
        Ok(v) => v,
        Err(e) => panic!(e.to_string()),
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
