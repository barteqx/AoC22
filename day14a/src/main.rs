use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Default)]
struct CaveMap {
    map: HashSet<(u32, u32)>,
    max_depth: u32,
}

impl CaveMap {
    fn add_point(&mut self, point: (u32, u32)) {
        self.map.insert(point);

        if point.1 > self.max_depth {
            self.max_depth = point.1;
        }
    }

    fn add_line(&mut self, line: ((u32, u32), (u32, u32))) {
        let min_x = min(line.0 .0, line.1 .0);
        let min_y = min(line.0 .1, line.1 .1);
        let max_x = max(line.0 .0, line.1 .0);
        let max_y = max(line.0 .1, line.1 .1);

        for i in min_x..=max_x {
            for j in min_y..=max_y {
                self.add_point((i, j));
            }
        }
    }
    fn simulate(&mut self) -> bool {
        let mut current = (500, 1);
        loop {
            let (x, y) = current;
            if y >= self.max_depth {
                return false;
            }
            if !self.map.contains(&(x, y+1)) {
                current = (x, y+1);
                continue;
            }
            if !self.map.contains(&(x-1, y+1)) {
                current = (x-1, y+1);
                continue;
            }
            if !self.map.contains(&(x+1, y+1)) {
                current = (x+1, y+1);
                continue;
            }
            break
        }
        self.map.insert(current);
        true
    }

    fn simulate_sand(&mut self) -> u32 {
        let mut i = 0;
        while self.simulate(){
            i += 1;
        }
        i
    }
}

fn rock_into_lines(rock: &mut Vec<(u32, u32)>) -> Vec<((u32, u32), (u32, u32))> {
    let mut p1 = rock.pop().unwrap();
    let mut lines = vec![];

    while !rock.is_empty() {
        let p2 = rock.pop().unwrap();
        lines.push((p1, p2));
        p1 = p2;
    }
    lines
}

fn parse_line(line: &String) -> Vec<(u32, u32)> {
    let split1 = line.split(" -> ");
    let mut points = vec![];
    for point in split1 {
        let mut split2 = point.split(',');
        points.push((
            split2.next().unwrap().parse::<u32>().unwrap(),
            split2.next().unwrap().parse::<u32>().unwrap(),
        ));
    }
    points
}

fn main() {
    let stdin = std::io::stdin();

    let mut map = CaveMap::default();

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                for l in rock_into_lines(&mut parse_line(&val)) {
                    map.add_line(l);
                }
            }
            Err(_) => (),
        }
    }
    println!("{}", map.simulate_sand());
}
