extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::ops;
use itertools::Itertools;
use std::time;

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

fn is_square(v: IntType) -> bool {
    let sqrt = (v as f64).sqrt();
    return (sqrt * sqrt) as IntType == v;
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
        let sqrt_term = b * b - 4 * a * c;
        if !is_square(sqrt_term) {
            return Zero;
        }
        let numerator1 = -b + (sqrt_term as f64).sqrt() as IntType;
        let numerator2 = -b - (sqrt_term as f64).sqrt() as IntType;
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
        (Zero, _) => Zero,
        (_, Zero) => Zero,
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

    let mut component_intersections: [Intersections; 3] =
        unsafe { std::mem::uninitialized() };
    component_intersections.iter_mut().set_from(
        itertools::multizip((two_a.iter(), two_b.iter(), two_c.iter()))
            .map(|(a, b, c)| solve_quadratic_positive(a, b, c)),
    );

    let intersections = intersection_union(
        intersection_union(component_intersections[0], component_intersections[1]),
        component_intersections[2],
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

fn part2(positions: &[IntVec3], velocities: &[IntVec3], accelerations: &[IntVec3]) -> usize {
    let intersect_start = time::Instant::now();
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
    println!("compute intersection time: {}ms", duration_to_milliseconds(intersect_start.elapsed()));

    let filter_start = time::Instant::now();
    let mut remaining_particles = num_particles;
    let mut min_intersection_particles: Vec<usize> = Vec::with_capacity(num_particles);
    loop {
        min_intersection_particles.clear();
        let mut min_intersection_time: Option<IntType> = None;
        for row in 1..num_particles {
            for col in 0..row {
                let i = lower_triangle_matrix_index(row, col);
                match (min_intersection_time, intersect_time[i]) {
                    (Some(min_t), Some(current_t)) => {
                        if current_t <= min_t {
                            if current_t < min_t {
                                min_intersection_particles.clear();
                            }
                            if !min_intersection_particles.contains(&row) {
                                min_intersection_particles.push(row);
                            }
                            if !min_intersection_particles.contains(&col) {
                                min_intersection_particles.push(col);
                            }
                            min_intersection_time = Some(current_t)
                        }
                    }
                    (None, Some(current_t)) => {
                        assert!(min_intersection_particles.len() == 0);
                        min_intersection_particles.push(row);
                        min_intersection_particles.push(col);
                        min_intersection_time = Some(current_t);
                    }
                    (_, None) => {}

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
            println!("filter intersections: {}ms", duration_to_milliseconds(filter_start.elapsed()));
            return remaining_particles;
        }
    }
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
    let (p, v, a) = get_input();

    let (answer1, t1) = time_fn(|| part1(&p, &v, &a));
    println!("Part 1: {} ({}ms)", answer1, t1);
    let (answer2, t2) = time_fn(|| part2(&p, &v, &a));
    println!("Part 2: {} ({}ms)", answer2, t2);
}
