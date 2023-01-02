use std::collections::{HashMap, HashSet};

use num::Integer;
use petgraph::{algo::dijkstra, graph::DiGraph};

type Pos = (i32, i32);

fn neighbors_or_self((x, y): Pos, len_x: i32, len_y: i32) -> impl Iterator<Item = Pos> {
    [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(move |&(x, y)| x >= 0 && x < len_x && y >= 0 && y < len_y)
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn shift(&self, (x, y): Pos, t: i32) -> Pos {
        match self {
            Direction::North => (x, y - t),
            Direction::South => (x, y + t),
            Direction::West => (x - t, y),
            Direction::East => (x + t, y),
        }
    }

    fn parse(b: u8) -> Option<Self> {
        match b {
            b'^' => Some(Self::North),
            b'v' => Some(Self::South),
            b'<' => Some(Self::West),
            b'>' => Some(Self::East),
            _ => None,
        }
    }
}

struct Valley {
    blizzards: HashMap<Pos, Direction>,
    len_x: i32,
    len_y: i32,
}

impl Valley {
    fn parse(input: &str) -> Self {
        let mut blizzards = HashMap::new();
        let mut len_x = 0;
        let mut len_y = -2;
        for (y, line) in input.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                if let Some(d) = Direction::parse(b) {
                    blizzards.insert((x as i32 - 1, y as i32 - 1), d);
                }
            }

            len_x = line.len() as i32 - 2; // assume all lines have the same length
            len_y += 1;
        }

        Self {
            blizzards,
            len_x,
            len_y,
        }
    }

    fn occupied_map(&self, t: i32) -> HashSet<Pos> {
        let mut result = HashSet::new();
        for (&pos, d) in &self.blizzards {
            let (x, y) = d.shift(pos, t);
            result.insert((x.rem_euclid(self.len_x), y.rem_euclid(self.len_y)));
        }

        result
    }
}

fn shortest_path(valley: &Valley, start_time: i32, start_pos: Pos, end_pos: Pos) -> i32 {
    let len_x = valley.len_x;
    let len_y = valley.len_y;
    let len_t = len_x.lcm(&len_y);

    let mut graph = DiGraph::new();

    let start = graph.add_node(());
    let end = graph.add_node(());
    let mut nodes = HashMap::new();

    for t in 0..len_t {
        let occupied = valley.occupied_map(start_time + t);

        for x in 0..len_x {
            for y in 0..len_y {
                let pos = (x, y);
                if !occupied.contains(&pos) {
                    let node = graph.add_node(());
                    nodes.insert((pos, t), node);

                    for n in neighbors_or_self(pos, len_x, len_y) {
                        if let Some(&p) = nodes.get(&(n, (t - 1).rem_euclid(len_t))) {
                            graph.add_edge(p, node, 1);
                        }
                    }

                    if pos == start_pos {
                        graph.add_edge(start, node, t);
                    }

                    if pos == end_pos {
                        graph.add_edge(node, end, 1);
                    }
                }
            }
        }
    }

    let result = dijkstra(&graph, start, Some(end), |e| *e.weight());
    return *result.get(&end).unwrap();
}

fn main() {
    let input = include_str!("../input");
    let valley = Valley::parse(input);
    let start = (0, 0);
    let end = (valley.len_x - 1, valley.len_y - 1);

    let l1 = shortest_path(&valley, 0, start, end);
    println!("{:?}", l1);

    let l2 = shortest_path(&valley, l1, end, start);
    let l3 = shortest_path(&valley, l1 + l2, start, end);
    println!("{:?}", l1 + l2 + l3);
}
