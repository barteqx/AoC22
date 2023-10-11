use std::collections::{HashSet, VecDeque};

#[derive(Debug, Default)]
struct Detector {
    len: usize,
    recent_letters: VecDeque<char>,
    count: i32,
}

impl Detector {
    fn new(len: usize) -> Self {
        Self {
            len: len,
            recent_letters: VecDeque::new(),
            count: 0,
        }
    }
    fn push_letter(&mut self, letter: char) -> (bool, i32) {
        self.count += 1;
        if self.recent_letters.len() > self.len - 1 {
            self.recent_letters.pop_back();
        }
        self.recent_letters.push_front(letter);

        let mut set: HashSet<char> = HashSet::default();

        for l in self.recent_letters.iter() {
            set.insert(*l);
        }

        if set.len() == self.len {
            return (true, self.count);
        }
        (false, self.count)
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut detector = Detector::new(4);
    let mut detector2 = Detector::new(14);

    let mut first_found = false;
    let mut second_found = false;

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                for letter in val.chars() {
                    let (res, offset) = detector.push_letter(letter);
                    let (res2, offset2) = detector2.push_letter(letter);

                    if res && !first_found {
                        println!("first: {}", offset);
                        first_found = true;
                    }

                    if res2 && !second_found {
                        println!("second: {}", offset2);
                        second_found = true;
                    }
                }
            }
            Err(_) => (),
        }
    }
}
