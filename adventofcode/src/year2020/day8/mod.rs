use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

#[derive(Clone, Copy, Debug)]
enum Code {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

struct VM {
    codes: Vec<Code>,
    acc: i32,
    idx: usize,
    visited: Vec<bool>,
    n: usize,
}

impl VM {
    fn new(codes: Vec<Code>) -> Self {
        let n = codes.len();
        let acc = 0;
        let idx = 0;
        let visited = vec![false; n];
        VM {
            codes,
            acc,
            idx,
            visited,
            n,
        }
    }

    fn exec(&mut self) -> bool {
        let code = self.codes[self.idx];
        self.visited[self.idx] = true;
        match code {
            Code::Nop(_) => {
                self.idx += 1;
            }
            Code::Acc(val) => {
                self.acc += val;
                self.idx += 1;
            }
            Code::Jmp(val) => {
                self.idx = (self.idx as i32 + val) as usize;
            }
        }
        if self.idx == self.n {
            true
        } else {
            self.visited[self.idx]
        }
    }

    fn exec_all(&mut self) -> bool {
        loop {
            if self.exec() {
                break;
            }
        }
        self.idx == self.n
    }
}

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut codes: Vec<Code> = vec![];
    for line in it {
        let words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let val = words[1].parse::<i32>().unwrap();
        match words[0].as_str() {
            "nop" => {
                codes.push(Code::Nop(val));
            }
            "acc" => {
                codes.push(Code::Acc(val));
            }
            "jmp" => {
                codes.push(Code::Jmp(val));
            }
            _ => {}
        }
    }
    let mut vm = VM::new(codes.clone());
    vm.exec_all();
    let res1 = vm.acc;
    let n = codes.len();
    for i in 0..n {
        let mut new_codes = codes.clone();
        match codes[i] {
            Code::Acc(_) => {
                continue;
            }
            Code::Jmp(val) => {
                new_codes[i] = Code::Nop(val);
            }
            Code::Nop(val) => {
                new_codes[i] = Code::Jmp(val);
            }
        }
        vm = VM::new(new_codes);
        if vm.exec_all() {
            break;
        }
    }
    let res2 = vm.acc;
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

adventofcode!(test, "input.txt", "output.txt");
