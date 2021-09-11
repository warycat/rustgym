use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Instruction {
    Mask(u64, u64, u64),
    Mem(u64, u64),
}

#[derive(Default)]
struct VM {
    one: u64,
    zero: u64,
    floating: u64,
    mem: HashMap<u64, u64>,
}

impl VM {
    fn exec1(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            Mask(zero, one, _) => {
                self.zero = zero;
                self.one = one;
            }
            Mem(addr, val) => {
                let entry = self.mem.entry(addr).or_default();
                *entry = val;
                *entry |= self.one;
                *entry &= !self.zero;
            }
        }
    }

    fn decode(floating: u64, one: u64, addr: u64) -> VecDeque<u64> {
        let mut queue: VecDeque<u64> = VecDeque::new();
        queue.push_back(one | addr);
        for i in 0..36 {
            let n = queue.len();
            if floating & 1 << i != 0 {
                for _ in 0..n {
                    let x = queue.pop_front().unwrap();
                    queue.push_back(x);
                    queue.push_back(x ^ 1 << i);
                }
            }
        }
        queue
    }

    fn exec2(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            Mask(zero, one, floating) => {
                self.zero = zero;
                self.one = one;
                self.floating = floating;
            }
            Mem(addr, val) => {
                for decoded_addr in VM::decode(self.floating, self.one, addr) {
                    let entry = self.mem.entry(decoded_addr).or_default();
                    *entry = val;
                }
            }
        }
    }

    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut instructions: Vec<Instruction> = vec![];
    for line in it {
        let words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        if words[0] == "mask" {
            let mut zero = 0u64;
            let mut one = 0u64;
            let mut floating = 0u64;
            let mut mask = words[2].to_string();
            for i in 0..36 {
                let c = mask.pop().unwrap();
                if c == '0' {
                    zero |= 1 << i;
                }
                if c == '1' {
                    one |= 1 << i;
                }
                if c == 'X' {
                    floating |= 1 << i;
                }
            }
            instructions.push(Instruction::Mask(zero, one, floating));
        } else {
            let address: String = words[0].chars().filter(|c| c.is_numeric()).collect();
            let addr = address.parse::<u64>().unwrap();
            let val = words[2].parse::<u64>().unwrap();
            instructions.push(Instruction::Mem(addr, val));
        }
    }
    let res1 = sum(&instructions);
    let res2 = decode(&instructions);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn sum(instructions: &[Instruction]) -> u64 {
    let mut vm = VM::default();
    for &instruction in instructions {
        vm.exec1(instruction);
    }
    vm.sum()
}

fn decode(instructions: &[Instruction]) -> u64 {
    let mut vm = VM::default();
    for &instruction in instructions {
        vm.exec2(instruction);
    }
    vm.sum()
}

adventofcode!(test, "input.txt", "output.txt");
