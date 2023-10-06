enum Rps {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Loss,
    Draw,
    Win,
}

fn play_round(round: &(Rps, Rps)) -> GameResult {
    match round.1 {
        Rps::Rock => match round.0 {
            Rps::Paper => GameResult::Loss,
            Rps::Scissors => GameResult::Win,
            _ => GameResult::Draw,
        },
        Rps::Paper => match round.0 {
            Rps::Scissors => GameResult::Loss,
            Rps::Rock => GameResult::Win,
            _ => GameResult::Draw,
        },
        Rps::Scissors => match round.0 {
            Rps::Paper => GameResult::Win,
            Rps::Rock => GameResult::Loss,
            _ => GameResult::Draw,
        },
    }
}

fn map_letter(letter: &str) -> Result<Rps, ()> {
    match letter {
        "A" | "X" => Ok(Rps::Rock),
        "B" | "Y" => Ok(Rps::Paper),
        "C" | "Z" => Ok(Rps::Scissors),
        _ => Err(()),
    }
}

fn score_round(player: &Rps, res: GameResult) -> i32 {
    let mut score = 0;

    match player {
        Rps::Rock => score += 1,
        Rps::Paper => score += 2,
        Rps::Scissors => score += 3,
    }

    match res {
        GameResult::Loss => score += 0,
        GameResult::Draw => score += 3,
        GameResult::Win => score += 6,
    }
    score
}

fn map_line(line: &str) -> (Rps, Rps) {
    let mut letters = line.trim().split(' ');
    (
        map_letter(letters.next().unwrap()).unwrap(),
        map_letter(letters.next().unwrap()).unwrap(),
    )
}

fn main() {
    let stdin = std::io::stdin();

    let mut score = 0;

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                let round = map_line(&val);
                score += score_round(&round.1, play_round(&round));
            }
            Err(_) => (),
        }
    }
    println!("{}", score)
}
