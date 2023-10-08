use std::collections::HashSet;

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96,
        'A'..='Z' => c as u32 - 38,
        _ => 0,
    }
}

fn find_common_item(v: &Vec<String>) -> char {
    let mut set = HashSet::new();
    let mut set2 = HashSet::new();
    for c in v[0].chars() {
        set.insert(c);
    }

    for c in v[1].chars() {
        if set.contains(&c) {
            set2.insert(c);
        }
    }

    for c in v[2].chars() {
        if set2.contains(&c) {
            return c;
        }
    }
    ' '
}

fn main() {
    let stdin = std::io::stdin();

    let mut score = 0;
    let mut elves = vec![String::new(), String::new(), String::new()];
    let mut elf_in_group = 0;

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                elves[elf_in_group] = val;
                elf_in_group = (elf_in_group + 1) % 3;
            }
            Err(_) => panic!("something went wrong!"),
        }
        if elf_in_group == 0 {
            score += get_priority(find_common_item(&elves));
        }
    }
    println!("{}", score)
}
