use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Operation {
    lhs: Option<i32>,
    rhs: Option<i32>,
    operation: fn(i32, i32) -> i32,
}

impl Operation {
    fn create(tokens: Vec<&str>) -> Self {
        let (lhs, op, rhs) = (tokens[0], tokens[1], tokens[2]);
        Self {
            lhs: match lhs.parse::<i32>() {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            rhs: match rhs.parse::<i32>() {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            operation: match op {
                "+" => |a, b| -> i32 { a + b },
                "-" => |a, b| -> i32 { a - b },
                "*" => |a, b| -> i32 { a * b },
                "/" => |a, b| -> i32 { a / b },
                _ => panic!("unknown operation!"),
            },
        }
    }

    fn execute(&self, current: i32) -> i32 {
        (self.operation)(self.lhs.unwrap_or(current), self.rhs.unwrap_or(current))
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i32>,
    operation: Operation,
    test: i32,
    if_true: usize,
    if_false: usize,
    item_conter: i32,
}

#[derive(Debug)]
struct MonkeyPark {
    monkeys: Vec<Monkey>,
}

impl MonkeyPark {
    fn new() -> Self {
        Self { monkeys: vec![] }
    }

    fn inspect(&mut self, monkey: usize) {
        let mut new_monkey = self.monkeys[monkey].clone();
        while !new_monkey.items.is_empty() {
            new_monkey.item_conter += 1;
            let mut element = new_monkey.items.pop_front().unwrap();
            element = new_monkey.operation.execute(element);
            element /= 3;
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
}

fn read_items(line: &String) -> VecDeque<i32> {
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

fn read_param(line: &String) -> Result<i32, ()> {
    let split_line = line.split(" ");
    for token in split_line.into_iter() {
        let parsed = token.parse::<i32>();
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
        item_conter: 0,
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

    for _ in 0..20 {
        monkeys.do_round();
    }

    monkeys.monkeys.sort_by_key(|k| k.item_conter);
    monkeys.monkeys.reverse();

    println!(
        "{}",
        monkeys.monkeys[0].item_conter * monkeys.monkeys[1].item_conter
    )
}
