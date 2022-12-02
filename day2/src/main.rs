#[derive(Clone, Copy, PartialEq, Eq)]
enum Gesture {
    Rock,
    Paper,
    Scissors,
}

impl Gesture {
    pub fn scores(self) -> [u32; 3] {
        match self {
            Self::Rock => [3, 1, 2],
            Self::Paper => [1, 2, 3],
            Self::Scissors => [2, 3, 1],
        }
    }

    pub fn parse(c: char) -> Option<Self> {
        match c {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            _ => None,
        }
    }

    pub fn result_score(self, other: Self) -> u32 {
        match (self, other) {
            (a, b) if a == b => 3,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => 0,
            _ => 6,
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let mut score1 = 0;
    for line in input.lines() {
        let opponent = Gesture::parse(line.chars().nth(0).expect("line too short"))
            .expect("unexpected gesture");

        let own_gesture = line.chars().nth(2).unwrap();
        match own_gesture {
            'X' => score1 += Gesture::Rock.result_score(opponent) + 1,
            'Y' => score1 += Gesture::Paper.result_score(opponent) + 2,
            'Z' => score1 += Gesture::Scissors.result_score(opponent) + 3,
            _ => println!("unexpected outcome"),
        }
    }

    println!("{}", score1);

    let mut score2 = 0;
    for line in input.lines() {
        let opponent = Gesture::parse(line.chars().nth(0).expect("line too short"))
            .expect("unexpected gesture");
        let scores = opponent.scores();

        let outcome = line.chars().nth(2).unwrap();
        match outcome {
            'X' => score2 += scores[0],
            'Y' => score2 += scores[1] + 3,
            'Z' => score2 += scores[2] + 6,
            _ => println!("unexpected outcome"),
        }
    }

    println!("{}", score2);
}
