use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Operation {
    lhs: Option<u64>,
    rhs: Option<u64>,
    operation: fn(u64, u64) -> u64,
}

impl Operation {
    fn create(tokens: Vec<&str>) -> Self {
        let (lhs, op, rhs) = (tokens[0], tokens[1], tokens[2]);
        Self {
            lhs: match lhs.parse::<u64>() {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            rhs: match rhs.parse::<u64>() {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            operation: match op {
                "+" => |a, b| -> u64 { a + b },
                "-" => |a, b| -> u64 { a - b },
                "*" => |a, b| -> u64 { a * b },
                "/" => |a, b| -> u64 { a / b },
                _ => panic!("unknown operation!"),
            },
        }
    }

    fn execute(&self, current: u64) -> u64 {
        (self.operation)(self.lhs.unwrap_or(current), self.rhs.unwrap_or(current))
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    item_counter: u64,
}

#[derive(Debug)]
struct MonkeyPark {
    monkeys: Vec<Monkey>,
    supermodulo: u64,
}

impl MonkeyPark {
    fn new() -> Self {
        Self {
            monkeys: vec![],
            supermodulo: 1,
        }
    }

    fn inspect(&mut self, monkey: usize) {
        let mut new_monkey = self.monkeys[monkey].clone();
        while !new_monkey.items.is_empty() {
            new_monkey.item_counter += 1;
            let mut element = new_monkey.items.pop_front().unwrap();
            element = new_monkey.operation.execute(element);
            element = element % self.supermodulo;
            let test = element % new_monkey.test == 0;
            if test {
                self.monkeys[new_monkey.if_true].items.push_back(element);
            } else {
                self.monkeys[new_monkey.if_false].items.push_back(element);
            }
        }
        self.monkeys[monkey] = new_monkey;
    }

    fn do_round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.inspect(i);
        }
    }

    fn calculate_supermod(&mut self) {
        for monkey in self.monkeys.iter() {
            self.supermodulo *= monkey.test;
        }
    }
}

fn read_items(line: &String) -> VecDeque<u64> {
    let mut out = VecDeque::new();
    let trimmed_line = line.replace("  Starting items: ", "");
    let split_line = trimmed_line.split(", ");
    for item in split_line {
        out.push_back(item.parse().unwrap());
    }
    out
}

fn read_operation(line: &String) -> Operation {
    let trimmed_line = line.replace("  Operation: new = ", "");
    let split_line: Vec<&str> = trimmed_line.split(" ").collect();
    Operation::create(split_line)
}

fn read_param(line: &String) -> Result<u64, ()> {
    let split_line = line.split(" ");
    for token in split_line.into_iter() {
        let parsed = token.parse::<u64>();
        match parsed {
            Ok(x) => return Ok(x),
            Err(_) => (),
        }
    }

    return Err(());
}

fn read_monkey(lines: Vec<String>) -> Monkey {
    Monkey {
        items: read_items(&lines[1]),
        operation: read_operation(&lines[2]),
        test: read_param(&lines[3]).unwrap(),
        if_true: read_param(&lines[4]).unwrap() as usize,
        if_false: read_param(&lines[5]).unwrap() as usize,
        item_counter: 0,
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut monkeys = MonkeyPark::new();
    let mut lines = vec![];

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                if val.is_empty() {
                    continue;
                }
                lines.push(val);
            }
            Err(_) => break,
        }

        if lines.len() == 6 {
            monkeys.monkeys.push(read_monkey(lines));
            lines = vec![];
        }
    }
    monkeys.calculate_supermod();

    for _ in 0..10000 {
        monkeys.do_round();
    }

    monkeys.monkeys.sort_by_key(|k| k.item_counter);
    monkeys.monkeys.reverse();

    println!(
        "{}",
        monkeys.monkeys[0].item_counter * monkeys.monkeys[1].item_counter
    )
}
