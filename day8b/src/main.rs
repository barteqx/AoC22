#[derive(Debug, Default)]
struct ForestMap {
    trees: Vec<Vec<i32>>,
}

impl ForestMap {
    fn add_tree_line(&mut self, line: &String) {
        let tree_line: Vec<i32> = line
            .chars()
            .into_iter()
            .map(|c| (c.to_string().parse().unwrap()))
            .collect();
        self.trees.push(tree_line);
    }

    fn count_trees(&mut self) -> i32 {
        let mut max = 0;
        for i in 0..self.trees.len() {
            for j in 0..self.trees[i].len() {
                let current = self.get_view_field_size(&(i, j));
                if current > max {
                    max = current;
                }
            }
        }
        max as i32
    }

    fn get_view_field_size(&mut self, pos: &(usize, usize)) -> usize {
        let (i, j) = (pos.0, pos.1);

        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;

        let height = self.trees[i][j];

        loop {
            a += 1;
            let offset = j as i32 - a;
            if offset < 0 {
                a -= 1;
                break;
            }
            let x = self.trees[i][offset as usize];

            if x >= height {
                break;
            }
        }

        loop {
            b += 1;
            let offset = j as i32 + b;
            if offset >= self.trees[i].len() as i32 {
                b -= 1;
                break;
            }
            let x = self.trees[i][offset as usize];

            if x >= height {
                break;
            }
        }

        loop {
            c += 1;
            let offset = i as i32 - c;
            if offset < 0 {
                c -= 1;
                break;
            }
            let x = self.trees[offset as usize][j];

            if x >= height {
                break;
            }
        }

        loop {
            d += 1;
            let offset = i as i32 + d;
            if offset >= self.trees.len() as i32 {
                d -= 1;
                break;
            }
            let x = self.trees[offset as usize][j];

            if x >= height {
                break;
            }
        }

        // println!(
        //     "{} {:?} {:?} {:?}",
        //     height,
        //     (i, j),
        //     (a, b, c, d),
        //     (a * b * c * d)
        // );

        (a * b * c * d) as usize
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
