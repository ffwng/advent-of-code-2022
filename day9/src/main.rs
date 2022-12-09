use std::collections::HashSet;

struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    pub fn move_head(&mut self, dx: i32, dy: i32) {
        self.knots[0].0 += dx;
        self.knots[0].1 += dy;

        for i in 1..self.knots.len() {
            let front = self.knots[i - 1];
            let mut back = self.knots[i];

            let diff = (front.0 - back.0).pow(2) + (front.1 - back.1).pow(2);
            if diff > 2 {
                back.0 += (front.0 - back.0).signum();
                back.1 += (front.1 - back.1).signum();
            }

            self.knots[i] = back;
        }
    }

    pub fn tail(&self) -> (i32, i32) {
        self.knots[self.knots.len() - 1]
    }
}

struct State {
    rope: Rope,
    visited: HashSet<(i32, i32)>,
}

impl State {
    pub fn new(knots: Vec<(i32, i32)>) -> Self {
        let visited = [knots[knots.len() - 1]].into();
        Self {
            rope: Rope { knots },
            visited,
        }
    }

    pub fn move_head(&mut self, dx: i32, dy: i32) {
        self.rope.move_head(dx, dy);
        self.visited.insert(self.rope.tail());
    }

    pub fn tail_visited(&self) -> usize {
        self.visited.len()
    }
}

fn run(state: &mut State, input: &str) {
    for line in input.lines() {
        if let Some((dir, amount)) = line.split_once(" ") {
            if let Ok(amount) = amount.parse() {
                for _ in 0..amount {
                    let (dx, dy) = match dir {
                        "D" => (0, -1),
                        "R" => (1, 0),
                        "U" => (0, 1),
                        "L" => (-1, 0),
                        _ => (0, 0),
                    };

                    state.move_head(dx, dy);
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let mut state1 = State::new(vec![(0, 0), (0, 0)]);
    run(&mut state1, input);
    println!("{}", state1.tail_visited());

    let mut state2 = State::new(std::iter::repeat((0, 0)).take(10).collect());
    run(&mut state2, input);
    println!("{}", state2.tail_visited());
}
