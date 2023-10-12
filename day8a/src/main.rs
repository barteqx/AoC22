#[derive(Debug, Default)]
struct ForestMap {
    trees: Vec<Vec<(i32, bool)>>,
}

impl ForestMap {
    fn add_tree_line(&mut self, line: &String) {
        let tree_line: Vec<(i32, bool)> = line
            .chars()
            .into_iter()
            .map(|c| (c.to_string().parse().unwrap(), false))
            .collect();
        self.trees.push(tree_line);
    }

    fn count_trees(&mut self) -> i32 {
        for i in 0..self.trees.first().unwrap().len() {
            self.trees.first_mut().unwrap()[i].1 = true;
            self.trees.last_mut().unwrap()[i].1 = true;
        }

        for i in 0..self.trees.len() {
            self.trees[i].first_mut().unwrap().1 = true;
            self.trees[i].last_mut().unwrap().1 = true;
        }

        for i in 0..self.trees.len() {
            self.scan_line(i);
        }

        for i in 0..self.trees.first().unwrap().len() {
            self.scan_column(i);
        }

        self.trees
            .iter()
            .map(|v| v.into_iter().map(|(_, b)| *b as i32).fold(0, |a, b| a + b))
            .fold(0, |a, b| a + b)
    }

    fn scan_line(&mut self, i: usize) {
        let mut max = 0;

        for j in 0..self.trees[i].len() {
            if self.trees[i][j].0 > max {
                self.trees[i][j].1 = true;
                max = self.trees[i][j].0;
            }
        }

        max = 0;

        for j in (0..self.trees[i].len()).rev() {
            if self.trees[i][j].0 > max {
                self.trees[i][j].1 = true;
                max = self.trees[i][j].0;
            }
        }
    }

    fn scan_column(&mut self, j: usize) {
        let mut max = 0;

        for i in 0..self.trees.len() {
            if self.trees[i][j].0 > max {
                self.trees[i][j].1 = true;
                max = self.trees[i][j].0;
            }
        }

        max = 0;

        for i in (0..self.trees.len()).rev() {
            if self.trees[i][j].0 > max {
                self.trees[i][j].1 = true;
                max = self.trees[i][j].0;
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut forest = ForestMap::default();

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                forest.add_tree_line(&val);
            }
            Err(_) => (),
        }
    }
    println!("{}", forest.count_trees());
}
