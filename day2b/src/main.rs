#[derive(Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
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

fn map_letter_object(letter: &str) -> Result<Rps, ()> {
    match letter {
        "A" => Ok(Rps::Rock),
        "B" => Ok(Rps::Paper),
        "C" => Ok(Rps::Scissors),
        _ => Err(()),
    }
}

fn map_letter_result(letter: &str) -> Result<GameResult, ()> {
    match letter {
        "X" => Ok(GameResult::Loss),
        "Y" => Ok(GameResult::Draw),
        "Z" => Ok(GameResult::Win),
        _ => Err(()),
    }
}

fn get_object(opponent_object: Rps, result: GameResult) -> Rps {
    match opponent_object {
        Rps::Paper => match result {
            GameResult::Win => Rps::Scissors,
            GameResult::Draw => Rps::Paper,
            GameResult::Loss => Rps::Rock,
        },
        Rps::Rock => match result {
            GameResult::Win => Rps::Paper,
            GameResult::Draw => Rps::Rock,
            GameResult::Loss => Rps::Scissors,
        },
        Rps::Scissors => match result {
            GameResult::Win => Rps::Rock,
            GameResult::Draw => Rps::Scissors,
            GameResult::Loss => Rps::Paper,
        },
    }
}

fn score_round(player: Rps, res: GameResult) -> i32 {
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
    let opponent = map_letter_object(letters.next().unwrap()).unwrap();
    let result = map_letter_result(letters.next().unwrap()).unwrap();
    (opponent, get_object(opponent, result))
}

fn main() {
    let stdin = std::io::stdin();

    let mut score = 0;

    for line in stdin.lines() {
        match line {
            Ok(val) => {
                let round = map_line(&val);
                score += score_round(round.1, play_round(&round));
            }
            Err(_) => (),
        }
    }
    println!("{}", score)
}
