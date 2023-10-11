use std::collections::{HashMap, VecDeque};

trait Storageable {
    fn stack_crate(&mut self, index: i32, value: char);
    fn crates_move(&mut self, from: i32, to: i32, amount: i32);
    fn get_top_crate(&self, index: i32) -> char;
    fn get_stacks_len(&self) -> usize;
}

#[derive(Debug, Default)]
struct Storage {
    stacks: HashMap<i32, VecDeque<char>>,
}

impl Storageable for Storage {
    fn stack_crate(&mut self, index: i32, value: char) {
        if !self.stacks.contains_key(&index) {
            self.stacks.insert(index, VecDeque::new());
        }
        self.stacks.get_mut(&index).unwrap().push_front(value);
    }

    fn crates_move(&mut self, from: i32, to: i32, amount: i32) {
        let stacks = &mut self.stacks;
        let original_stack = stacks.get_mut(&from).unwrap();

        let mut mv_crates = VecDeque::new();
        for _ in 0..amount {
            mv_crates.push_front(original_stack.pop_back().unwrap());
        }

        drop(original_stack);

        let new_stack = stacks.get_mut(&to).unwrap();
        for _ in 0..mv_crates.len() {
            new_stack.push_back(mv_crates.pop_back().unwrap());
        }
    }

    fn get_top_crate(&self, index: i32) -> char {
        self.stacks.get(&index).unwrap().back().unwrap().clone()
    }

    fn get_stacks_len(&self) -> usize {
        self.stacks.len()
    }
}

fn split_line(line: &String) -> Vec<char> {
    let mut i = 0;
    let chars: Vec<char> = line.chars().collect();
    let mut out = vec![];
    while i * 4 + 1 < chars.len() {
        out.push(chars[4 * i + 1]);
        i += 1;
    }

    out
}

fn parse_move(line: &String) -> (i32, i32, i32) {
    let line_split: Vec<&str> = line.split(' ').collect();
    (
        line_split[1].parse::<i32>().unwrap(),
        line_split[3].parse::<i32>().unwrap(),
        line_split[5].parse::<i32>().unwrap(),
    )
}

fn main() {
    let stdin = std::io::stdin();
    let mut stacks = Storage::default();
    let mut building_stacks = true;

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                match val.as_str() {
                    "" => {
                        building_stacks = false;
                        continue;
                    }
                    _ => (),
                }

                if building_stacks {
                    let chars = split_line(&val);
                    for i in 0..chars.len() {
                        match chars[i] {
                            'A'..='Z' => stacks.stack_crate(i as i32 + 1, chars[i]),
                            _ => (),
                        }
                    }
                } else {
                    let mv = parse_move(&val);
                    stacks.crates_move(mv.1, mv.2, mv.0);
                }
            }
            Err(_) => panic!("something went wrong!"),
        }
    }

    println!();
    for i in 1..=stacks.get_stacks_len() {
        print!("{}", stacks.get_top_crate(i as i32));
    }

    println!()
}
