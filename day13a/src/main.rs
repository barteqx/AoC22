use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    List(Vec<Node>),
    Number(i32),
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(l), Node::Number(r)) => l.partial_cmp(r),
            (l, r) => l.with_slice(|l| r.with_slice(|r| l.partial_cmp(r))),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut score = 0;

    let mut i = 1;

    let mut packets = vec![];


    for line in stdin.lines() {
        match line {
            Ok(val) => {
                if val.is_empty() {
                    continue;
                }
                let packet = serde_json::from_str::<Node>(&val).unwrap();
                packets.push(packet);
            }
            Err(_) => (),
        }

        if packets.len() == 2 {
            if packets[0] < packets[1] {
                score += i;
            }
            packets = vec![];
            i += 1;

        }
    }
    println!("{}", score)
}
