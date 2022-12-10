struct State {
    cycle: u32,
    xreg: i32,
    signal_strength: i32,
}

impl State {
    pub fn new() -> Self {
        Self {
            cycle: 1,
            xreg: 1,
            signal_strength: 0,
        }
    }

    pub fn noop(&mut self) {
        self.next_cycle();
    }

    pub fn addx(&mut self, val: i32) {
        self.next_cycle();
        self.next_cycle();
        self.xreg += val;
    }

    pub fn signal_strength(&self) -> i32 {
        // assume that if the last instruction ended right before a critical cycle, this next cycle is not executed and
        // thus does not contribute to the signal strength (in particular, if the last instruction was addx, the new X
        // value does not matter)
        self.signal_strength
    }

    fn next_cycle(&mut self) {
        if self.cycle > 1 && self.cycle % 40 == 1 {
            println!();
        }

        if ((self.cycle as i32 - 1) % 40 - self.xreg).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if (self.cycle + 20) % 40 == 0 {
            self.signal_strength += self.cycle as i32 * self.xreg;
        }

        self.cycle += 1;
    }
}

fn main() {
    let input = include_str!("../input");

    let mut state = State::new();
    for line in input.lines() {
        if line == "noop" {
            state.noop();
        } else {
            if let Some(("addx", val)) = line.split_once(' ') {
                if let Ok(val) = val.parse() {
                    state.addx(val);
                }
            }
        }
    }

    println!("\n{}", state.signal_strength());
}
