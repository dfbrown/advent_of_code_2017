#![feature(entry_and_modify)]

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

fn get_root<'a>(discs: &'a HashMap<&'a str, Disc<'a>>) -> Option<(&'a str, &Disc<'a>)> {
    for (name, disc) in discs.iter() {
        if disc.parent.is_none() {
            return Some((name, disc));
        }
    }
    return None;
}

enum BalancedStatus {
    Unbalanced(u32),
    Balanced(u32)
}

fn is_tower_balanced<'a>(base_disc: &'a Disc<'a>, discs: &HashMap<&'a str, Disc<'a>>) -> BalancedStatus {
    let mut weights: HashMap<u32, (u32, u32)> = HashMap::new();

    let mut total_weight = base_disc.weight;;
    for child_name in base_disc.children.iter() {
        let child = discs.get(child_name).expect("Child does not exist???");
        match is_tower_balanced(child, discs) {
            BalancedStatus::Unbalanced(needed_weight) => {
                return BalancedStatus::Unbalanced(needed_weight)
            }
            BalancedStatus::Balanced(subtower_weight) => {
                total_weight += subtower_weight;
                weights.entry(subtower_weight)
                    .and_modify(|&mut (ref mut count, _)| *count += 1)
                    .or_insert((1, child.weight));
            }
        }
    }

    if weights.len() > 1 {
        assert!(weights.len() == 2);
        let mut good_weight = 0;
        let mut bad_weight = 0;
        let mut bad_child_weight = 0;
        for (&total_weight, &(count, child_weight)) in weights.iter() {
            if count == 1 {
                bad_weight = total_weight;
                bad_child_weight = child_weight
            } else {
                good_weight = total_weight;
            }
        }

        return BalancedStatus::Unbalanced(good_weight - (bad_weight - bad_child_weight));
    }

    return BalancedStatus::Balanced(total_weight);
}

fn part1<'a>(discs: &'a HashMap<&'a str, Disc<'a>>) -> &'a str {
    return match get_root(discs) {
        Some((name, _)) => name,
        None => ""
    }
}

fn part2<'a>(discs: &HashMap<&'a str, Disc<'a>>) -> u32 {
    let (_, root_disc) = get_root(discs).expect("No root?");
    return match is_tower_balanced(root_disc, discs) {
        BalancedStatus::Unbalanced(expected_weight) => expected_weight,
        BalancedStatus::Balanced(_) => 0
    }
}

fn main() {
    let input = get_input();
    let discs = parse_input(input.as_str());
    println!("Part 1: {}", part1(&discs));
    println!("Part 2: {}", part2(&discs));
}
