use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

type RegIndex = u8;
type RegType = isize;
const REGISTER_COUNT: usize = 16;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum Operand {
    Value(RegType),
    Register(RegIndex),
}

#[derive(Debug)]
enum Instruction {
    Snd(Operand),
    Set(RegIndex, Operand),
    Add(RegIndex, Operand),
    Mul(RegIndex, Operand),
    Mod(RegIndex, Operand),
    Rcv(RegIndex),
    Jgz(Operand, Operand),
}

enum InstructionResult {
    Continue,
    SendValue(RegType),
    Stalled,
}

#[derive(Default)]
#[derive(Debug)]
struct MachineState {
    registers: [RegType; REGISTER_COUNT],
    pc: usize,
    rcv_queue: VecDeque<RegType>,
}

fn parse_register(operand: &str) -> RegIndex {
    assert!(operand.len() == 1, "Registers should be single characters (got {})", operand);
    let reg_index = operand.as_bytes()[0] - b'a';
    assert!((reg_index as usize) < REGISTER_COUNT, "Register index too large: {}", operand);
    return reg_index;
}

impl Operand {
    fn parse(operand: &str) -> Operand {
        if let Ok(value) = operand.parse::<RegType>() {
            return Operand::Value(value);
        } else {
            return Operand::Register(parse_register(operand));
        }
    }
    fn value(&self, state: &MachineState) -> RegType {
        match *self {
            Operand::Value(v) => v,
            Operand::Register(r) => state.registers[r as usize]
        }
    }
}

impl Instruction {
    fn parse(instruction: &str) -> Instruction {
        let mut words = instruction.split_whitespace();
        let instruction = words.next().expect("No instruction?");
        return match instruction {
            "snd" => {
                Instruction::Snd(Operand::parse(words.next().expect("No snd operand?")))
            }
            "set" => {
                Instruction::Set(
                    parse_register(words.next().expect("No set first operand?")),
                    Operand::parse(words.next().expect("No set second operand?")))
            }
            "add" => {
                Instruction::Add(
                    parse_register(words.next().expect("No add first operand?")),
                    Operand::parse(words.next().expect("No add second operand?")))
            }
            "mul" => {
                Instruction::Mul(
                    parse_register(words.next().expect("No mul first operand?")),
                    Operand::parse(words.next().expect("No mul second operand?")))
            }
            "mod" => {
                Instruction::Mod(
                    parse_register(words.next().expect("No mod first operand?")),
                    Operand::parse(words.next().expect("No mod second operand?")))
            }
            "rcv" => {
                Instruction::Rcv(parse_register(words.next().expect("No rcv operand")))
            }
            "jgz" => {
                Instruction::Jgz(
                    Operand::parse(words.next().expect("No jgz first operand")),
                    Operand::parse(words.next().expect("No jgz second operand")))
            }
            other => panic!("Unexpected instruction {}", other)
        }
    }
}

impl MachineState {
    fn run_instruction(&mut self, instruction: &Instruction) -> InstructionResult {
        use Instruction::*;
        use InstructionResult::*;
        let mut pc_increment: isize = 1;
        let result = match instruction {
            &Snd(operand) => {
                SendValue(operand.value(self))
            }
            &Set(reg_index, operand) => {
                self.registers[reg_index as usize] = operand.value(self);
                Continue
            }
            &Add(reg_index, operand) => {
                self.registers[reg_index as usize] += operand.value(self);
                Continue
            }
            &Mul(reg_index, operand) => {
                self.registers[reg_index as usize] *= operand.value(self);
                Continue
            }
            &Mod(reg_index, operand) => {
                self.registers[reg_index as usize] %= operand.value(self);
                Continue
            }
            &Rcv(reg_index) => {
                if let Some(v) = self.rcv_queue.pop_front() {
                    self.registers[reg_index as usize] = v;
                    Continue
                } else {
                    pc_increment = 0;
                    Stalled
                }
            }
            &Jgz(op1, op2) => {
                if op1.value(self) > 0 {
                    pc_increment = op2.value(self) as isize;
                }
                Continue
            }
        };
        self.pc = self.pc.wrapping_add(pc_increment as usize);
        return result;
    }
}

fn part1(instructions: &[Instruction]) -> Option<RegType> {
    use InstructionResult::*;
    let mut state: MachineState = Default::default();
    let mut last_snd = None;
    while let Some(instruction) = instructions.get(state.pc) {
        match state.run_instruction(instruction) {
            SendValue(v) => last_snd = Some(v),
            Stalled => return last_snd,
            Continue => {}
        }
    }
    return None;
}

fn part2(instructions: &[Instruction]) -> RegType {
    use InstructionResult::*;
    const NUM_STATES: usize = 2;
    let mut states: [MachineState; NUM_STATES] = Default::default();
    for (i, state) in states.iter_mut().enumerate() {
        state.registers[parse_register("p") as usize] = i as RegType;
    }
    let mut stalled = [false; NUM_STATES];
    let mut sends = [0; NUM_STATES];

    // Iteratively run each program until stalled. Stop when all programms are stalled.
    let mut current_program_index = 0;
    while stalled.iter().any(|&x| !x) {
        let next_program_index = (current_program_index + 1) % states.len();
        // Run the current program until it finishes or stalls
        while let Some(instruction) = instructions.get(states[current_program_index].pc) {
            match states[current_program_index].run_instruction(instruction) {
                SendValue(v) => {
                    states[next_program_index].rcv_queue.push_back(v);
                    sends[current_program_index] += 1;
                    stalled[next_program_index] = false;
                }
                Stalled => {
                    break;
                }
                Continue => {}
            }
        }
        stalled[current_program_index] = true;
        current_program_index = next_program_index;
    }

    return sends[1];
}

fn main() {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    let instructions: Vec<_> = input_str.split_terminator('\n').map(Instruction::parse).collect();

    println!("Part 1: {:}", part1(&instructions).expect("Program ended without part1 result"));
    println!("Part 2: {:}", part2(&instructions));
}
