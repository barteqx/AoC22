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
    signal_strength: i32,
}

impl Cpu {
    fn new() -> Self {
        let mut x = Self::default();
        x.register = 1;
        x.cycle = 1;
        x
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
        println!("{:?}", op);
        self.exec();
    }

    fn exec(&mut self) {
        loop {
            println!(
                "cycle: {}, reg: {}",
                self.cycle,
                self.register
            );
            if ((self.cycle - 20) % 40) == 0 {
                self.signal_strength += self.cycle * self.register;
                println!(
                    "cycle: {}, reg: {}, signal: {}",
                    self.cycle,
                    self.register,
                    self.cycle * self.register
                );
            }
            self.cycle += 1;
            self.op_cycles -= 1;
            if self.op_cycles == 0 {
                self.register += self.add_reg;
                self.add_reg = 0;
            }
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

fn main() {
    let stdin = std::io::stdin();

    let mut cpu = Cpu::new();

    for line in stdin.lines() {
        match line {
            Ok(val) => cpu.execute_op(&parse_instruction(&val).unwrap()),
            Err(_) => (),
        }
    }
    println!("{}", cpu.signal_strength);
}
