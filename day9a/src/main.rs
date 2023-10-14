use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Default)]
struct Line {
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Line {
    fn move_up(&mut self) {
        self.head.0 += 1;
    }

    fn move_down(&mut self) {
        self.head.0 -= 1;
    }

    fn move_right(&mut self) {
        self.head.1 += 1;
    }

    fn move_left(&mut self) {
        self.head.1 -= 1;
    }

    fn insert_current_tail(&mut self) {
        self.visited.insert(self.tail);
    }

    fn move_head(&mut self, mv: Move) {
        let distance: i32;
        match mv {
            Move::Up(x) => distance = x,
            Move::Down(x) => distance = x,
            Move::Left(x) => distance = x,
            Move::Right(x) => distance = x,
        }

        for _ in 0..distance {
            match mv {
                Move::Up(_) => {
                    self.move_up();
                    self.update_tail();
                }
                Move::Down(_) => {
                    self.move_down();
                    self.update_tail();
                }
                Move::Left(_) => {
                    self.move_left();
                    self.update_tail();
                }
                Move::Right(_) => {
                    self.move_right();
                    self.update_tail();
                }
            }
        }
    }

    fn update_tail(&mut self) {
        if self.tail == self.head {
            return;
        }

        if self.head.0 == self.tail.0 {
            if self.head.1 == self.tail.1 - 2 {
                self.tail.1 -= 1;
            }
            if self.head.1 == self.tail.1 + 2 {
                self.tail.1 += 1;
            }
            self.insert_current_tail();
            return;
        }

        if self.head.1 == self.tail.1 {
            if self.head.0 == self.tail.0 - 2 {
                self.tail.0 -= 1;
            }
            if self.head.0 == self.tail.0 + 2 {
                self.tail.0 += 1;
            }
            self.insert_current_tail();
            return;
        }

        let up_right_1: (i32, i32) = (self.tail.0 + 2, self.tail.1 + 1);
        let up_right_2: (i32, i32) = (self.tail.0 + 1, self.tail.1 + 2);
        let up_left_1: (i32, i32) = (self.tail.0 - 2, self.tail.1 + 1);
        let up_left_2: (i32, i32) = (self.tail.0 - 1, self.tail.1 + 2);
        let down_right_1: (i32, i32) = (self.tail.0 + 2, self.tail.1 - 1);
        let down_right_2: (i32, i32) = (self.tail.0 + 1, self.tail.1 - 2);
        let down_left_1: (i32, i32) = (self.tail.0 - 2, self.tail.1 - 1);
        let down_left_2: (i32, i32) = (self.tail.0 - 1, self.tail.1 - 2);

        if self.head == up_right_1 || self.head == up_right_2 {
            self.tail.0 += 1;
            self.tail.1 += 1;
        } else if self.head == up_left_1 || self.head == up_left_2 {
            self.tail.0 -= 1;
            self.tail.1 += 1;
        } else if self.head == down_right_1 || self.head == down_right_2 {
            self.tail.0 += 1;
            self.tail.1 -= 1;
        } else if self.head == down_left_1 || self.head == down_left_2 {
            self.tail.0 -= 1;
            self.tail.1 -= 1;
        }
        self.insert_current_tail();
    }
}

fn covert_line_to_move(line: &String) -> Result<Move, String> {
    let mut split_line = line.split(' ');
    let direction = split_line.next().unwrap();
    let distance: i32 = split_line.next().unwrap().parse().unwrap();

    match direction {
        "U" => Ok(Move::Up(distance)),
        "D" => Ok(Move::Down(distance)),
        "R" => Ok(Move::Right(distance)),
        "L" => Ok(Move::Left(distance)),
        _ => Err("Wrong move.".to_string()),
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut model_line = Line::default();

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                model_line.move_head(covert_line_to_move(&val).unwrap());
            }
            Err(_) => (),
        }
    }
    // println!("{:?}", model_line);
    println!("{}", model_line.visited.len());
}
