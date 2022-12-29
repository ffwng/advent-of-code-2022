use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

type Pos = (i32, i32, i32);

fn neighbors((x, y, z): Pos) -> impl Iterator<Item = Pos> {
    [-1, 1]
        .into_iter()
        .flat_map(move |d| [(x + d, y, z), (x, y + d, z), (x, y, z + d)])
}

fn fill_steam(seen_cubes: &mut HashSet<Pos>, max_dim: i32) {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    let valid = |n| n >= 0 && n <= max_dim + 1;

    while let Some(p) = queue.pop_front() {
        if valid(p.0) && valid(p.1) && valid(p.2) && !seen_cubes.contains(&p) {
            seen_cubes.insert(p);
            queue.extend(neighbors(p));
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let mut max_dim = 0;
    let mut seen_cubes = HashSet::new();
    let mut surface_area = 0;

    for line in input.lines() {
        let pos: Pos = line
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        max_dim = max_dim.max(pos.0).max(pos.1).max(pos.2);

        seen_cubes.insert(pos);
        for neighbor in neighbors(pos) {
            if seen_cubes.contains(&neighbor) {
                // the surface of the other cube is now covered
                surface_area -= 1;
            } else {
                // the surface of the cube is (currently) not covered
                surface_area += 1;
            }
        }
    }

    println!("{surface_area}");

    fill_steam(&mut seen_cubes, max_dim);

    let mut inside_surface_area = 0;
    for x in 0..=max_dim {
        for y in 0..=max_dim {
            for z in 0..=max_dim {
                let pos = (x, y, z);
                if !seen_cubes.contains(&pos) {
                    for neighbor in neighbors(pos) {
                        if seen_cubes.contains(&neighbor) {
                            inside_surface_area += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", surface_area - inside_surface_area);
}
