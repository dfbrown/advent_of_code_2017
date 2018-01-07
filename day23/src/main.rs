use std::fs::File;
use std::io::prelude::*;

const NUM_REGISTERS: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operand {
    Value(i32),
    Register(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Set(u8, Operand),
    Sub(u8, Operand),
    Mul(u8, Operand),
    Jnz(Operand, Operand),
}

fn parse_register(reg: &str) -> Option<u8> {
    if reg.len() == 1 {
        let first_char = reg.as_bytes()[0];
        if first_char >= b'a' && first_char < b'a' + NUM_REGISTERS {
            return Some(first_char - b'a');
        }
    }
    return None;
}

fn parse_operand(op: &str) -> Operand {
    if let Some(reg) = parse_register(op) {
        return Operand::Register(reg);
    }
    return Operand::Value(op.parse::<i32>().expect("Operand is not a register or an integer"));
}

fn parse_instruction(instruction: &str) -> Instruction {
    use Instruction::*;
    let mut words = instruction.split_whitespace();
    let instruction = words.next().expect("Expected instruction");
    let op1 = words.next().expect("Expected operand 1");
    let op2 = words.next().expect("Expected operand 2");
    match instruction {
        "set" => Set(parse_register(op1).expect("Invalid Register"), parse_operand(op2)),
        "sub" => Sub(parse_register(op1).expect("Invalid Register"), parse_operand(op2)),
        "mul" => Mul(parse_register(op1).expect("Invalid Register"), parse_operand(op2)),
        "jnz" => Jnz(parse_operand(op1), parse_operand(op2)),
        other => panic!("Invalid instruction {}", other)
    }
}

fn get_input() -> Vec<Instruction> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    let mut instructions = Vec::new();
    for line in input_str.lines() {
        instructions.push(parse_instruction(line));
    }

    return instructions;
}

fn operand_value(op: Operand, registers: &[i32]) -> i32 {
    use Operand::*;
    match op {
        Value(v) => v,
        Register(r) => registers[r as usize],
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    use Instruction::*;
    let mut pc = 0;
    let mut registers = [0i32; NUM_REGISTERS as usize];
    let mut mul_count = 0;
    while pc < instructions.len() {
        match instructions[pc] {
            Set(reg, op2) => {
                registers[reg as usize] = operand_value(op2, &registers);
            },
            Sub(reg, op2) => {
                registers[reg as usize] -= operand_value(op2, &registers);
            },
            Mul(reg, op2) => {
                registers[reg as usize] *= operand_value(op2, &registers);
                mul_count += 1;
            }
            Jnz(op1, op2) => {
                if operand_value(op1, &registers) != 0 {
                    // subtract 1 from the jump amount because we always increment the program
                    // counter
                    pc = pc.wrapping_add((operand_value(op2, &registers) - 1) as usize);
                }
            }
        }
        pc = pc.wrapping_add(1);
    }
    return mul_count;
}

// Manually converted and optimized
fn part2() -> usize {
    let mut b = 109300;
    let c = 126300;
    let mut h = 0;
    while b < c {
        let limit = (b as f64).sqrt() as usize;
        for d in 2..(limit + 1) {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
        b += 17;
    }
    return h;
}


fn main() {
    let instructions = get_input();
    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2());
}
