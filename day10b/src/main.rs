#[derive(Debug)]
enum Op {
    Noop,
    Addx(i32),
}

#[derive(Debug, Default)]
struct Cpu {
    cycle: i32,
    op_cycles: i32,
    register: i32,
    add_reg: i32,
    screen_line:Vec<bool>,
}

impl Cpu {
    fn new() -> Self {
        let mut x = Self::default();
        x.register = 1;
        x.cycle = 1;
        x.screen_line = vec![false; 40];
        x
    }

    fn print_px(&mut self, cycle: i32) {
        if (self.register-1..=self.register+1).contains(&cycle) {
            self.screen_line[cycle as usize] = true;
        }
    }

    fn execute_op(&mut self, op: &Op) {
        match op {
            Op::Noop => {
                self.op_cycles = 1;
            }
            Op::Addx(x) => {
                self.op_cycles = 2;
                self.add_reg = *x;
            }
        }
        self.exec();
    }

    fn exec(&mut self) {
        loop {
            let cycle_in_line = self.cycle % 40;
            if (cycle_in_line) == 0 {
                print_line(&self.screen_line);
                self.screen_line = vec![false; 40];
            }
            self.cycle += 1;
            self.op_cycles -= 1;
            if self.op_cycles == 0 {
                self.register += self.add_reg;
                self.add_reg = 0;
            }
            self.print_px(cycle_in_line);
            if self.op_cycles == 0 {
                break;
            }
        }
    }
}

fn parse_instruction(instruction: &String) -> Result<Op, String> {
    let mut tokens = instruction.split(' ');
    let ins = tokens.next().unwrap();
    let reg = tokens.next();

    match ins {
        "noop" => Ok(Op::Noop),
        "addx" => Ok(Op::Addx(reg.unwrap().parse::<i32>().unwrap())),
        _ => Err("Wrong command!".to_string()),
    }
}

fn print_line(v: &Vec<bool>) {
    for px in v {
        match px {
            true => print!("#"),
            false => print!(" "),
        }
    }
    println!();
}

fn main() {
    let stdin = std::io::stdin();

    let mut cpu = Cpu::new();

    for line in stdin.lines() {
        match line {
            Ok(val) => cpu.execute_op(&parse_instruction(&val).unwrap()),
            Err(_) => (),
        }
    }
}
