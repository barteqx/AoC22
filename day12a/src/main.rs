use std::collections::VecDeque;

#[derive(Default)]
struct Graph {
    height_map: Vec<Vec<u32>>,
    visited: Vec<Vec<bool>>,
    distance: Vec<Vec<u32>>,
    queue: VecDeque<(usize, usize)>,
    start: (usize, usize),
    destination: (usize, usize),
}

impl Graph {
    fn add_line(&mut self, line: &String) {
        let mut height_v = vec![];
        let mut visited_v = vec![];
        let mut distance_v = vec![];

        for c in line.chars() {
            height_v.push(match c {
                'S' => 'a' as u32,
                'E' => 'z' as u32,
                _ =>c as u32
            });
            visited_v.push(false);
            distance_v.push(match c {
                'S' => {
                    self.start = (self.height_map.len(), height_v.len()-1);
                    0
                }
                'E' => {
                    self.destination = (self.height_map.len(), height_v.len()-1);
                    u32::MAX
                }
                _ => u32::MAX,
            });
        }
        self.height_map.push(height_v);
        self.visited.push(visited_v);
        self.distance.push(distance_v);
        self.queue.push_back(self.start);
    }

    fn get_unvisited_neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = (pos.0 as i32, pos.1 as i32);
        let mut neighbors = vec![];
         for (i,j) in [(x, y-1), (x, y+1), (x-1, y), (x+1, y)]{
                if !(0..self.height_map.len() as i32).contains(&i)
                    || !(0..self.height_map.first().unwrap().len() as i32).contains(&j)
                {
                    continue;
                }
                if !self.visited[i as usize][j as usize]
                    && self.height_map[x as usize][y as usize] + 1
                        >= self.height_map[i as usize][j as usize]
                {
                    if !self.queue.contains(&(i as usize, j as usize)) {
                        neighbors.push((i as usize, j as usize));
                    }
                }
            }
        neighbors
    }

    fn bfs(&mut self) {
        while !self.queue.is_empty() {
            let (i, j) = self.queue.pop_front().unwrap();
            if (i,j) == self.destination {
                return;
            }
            self.visited[i][j] = true;

            let neighbors = self.get_unvisited_neighbors((i, j));

            for (x, y) in neighbors {
                if self.distance[x][y] > self.distance[i][j] + 1 {
                    self.distance[x][y] = self.distance[i][j] + 1;
                }
                self.queue.push_back((x,y));
            }
        }
    }

    fn get_distance_to_dest(&self) -> u32 {
        self.distance[self.destination.0][self.destination.1]
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut map = Graph::default();

    for line in stdin.lines() {
        match line {
            Ok(val) => map.add_line(&val),
            Err(_) => (),
        }
    }
    map.bfs();
    println!("{}", map.get_distance_to_dest());
}
