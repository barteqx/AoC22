use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
struct Line {
    line: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            line: vec![(0, 0); 10],
            visited: HashSet::default(),
        }
    }
}

impl Line {
    fn get_head(&mut self) -> &mut (i32, i32) {
        self.line.first_mut().unwrap()
    }
    fn move_up(&mut self) {
        self.get_head().0 += 1;
    }

    fn move_down(&mut self) {
        self.get_head().0 -= 1;
    }

    fn move_right(&mut self) {
        self.get_head().1 += 1;
    }

    fn move_left(&mut self) {
        self.get_head().1 -= 1;
    }

    fn insert_current_tail(&mut self) {
        self.visited.insert(*self.line.last().unwrap());
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
                }
                Move::Down(_) => {
                    self.move_down();
                }
                Move::Left(_) => {
                    self.move_left();
                }
                Move::Right(_) => {
                    self.move_right();
                }
            }
            self.update_tail();
        }
    }

    fn update_tail(&mut self) {
        for i in 0..self.line.len() - 1 {
            self.update_knot(i);
        }
        self.insert_current_tail();
    }

    fn update_knot(&mut self, i: usize) {
        let knot = self.line.get(i).unwrap();
        let mut next_knot = *self.line.get(i + 1).unwrap();

        let up_right_1: (i32, i32) = (next_knot.0 + 2, next_knot.1 + 1);
        let up_right_2: (i32, i32) = (next_knot.0 + 1, next_knot.1 + 2);
        let up_right_3: (i32, i32) = (next_knot.0 + 2, next_knot.1 + 2);
        let up_left_1: (i32, i32) = (next_knot.0 - 2, next_knot.1 + 1);
        let up_left_2: (i32, i32) = (next_knot.0 - 1, next_knot.1 + 2);
        let up_left_3: (i32, i32) = (next_knot.0 - 2, next_knot.1 + 2);
        let down_right_1: (i32, i32) = (next_knot.0 + 2, next_knot.1 - 1);
        let down_right_2: (i32, i32) = (next_knot.0 + 1, next_knot.1 - 2);
        let down_right_3: (i32, i32) = (next_knot.0 + 2, next_knot.1 - 2);
        let down_left_1: (i32, i32) = (next_knot.0 - 2, next_knot.1 - 1);
        let down_left_2: (i32, i32) = (next_knot.0 - 1, next_knot.1 - 2);
        let down_left_3: (i32, i32) = (next_knot.0 - 2, next_knot.1 - 2);

        if *knot == next_knot {
            return
        } else if knot.0 == next_knot.0 {
            if knot.1 == next_knot.1 - 2 {
                next_knot.1 -= 1;
            }
            if knot.1 == next_knot.1 + 2 {
                next_knot.1 += 1;
            }
        } else if knot.1 == next_knot.1 {
            if knot.0 == next_knot.0 - 2 {
                next_knot.0 -= 1;
            }
            if knot.0 == next_knot.0 + 2 {
                next_knot.0 += 1;
            }
        } else if *knot == up_right_1 || *knot == up_right_2 || *knot == up_right_3 {
            next_knot.0 += 1;
            next_knot.1 += 1;
        } else if *knot == up_left_1 || *knot == up_left_2  || *knot == up_left_3 {
            next_knot.0 -= 1;
            next_knot.1 += 1;
        } else if *knot == down_right_1 || *knot == down_right_2 || *knot == down_right_3 {
            next_knot.0 += 1;
            next_knot.1 -= 1;
        } else if *knot == down_left_1 || *knot == down_left_2 || *knot == down_left_3{
            next_knot.0 -= 1;
            next_knot.1 -= 1;
        }
        self.line[i + 1] = next_knot;
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
    println!("{}", model_line.visited.len());
}
