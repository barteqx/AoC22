use std::collections::HashMap;

#[derive(Debug)]
struct Directories {
    dirs: HashMap<String, i64>,
    current_dir: Vec<String>,
}

impl Default for Directories {
    fn default() -> Self { 
        Self {
            dirs: HashMap::from([("".to_string(), 0 as i64)]),
            current_dir: Vec::default(),
        }
     }
}

impl Directories {
    fn add_dir(&mut self, dir: &String) {
        let new_dir = get_new_path(&self.current_dir, &dir.to_string());
        self.dirs.insert(new_dir, 0);
    }

    fn add_size_to_path(&mut self, size: i64) {
        let mut i = self.current_dir.len();
        let mut current_slice: &[String];
        while {
            current_slice = &self.current_dir[..i];
            let path = &get_concatenated_path(&current_slice.to_vec());
            let newsize = self.dirs.get(path).unwrap() + size;
            self.dirs.insert(path.to_string(), newsize);
            if i > 0 { i -= 1; }

            !current_slice.is_empty()
        } {}
    }

    fn change_directory(&mut self, dir: &str) {
        match dir {
            "/" => self.current_dir.clear(),
            ".." => {
                self.current_dir.pop().unwrap();
            }
            _ => {
                let new_dir = get_new_path(&self.current_dir, &dir.to_string());
                self.dirs.get(&new_dir).unwrap();
                self.current_dir.push(dir.to_string());
            }
        }
    }
}

fn get_concatenated_path(path: &Vec<String>) -> String {
    path.join("/")
}

fn get_new_path(path: &Vec<String>, dir: &String) -> String {
    let mut new_dir = get_concatenated_path(path);
    if !new_dir.is_empty() {
        new_dir.push_str("/");
    }
    new_dir.push_str(dir);
    new_dir
}

fn main() {
    let stdin = std::io::stdin();

    let mut dirs = Directories::default();

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                let cmd: Vec<&str> = val.split_whitespace().collect();
                match cmd[0] {
                    "$" => match cmd[1] {
                        "cd" => {
                            dirs.change_directory(cmd[2]);
                        }
                        _ => (),
                    },
                    "dir" => {
                        dirs.add_dir(&cmd[1].to_string());
                    }
                    _ => {
                        let size = cmd[0].parse::<i64>();
                        match size {
                            Ok(x) => dirs.add_size_to_path(x),
                            Err(_) => (),
                        }
                    }
                }
            }
            Err(_) => (),
        }
    }
    let score = dirs
        .dirs
        .iter()
        .filter(|&(_, v)| v < &100000)
        .map(|(_, v)| v)
        .fold(0, |a, b| a + b);
    println!("{}", score)
}
