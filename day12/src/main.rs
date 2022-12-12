use std::cmp::Reverse;
use std::collections::HashSet;

use priority_queue::PriorityQueue;

fn valid_step(start: u8, end: u8) -> bool {
    end <= start + 1
}

fn fewest_steps<'a>(
    grid: &'a [Vec<u8>],
    start: (usize, usize),
    end: (usize, usize),
    any_start: bool,
) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut queue = PriorityQueue::new();
    queue.push(start, Reverse(0));

    if any_start {
        for (y, line) in grid.iter().enumerate() {
            for (x, level) in line.iter().enumerate() {
                if *level == b'a' {
                    queue.push((x, y), Reverse(0));
                }
            }
        }
    }

    while let Some((pos @ (x, y), Reverse(distance))) = queue.pop() {
        if pos == end {
            return Some(distance);
        }

        let level = grid[y][x];
        let mut check = |next @ (x_next, y_next)| {
            let line: &'a Vec<u8> = &grid[y_next];
            if !visited.contains(&next) && valid_step(level, line[x_next]) {
                queue.push_increase(next, Reverse(distance + 1));
            }
        };

        if x > 0 {
            check((x - 1, y));
        }
        if x < grid[y].len() - 1 {
            check((x + 1, y));
        }
        if y > 0 {
            check((x, y - 1));
        }
        if y < grid.len() - 1 {
            check((x, y + 1));
        }

        visited.insert(pos);
    }

    None
}

fn main() {
    let input = include_bytes!("../input");

    let (grid, start, end) = {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let grid: Vec<Vec<u8>> = input
            .split(|&b| b == b'\n')
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, level)| match *level {
                        b'S' => {
                            start = (x, y);
                            b'a'
                        }
                        b'E' => {
                            end = (x, y);
                            b'z'
                        }
                        level => level,
                    })
                    .collect()
            })
            .collect();

        (grid, start, end)
    };

    println!("{:?}", fewest_steps(&grid, start, end, false));
    println!("{:?}", fewest_steps(&grid, start, end, true));
}
