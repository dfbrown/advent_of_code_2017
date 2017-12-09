use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug)]
struct Disc<'a> {
    weight: u32,
    children: Vec<&'a str>,
    parent: Option<&'a str>
}

fn get_input() -> String {
    let mut f = File::open("input.txt")
        .expect("Could not open file");

    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");
    return input_str
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Disc<'a>> {
    let mut discs: HashMap<&'a str, Disc<'a>> = HashMap::new();

    for line in input.lines() {
        let mut word_iter = line.split_whitespace();
        let name = word_iter.next().expect("Expected word");
        let weight_str = word_iter.next().expect("Expected weight");
        let weight = weight_str[1..weight_str.len()-1].parse::<u32>().expect("Could not parse size");

        // Skip the "->" before children
        word_iter.next();

        // Children are comma separated, so strip that off
        let children = Vec::from_iter(word_iter.map(|x| x.trim_right_matches(',')));

        for child in children.iter() {
            match discs.entry(child) {
                Entry::Occupied(mut v) => {
                    v.get_mut().parent = Some(name);
                }
                Entry::Vacant(v) => {
                    v.insert(Disc {
                        weight: Default::default(),
                        children: Default::default(),
                        parent: Some(name)
                    });
                }
            }
        }
        match discs.entry(name) {
            Entry::Occupied(mut v) => {
                v.get_mut().weight = weight;
                v.get_mut().children = children;
                assert!(!v.get().parent.is_none());
            }
            Entry::Vacant(v) => {
                v.insert(Disc {
                    weight: weight,
                    children: children,
                    parent: None
                });
            }
        }
    }

    return discs;
}

fn part1<'a>(discs: &HashMap<&'a str, Disc<'a>>) -> &'a str {
    for (name, disc) in discs.iter() {
        if disc.parent.is_none() {
            return name;
        }
    }
    return "";
}

fn main() {
    let input = get_input();
    let discs = parse_input(input.as_str());
    println!("part1 {}", part1(&discs));
}
