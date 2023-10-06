use std::collections::VecDeque;

use text_io::read;
fn main() {
    let mut max_cal = 0;
    let mut current_cal = 0;
    let mut read_line: String;
    let mut top_vec = VecDeque::new();
    loop {
        read_line = read!("{}\n");

        match read_line.as_str() {
            "" => {
                if max_cal < current_cal {
                    max_cal = current_cal;
                }
                top_vec.push_front(current_cal);
                current_cal = 0;
            }
            "end" => break,
            _ => {
                let cal = read_line.parse::<i32>();
                match cal {
                    Ok(x) => current_cal += x,
                    Err(_) => (),
                }
            }
        }
    }

    // First task
    println!("{}", max_cal);

    // Second task
    top_vec.make_contiguous().sort_by(|a, b| b.cmp(a));
    println!("{}", top_vec.range(..3).sum::<i32>());
}
