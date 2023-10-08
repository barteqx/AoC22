fn parse_line(line: &String) -> (i32, i32, i32, i32) {
    let mut ranges = line.split(',');
    let a: i32;
    let b: i32;
    let c: i32;
    let d: i32;

    let range1 = ranges.next().unwrap();
    let mut range_split = range1.split('-');
    a = range_split.next().unwrap().parse().unwrap();
    b = range_split.next().unwrap().parse().unwrap();

    let range2 = ranges.next().unwrap();
    range_split = range2.split('-');
    c = range_split.next().unwrap().parse().unwrap();
    d = range_split.next().unwrap().parse().unwrap();
    (a, b, c, d)
}

fn overlaps(range: &(i32, i32, i32, i32)) -> bool {
    (range.2..=range.3).contains(&range.0)
        || (range.2..=range.3).contains(&range.1)
        || (range.0..=range.1).contains(&range.2)
        || (range.0..=range.1).contains(&range.3)
}

fn main() {
    let stdin = std::io::stdin();
    let mut score = 0;
    for line in stdin.lines() {
        match line {
            Ok(val) => {
                if overlaps(&parse_line(&val)) {
                    score += 1;
                }
            }
            Err(_) => panic!("something went wrong!"),
        }
    }
    println!("{}", score)
}
