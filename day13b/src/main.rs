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

    let mut score = 1;

    let divider_1 = serde_json::from_str::<Node>(&"[[2]]").unwrap();
    let divider_2 = serde_json::from_str::<Node>(&"[[6]]").unwrap();

    let mut packets = vec![divider_1.clone(), divider_2.clone()];


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
    }

    packets.sort();

    for (i, node) in packets.iter().enumerate() {
        if *node == divider_1 || *node == divider_2 {
            score *= i+1;
        }
    }
    
    println!("{}", score)
}
