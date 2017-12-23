#![feature(test)]

extern crate itertools;
extern crate test;

use std::fs::File;
use std::io::prelude::*;
use std::ops;
use itertools::Itertools;
use std::collections::HashSet;

#[cfg(test)]
use test::Bencher;

type IntType = i32;


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct IntVec3 {
    values: [IntType; 3],
}

impl IntVec3 {
    fn from_number_iter<I: Iterator<Item = IntType>>(iter: &mut I) -> IntVec3 {
        IntVec3::new(
            iter.next().expect("No number?"),
            iter.next().expect("No number?"),
            iter.next().expect("No number?"),
        )
    }
    fn new(x: IntType, y: IntType, z: IntType) -> IntVec3 {
        IntVec3 { values: [x, y, z] }
    }
    fn l1_norm(&self) -> IntType {
        self.values.iter().fold(0, |a, &x| a + x.abs())
    }
    fn iter(&self) -> std::iter::Cloned<std::slice::Iter<IntType>> {
        self.values.iter().cloned()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<IntType> {
        self.values.iter_mut()
    }
}

impl ops::Add<IntVec3> for IntVec3 {
    type Output = IntVec3;
    fn add(self, rhs: IntVec3) -> IntVec3 {
        let mut result: IntVec3 = Default::default();
        result
            .iter_mut()
            .set_from(self.iter().zip(rhs.iter()).map(|(l, r)| l + r));
        return result;
    }
}

impl ops::Sub<IntVec3> for IntVec3 {
    type Output = IntVec3;
    fn sub(self, rhs: IntVec3) -> IntVec3 {
        let mut result: IntVec3 = Default::default();
        result
            .iter_mut()
            .set_from(self.iter().zip(rhs.iter()).map(|(l, r)| l - r));
        return result;
    }
}

impl ops::Mul<IntType> for IntVec3 {
    type Output = IntVec3;
    fn mul(self, rhs: IntType) -> IntVec3 {
        let mut result: IntVec3 = Default::default();
        result.iter_mut().set_from(self.iter().map(|l| l * rhs));
        return result;
    }
}

impl ops::Mul<IntVec3> for IntVec3 {
    type Output = IntVec3;
    fn mul(self, rhs: IntVec3) -> IntVec3 {
        let mut result: IntVec3 = Default::default();
        result
            .iter_mut()
            .set_from(self.iter().zip(rhs.iter()).map(|(l, r)| l * r));
        return result;
    }
}

impl ops::Neg for IntVec3 {
    type Output = IntVec3;
    fn neg(self) -> IntVec3 {
        let mut result: IntVec3 = Default::default();
        result.iter_mut().set_from(self.iter().map(ops::Neg::neg));
        return result;
    }
}

impl ops::Div<IntType> for IntVec3 {
    type Output = IntVec3;
    fn div(self, rhs: IntType) -> IntVec3 {
        let mut result: IntVec3 = Default::default();
        result.iter_mut().set_from(self.iter().map(|l| l / rhs));
        return result;
    }
}

impl ops::Rem<IntVec3> for IntVec3 {
    type Output = IntVec3;
    fn rem(self, rhs: IntVec3) -> IntVec3 {
        let mut result: IntVec3 = Default::default();
        result
            .iter_mut()
            .set_from(self.iter().zip(rhs.iter()).map(|(l, r)| l % r));
        return result;
    }
}

fn parse_particle(line: &str) -> (IntVec3, IntVec3, IntVec3) {
    let mut numbers_iter = line.split(|x| !char::is_digit(x, 10) && x != '-')
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<IntType>().unwrap());
    (
        IntVec3::from_number_iter(&mut numbers_iter),
        IntVec3::from_number_iter(&mut numbers_iter),
        IntVec3::from_number_iter(&mut numbers_iter),
    )
}

fn get_input() -> (Vec<IntVec3>, Vec<IntVec3>, Vec<IntVec3>) {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    let mut positions: Vec<IntVec3> = Vec::new();
    let mut velocities: Vec<IntVec3> = Vec::new();
    let mut accelerations: Vec<IntVec3> = Vec::new();
    for line in input_str.lines() {
        let (p, v, a) = parse_particle(line);
        positions.push(p);
        velocities.push(v);
        accelerations.push(a);
    }
    return (positions, velocities, accelerations);
}

fn part1(positions: &[IntVec3], velocities: &[IntVec3], accelerations: &[IntVec3]) -> usize {
    itertools::multizip((positions, velocities, accelerations))
        .enumerate()
        .min_by_key(|&(_, (p, v, a))| (a.l1_norm(), v.l1_norm(), p.l1_norm()))
        .expect("Expected a min")
        .0
}

fn integer_sqrt(v: IntType) -> Option<i32> {
    let sqrt = (v as f64).sqrt();
    if (sqrt * sqrt) as IntType == v {
        return Some(sqrt as IntType);
    }
    return None;
}

#[derive(Debug, Clone, Copy)]
enum Intersections {
    Infinite,
    One(IntType),
    Two(IntType, IntType),
    Zero,
}

// Only return positive, integer results
fn solve_quadratic_positive(a: IntType, b: IntType, c: IntType) -> Intersections {
    use Intersections::*;
    if a == 0 {
        if b == 0 {
            if c == 0 {
                return Infinite;
            } else {
                return Zero;
            }
        } else {
            if c % b == 0 {
                let intersection = -c / b;
                if intersection >= 0 {
                    return One(-c / b);
                }
            }
            return Zero;
        }
    } else {
        let sqrt_term = match integer_sqrt(b * b - 4 * a * c) {
            None => { return Zero; }
            Some(v) => v
        };
        let numerator1 = -b + sqrt_term;
        let numerator2 = -b - sqrt_term;
        let denominator = 2 * a;
        let mut intersection1 = None;
        let mut intersection2 = None;
        if numerator1 % denominator == 0 {
            let intersection = numerator1 / denominator;
            if intersection >= 0 {
                intersection1 = Some(intersection);
            }
        }
        if numerator2 % denominator == 0 {
            let intersection = numerator2 / denominator;
            if intersection >= 0 {
                intersection2 = Some(intersection);
            }
        }
        return match (intersection1, intersection2) {
            (Some(i1), Some(i2)) =>
                if i1 == i2 {
                    One(i1)
                } else {
                    Two(std::cmp::min(i1, i2), std::cmp::max(i1, i2))
                },
            (Some(i1), None) => One(i1),
            (None, Some(i2)) => One(i2),
            (None, None) => Zero,
        }
    }
}

fn intersection_union(
    intersections1: Intersections,
    intersections2: Intersections,
) -> Intersections {
    use Intersections::*;
    match (intersections1, intersections2) {
        (Zero, _) => Zero,
        (_, Zero) => Zero,
        (Infinite, other) => other,
        (other, Infinite) => other,
        (One(i1), One(i2)) => if i1 == i2 { One(i1) } else { Zero },
        (One(i1), Two(i2_1, i2_2)) => if i1 == i2_1 || i1 == i2_2 { One(i1) } else { Zero },
        (Two(i2_1, i2_2), One(i1)) => if i1 == i2_1 || i1 == i2_2 { One(i1) } else { Zero },
        (Two(i1_1, i1_2), Two(i2_1, i2_2)) => {
            let eq11 = i1_1 == i2_1;
            let eq12 = i1_1 == i2_2;
            let eq21 = i1_2 == i2_1;
            let eq22 = i1_2 == i2_2;
            if eq11 && eq22 {
                assert!(i1_1 != i1_2);
                Two(i1_1, i1_2)
            } else if eq11 || eq12 {
                assert!(eq21 == false);
                assert!(eq22 == false);
                One(i1_1)
            } else if eq21 || eq22 {
                assert!(eq11 == false);
                assert!(eq12 == false);
                One(i1_2)
            } else {
                Zero
            }
        }
    }
}

fn first_intersection_time(
    p0: IntVec3,
    v0: IntVec3,
    a0: IntVec3,
    p1: IntVec3,
    v1: IntVec3,
    a1: IntVec3,
) -> Option<IntType> {
    let two_a = a0 - a1;
    let two_b = (v0 - v1) * 2 + a0 - a1;
    let two_c = (p0 - p1) * 2;

    let component_intersections = (
        solve_quadratic_positive(two_a.values[0], two_b.values[0], two_c.values[0]),
        solve_quadratic_positive(two_a.values[1], two_b.values[1], two_c.values[1]),
        solve_quadratic_positive(two_a.values[2], two_b.values[2], two_c.values[2]),
        );

    let intersections = intersection_union(
        intersection_union(component_intersections.0, component_intersections.1),
        component_intersections.2,
    );

    return match intersections {
        Intersections::Two(v0, v1) => {
            assert!(v1 > v0, "v0: {}, v1: {}", v0, v1);
            Some(v0)
        },
        Intersections::One(v) => Some(v),
        Intersections::Infinite => Some(0),
        Intersections::Zero => None,
    }
}

fn lower_triangle_matrix_size(dim: usize) -> usize {
    dim * (dim - 1) / 2
}

fn lower_triangle_matrix_index(row: usize, col: usize) -> usize {
    assert!(row > col);
    row * (row - 1) / 2 + col
}

fn get_all_intersections(positions: &[IntVec3], velocities: &[IntVec3], accelerations: &[IntVec3]) -> Vec<Option<IntType>> {
    let num_particles = positions.len();
    let mut intersect_time: Vec<Option<IntType>> = vec![None; lower_triangle_matrix_size(num_particles)];
    for row in 1..num_particles {
        for col in 0..row {
            let i = lower_triangle_matrix_index(row, col);
            intersect_time[i] = first_intersection_time(
                positions[row],
                velocities[row],
                accelerations[row],
                positions[col],
                velocities[col],
                accelerations[col]);
        }
    }
    return intersect_time;
}

fn filter_colliding_particles(intersect_time: &mut [Option<IntType>]) -> usize {
    let num_particles = (1 + ((1 + 8 * intersect_time.len()) as f64).sqrt() as usize) / 2;
    assert!((num_particles * (num_particles - 1) / 2) == intersect_time.len());

    let mut remaining_particles = num_particles;
    let mut min_intersection_particles: HashSet<usize> = HashSet::new();
    loop {
        min_intersection_particles.clear();
        let mut min_intersection_time: IntType = IntType::max_value();
        for row in 1..num_particles {
            for col in 0..row {
                let i = lower_triangle_matrix_index(row, col);
                if let Some(current_t) = intersect_time[i] {
                    if current_t <= min_intersection_time {
                        if current_t < min_intersection_time {
                            min_intersection_particles.clear();
                        }
                        min_intersection_particles.insert(row);
                        min_intersection_particles.insert(col);
                        min_intersection_time = current_t
                    }
                }
            }
        }
        if min_intersection_particles.len() > 0 {
            for &particle in min_intersection_particles.iter() {
                remaining_particles -= 1;
                for col in 0..particle {
                    let i = lower_triangle_matrix_index(particle, col);
                    intersect_time[i] = None;
                }
                for row in (particle + 1)..num_particles {
                    let i = lower_triangle_matrix_index(row, particle);
                    intersect_time[i] = None;
                }
            }
        } else {
            return remaining_particles;
        }
    }
}

fn part2(positions: &[IntVec3], velocities: &[IntVec3], accelerations: &[IntVec3]) -> usize {
    let mut intersect_time = get_all_intersections(positions, velocities, accelerations);
    return filter_colliding_particles(&mut intersect_time);
}

#[bench]
fn part2_bench(b: &mut Bencher) {
    let (p, v, a) = get_input();
    b.iter(|| {
        test::black_box(part2(&p, &v, &a));
    });
}

#[bench]
fn intersection_bench(b: &mut Bencher) {
    let (p, v, a) = get_input();
    b.iter(|| {
        test::black_box(get_all_intersections(&p, &v, &a));
    });
}

fn main() {
    let (p, v, a) = get_input();

    println!("Part 1: {}", part1(&p, &v, &a));
    println!("Part 2: {}", part2(&p, &v, &a));
}
