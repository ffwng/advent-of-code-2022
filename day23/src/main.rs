use std::collections::{hash_map::Entry, HashMap, HashSet};

type Pos = (isize, isize);

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn offsets(&self, (x, y): Pos) -> [Pos; 3] {
        match self {
            Direction::North => [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
            Direction::South => [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
            Direction::West => [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
            Direction::East => [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
        }
    }

    fn shift(&self, (x, y): Pos) -> Pos {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }
}

fn spread(elves: &mut HashSet<Pos>, directions: &[Direction]) -> bool {
    let mut proposed = HashMap::new();

    for &pos in elves.iter() {
        if directions.iter().all(|d| {
            d.offsets(pos)
                .iter()
                .all(|check_pos| !elves.contains(check_pos))
        }) {
            // the elf contains no neighbors, so doesn’t move at all
            continue;
        }

        for &d in directions {
            if d.offsets(pos)
                .iter()
                .all(|check_pos| !elves.contains(check_pos))
            {
                let target_pos = d.shift(pos);
                match proposed.entry(target_pos) {
                    Entry::Occupied(mut e) => {
                        // mark the position as proposed by multiple elves
                        e.insert(None);
                    }
                    Entry::Vacant(e) => {
                        e.insert(Some(pos));
                    }
                }

                break;
            }
        }
    }

    let mut changed = false;
    for (target_pos, source_pos) in proposed {
        if let Some(source_pos) = source_pos {
            elves.remove(&source_pos);
            elves.insert(target_pos);
            changed = true;
        }
    }

    changed
}

fn main() {
    let input = include_str!("../input");

    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    for y in -3..11 {
        for x in -3..11 {
            print!("{}", if elves.contains(&(x, y)) { '#' } else { '.' })
        }
        println!();
    }
    println!();

    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let mut cnt = 0;
    while spread(&mut elves, &directions) {
        directions.rotate_left(1);
        cnt += 1;

        if cnt == 10 {
            // assume that there are elves in the first column and row
            let mut min_x = 0;
            let mut max_x = 0;
            let mut min_y = 0;
            let mut max_y = 0;
            for &(x, y) in &elves {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }

            println!(
                "{}",
                (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as isize
            )
        }
    }

    println!("{}", cnt + 1);
}
