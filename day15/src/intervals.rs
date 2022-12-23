#[derive(Debug)]
pub struct Interval {
    from: i32,
    to: i32,
}

impl Interval {
    pub fn new(from: i32, to: i32) -> Self {
        Self { from, to }
    }

    pub fn empty() -> Self {
        Self { from: 1, to: 0 }
    }

    fn clip(self, other: &Self) -> Vec<Self> {
        if self.from >= other.from && self.to <= other.to {
            // other encloses self, clip is empty
            vec![]
        } else if other.from >= self.from && other.to <= self.to {
            // self encloses other, clip consist of the part before and after other
            vec![
                Self {
                    from: self.from,
                    to: other.from - 1,
                },
                Self {
                    from: other.to + 1,
                    to: self.to,
                },
            ]
        } else if self.from >= other.from && self.from <= other.to {
            // overlap on right side, clip consist of the part after other
            vec![Self {
                from: other.to + 1,
                to: self.to,
            }]
        } else if self.to <= other.to && self.to >= other.from {
            // overlap on left side, clip consist of the part before other
            vec![Self {
                from: self.from,
                to: other.from - 1,
            }]
        } else {
            // no overlap
            vec![self]
        }
    }

    fn is_empty(&self) -> bool {
        self.from > self.to
    }

    fn area(&self) -> i32 {
        if self.is_empty() {
            0
        } else {
            self.to - self.from + 1
        }
    }
}

pub struct IntervalSet {
    areas: Vec<Interval>,
}

impl IntervalSet {
    pub fn new() -> Self {
        Self { areas: Vec::new() }
    }

    pub fn insert(&mut self, interval: Interval) {
        let new_areas = self
            .areas
            .iter()
            .fold(vec![interval], |acc, elem| {
                acc.into_iter().flat_map(|a| a.clip(elem)).collect()
            })
            .into_iter()
            .filter(|a| !a.is_empty());

        self.areas.extend(new_areas);
    }

    pub fn area(&self) -> i32 {
        self.areas.iter().map(Interval::area).sum()
    }
}
