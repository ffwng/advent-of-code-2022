use std::collections::{hash_map::Entry, HashMap};

use bitset_core::BitSet;

struct RockColumn {
    offset: usize,
    height: usize,
}

type Rock = &'static [RockColumn];

const ROCKS: &'static [Rock] = &[
    &[
        RockColumn {
            offset: 0,
            height: 1,
        },
        RockColumn {
            offset: 0,
            height: 1,
        },
        RockColumn {
            offset: 0,
            height: 1,
        },
        RockColumn {
            offset: 0,
            height: 1,
        },
    ],
    &[
        RockColumn {
            offset: 1,
            height: 1,
        },
        RockColumn {
            offset: 0,
            height: 3,
        },
        RockColumn {
            offset: 1,
            height: 1,
        },
    ],
    &[
        RockColumn {
            offset: 0,
            height: 1,
        },
        RockColumn {
            offset: 0,
            height: 1,
        },
        RockColumn {
            offset: 0,
            height: 3,
        },
    ],
    &[RockColumn {
        offset: 0,
        height: 4,
    }],
    &[
        RockColumn {
            offset: 0,
            height: 2,
        },
        RockColumn {
            offset: 0,
            height: 2,
        },
    ],
];

#[derive(PartialEq, Eq, Hash)]
struct CycleKey {
    rock_index: usize,
    movement_index: usize,
}

struct CycleValue {
    rock_count: usize,
    height: usize,
}

#[derive(Debug)]
struct Cycle {
    length: usize,
    height: usize,
}

struct Map {
    rock_count: usize,
    filled: Vec<u8>,
    max_height: usize,
    cycle_candidates: HashMap<CycleKey, CycleValue>,
}

impl Map {
    fn new() -> Self {
        Self {
            rock_count: 0,
            filled: Vec::new(),
            max_height: 0,
            cycle_candidates: HashMap::new(),
        }
    }

    fn place(
        &mut self,
        rock: Rock,
        rock_index: usize,
        mut movement: impl Iterator<Item = (usize, isize)>,
    ) -> Option<Cycle> {
        let mut left = 2usize;
        let mut height = self.max_height + 3;
        let mut movement_index;

        loop {
            let (idx, step) = movement.next().unwrap();
            movement_index = idx;

            if let Some(new_left) = left.checked_add_signed(step) {
                if new_left + rock.len() <= 7 && self.is_valid(rock, new_left, height) {
                    left = new_left;
                }
            }

            if height > 0 && self.is_valid(rock, left, height - 1) {
                height -= 1;
            } else {
                break;
            }
        }

        self.rock_count += 1;

        let mut result = None;
        for (i, column) in rock.iter().enumerate() {
            let base_height = height + column.offset;
            for h in 0..column.height {
                let cycle = self.fill(left + i, base_height + h, rock_index, movement_index);
                result = result.or(cycle);
            }
        }

        result
    }

    fn is_valid(&self, rock: Rock, left: usize, height: usize) -> bool {
        rock.iter().enumerate().all(|(i, column)| {
            (0..column.height).all(|h| !self.is_filled(left + i, height + column.offset + h))
        })
    }

    fn fill(
        &mut self,
        left: usize,
        height: usize,
        rock_index: usize,
        movement_index: usize,
    ) -> Option<Cycle> {
        if height >= self.filled.len() {
            self.filled.resize(height + 1, 0);
        }

        self.filled[height].bit_set(left);
        self.max_height = self.max_height.max(height + 1);

        if self.filled[height] == 0b1111111 {
            let entry = self.cycle_candidates.entry(CycleKey {
                rock_index,
                movement_index,
            });

            match entry {
                Entry::Occupied(e) => {
                    let prev = e.get();
                    return Some(Cycle {
                        length: self.rock_count - prev.rock_count,
                        height: height - prev.height,
                    });
                }
                Entry::Vacant(e) => {
                    e.insert(CycleValue {
                        height,
                        rock_count: self.rock_count,
                    });
                }
            }
        }

        None
    }

    fn is_filled(&self, left: usize, height: usize) -> bool {
        self.filled
            .get(height)
            .map_or(false, |row| row.bit_test(left))
    }
}

fn solve(input: &[u8], steps: usize) {
    let mut movement = input
        .iter()
        .map(|&b| {
            if b == b'<' {
                -1
            } else if b == b'>' {
                1
            } else {
                panic!("unexpected byte {b}")
            }
        })
        .enumerate()
        .cycle();

    let mut map = Map::new();

    let mut remaining = steps;
    let mut cycle_height = 0;
    for (idx, rock) in ROCKS.iter().enumerate().cycle() {
        if remaining == 0 {
            break;
        }

        if let Some(cycle) = map.place(rock, idx, &mut movement) {
            cycle_height += cycle.height * (remaining / cycle.length);
            remaining = remaining % cycle.length;
        }

        remaining -= 1;
    }

    println!("{}", cycle_height + map.max_height);
}

fn main() {
    let input = include_bytes!("../input");
    solve(input, 2022);
    solve(input, 1000000000000);
}
