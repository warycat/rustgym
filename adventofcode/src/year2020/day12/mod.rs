use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

struct Instruction {
    c: char,
    val: i32,
}

impl Instruction {
    fn new(s: String) -> Self {
        let val = s[1..].parse::<i32>().unwrap();
        let c = s.chars().next().unwrap();
        Instruction { c, val }
    }
}

#[derive(Default)]
struct State1 {
    ship: Pos,
    dir: i32,
}

impl State1 {
    fn exec(&mut self, instruction: &Instruction) {
        let val = instruction.val;
        let c = instruction.c;
        match c {
            'N' => self.ship.y += val,
            'S' => self.ship.y -= val,
            'E' => self.ship.x += val,
            'W' => self.ship.x -= val,
            'L' => self.dir += val,
            'R' => self.dir -= val,
            'F' => {
                while self.dir < 0 {
                    self.dir += 360;
                }
                self.dir %= 360;
                if self.dir == 0 {
                    self.ship.x += val;
                }
                if self.dir == 90 {
                    self.ship.y += val;
                }
                if self.dir == 180 {
                    self.ship.x -= val;
                }
                if self.dir == 270 {
                    self.ship.y -= val;
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Default)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    fn l90(&self) -> Self {
        Pos::new(-self.y, self.x)
    }

    fn l180(&self) -> Self {
        self.l90().l90()
    }

    fn l270(&self) -> Self {
        self.l180().l90()
    }
}

#[derive(Debug)]
struct State2 {
    ship: Pos,
    waypoint: Pos,
}

impl State2 {
    fn default() -> Self {
        let ship = Pos::new(0, 0);
        let waypoint = Pos::new(10, 1);
        State2 { ship, waypoint }
    }

    fn exec(&mut self, instruction: &Instruction) {
        let val = instruction.val;
        let c = instruction.c;
        match c {
            'N' => self.waypoint.y += val,
            'S' => self.waypoint.y -= val,
            'E' => self.waypoint.x += val,
            'W' => self.waypoint.x -= val,
            'L' => {
                if val == 90 {
                    self.waypoint = self.waypoint.l90();
                }
                if val == 180 {
                    self.waypoint = self.waypoint.l180();
                }
                if val == 270 {
                    self.waypoint = self.waypoint.l270();
                }
            }
            'R' => {
                if val == 90 {
                    self.waypoint = self.waypoint.l270();
                }
                if val == 180 {
                    self.waypoint = self.waypoint.l180();
                }
                if val == 270 {
                    self.waypoint = self.waypoint.l90();
                }
            }
            'F' => {
                let dx = val * self.waypoint.x;
                let dy = val * self.waypoint.y;
                self.ship.x += dx;
                self.ship.y += dy;
            }
            _ => {}
        }
    }
}

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    #[allow(clippy::redundant_closure)]
    let instructions: Vec<Instruction> = it.map(|s| Instruction::new(s)).collect();
    let res1 = manhattan_distance1(&instructions);
    let res2 = manhattan_distance2(&instructions);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn manhattan_distance1(instructions: &[Instruction]) -> i32 {
    let mut state = State1::default();
    for instruction in instructions {
        state.exec(instruction);
    }
    state.ship.x.abs() + state.ship.y.abs()
}

fn manhattan_distance2(instructions: &[Instruction]) -> i32 {
    let mut state = State2::default();
    for instruction in instructions {
        state.exec(instruction);
    }
    state.ship.x.abs() + state.ship.y.abs()
}

adventofcode!(test, "input.txt", "output.txt");
