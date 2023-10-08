use std::collections::HashSet;

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96,
        'A'..='Z' => c as u32 - 38,
        _ => 0,
    }
}

fn get_halves(s: &String) -> (&str, &str) {
    let half_len = s.len() / 2;
    (&s[..half_len], &s[half_len..])
}

fn find_common_item(s: (&str, &str)) -> char {
    let mut set = HashSet::new();
    for c in s.0.chars() {
        set.insert(c);
    }

    for c in s.1.chars() {
        if set.contains(&c) {
            return c;
        }
    }
    ' '
}

fn main() {
    let stdin = std::io::stdin();

    let mut score = 0;

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                score += get_priority(find_common_item(get_halves(&val)));
            }
            Err(_) => (),
        }
    }
    println!("{}", score)
}
