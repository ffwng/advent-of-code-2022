use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone)]
struct Map {
    blocked: HashMap<i32, Vec<i32>>,
}

impl Map {
    fn new() -> Self {
        Self {
            blocked: HashMap::new(),
        }
    }

    fn mark_blocked(&mut self, x: i32, y: i32) {
        let column = self.blocked.entry(x).or_insert(Vec::new());
        if let Err(pos) = column.binary_search(&y) {
            column.insert(pos, y);
        }
    }

    fn is_blocked(&self, x: i32, y: i32) -> bool {
        if let Some(column) = self.blocked.get(&x) {
            column.binary_search(&y).is_ok()
        } else {
            false
        }
    }

    fn next_drop_level(&self, x: i32, y: i32) -> Option<i32> {
        if let Some(column) = self.blocked.get(&x) {
            if let Err(pos) = column.binary_search(&y) {
                if pos < column.len() {
                    return Some(column[pos]);
                }
            }
        }

        None
    }
}

fn parse_point(point: &str) -> Option<(i32, i32)> {
    let (x, y) = point.split_once(',')?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

fn simulate_drop(map: &Map) -> Result<(i32, i32), i32> {
    let mut x = 500;
    let mut y = 0;
    while let Some(next_y) = map.next_drop_level(x, y) {
        y = next_y;
        if !map.is_blocked(x - 1, y) {
            x = x - 1;
        } else if !map.is_blocked(x + 1, y) {
            x = x + 1;
        } else {
            return Ok((x, y - 1));
        }
    }

    Err(x)
}

fn main() {
    let input = include_str!("../input");

    let mut map = Map::new();
    let mut y_max = 0;
    for line in input.lines() {
        for (p1, p2) in line.split(" -> ").tuple_windows() {
            let p1 = parse_point(p1).unwrap();
            let p2 = parse_point(p2).unwrap();

            y_max = y_max.max(p1.1).max(p2.1);

            if p1.0 == p2.0 {
                // vertical line
                for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                    map.mark_blocked(p1.0, y);
                }
            } else if p1.1 == p2.1 {
                // horizontal line
                for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
                    map.mark_blocked(x, p1.1);
                }
            } else {
                panic!("diagonal line from {:?} to {:?}", p1, p2);
            }
        }
    }

    let mut map1 = map.clone();
    let mut counter1 = 0;
    while let Ok((x, y)) = simulate_drop(&map1) {
        map1.mark_blocked(x, y);
        counter1 += 1;
    }

    println!("{counter1}");

    let mut map2 = map;
    let mut counter2 = 0;
    while !map2.is_blocked(500, 0) {
        match simulate_drop(&map2) {
            Ok((x, y)) => map2.mark_blocked(x, y),
            Err(x) => map2.mark_blocked(x, y_max + 1),
        }
        counter2 += 1;
    }

    println!("{counter2}");
}
