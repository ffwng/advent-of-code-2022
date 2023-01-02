use std::collections::HashMap;

use regex::Regex;

type Pos = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn right(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    fn facing(self) -> usize {
        match self {
            Self::Up => 3,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 0,
        }
    }

    fn opposite(self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

struct Row {
    offset: usize,
    tiles: Vec<Tile>,
}

impl Row {
    fn contains_x(&self, x: usize) -> bool {
        x >= self.offset && x < self.offset + self.tiles.len()
    }

    fn tile(&self, x: usize) -> Option<Tile> {
        if self.contains_x(x) {
            Some(self.tiles[x - self.offset])
        } else {
            None
        }
    }
}

struct Board {
    map: Vec<Row>,
}

impl Board {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn add_row(&mut self, line: &str) {
        let mut offset = 0;
        let tiles = line
            .bytes()
            .skip_while(|&c| {
                if c == b' ' {
                    offset += 1;
                    true
                } else {
                    false
                }
            })
            .take_while(|&c| c != b' ')
            .map(|c| if c == b'.' { Tile::Open } else { Tile::Wall })
            .collect();

        self.map.push(Row { offset, tiles });
    }

    fn next_pos(&self, pos: Pos, dir: Direction) -> Pos {
        let (mut x, mut y) = pos;

        match dir {
            Direction::Up => {
                if y == 0 || !self.map[y - 1].contains_x(x) {
                    // go down to find the matching row to wrap around
                    while y < self.map.len() - 1 && self.map[y + 1].contains_x(x) {
                        y += 1;
                    }
                } else {
                    y = (self.map.len() + y - 1) % self.map.len();
                }
            }
            Direction::Down => {
                if y == self.map.len() - 1 || !self.map[y + 1].contains_x(x) {
                    // go up to find the matching row to wrap around
                    while y > 0 && self.map[y - 1].contains_x(x) {
                        y -= 1;
                    }
                } else {
                    y = (y + 1) % self.map.len();
                }
            }
            Direction::Left => {
                let row = &self.map[y];
                x = if x == 0 || !row.contains_x(x - 1) {
                    row.offset + row.tiles.len() - 1
                } else {
                    x - 1
                };
            }
            Direction::Right => {
                let row = &self.map[y];
                x = if !row.contains_x(x + 1) {
                    row.offset
                } else {
                    x + 1
                };
            }
        };

        // unwrap, because x and y should always be valid at this point
        let tile = self.map[y].tile(x).unwrap();
        if tile == Tile::Wall {
            pos
        } else {
            (x, y)
        }
    }
}

struct CubeBoard {
    sides: [Vec<Vec<Tile>>; 6],
    foldings: HashMap<(usize, Direction), (usize, Direction)>,
}

impl CubeBoard {
    fn new(foldings: HashMap<(usize, Direction), (usize, Direction)>) -> Self {
        Self {
            sides: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
            foldings,
        }
    }

    fn add_row(&mut self, line: &str, side: usize) {
        self.sides[side - 1].push(
            line.bytes()
                .map(|c| if c == b'.' { Tile::Open } else { Tile::Wall })
                .collect(),
        );
    }

    fn next_pos(&self, start_side: usize, pos: Pos, dir: Direction) -> (usize, Pos, Direction) {
        let mut side = start_side;
        let (mut x, mut y) = pos;
        let mut wrap = false;
        let mut new_dir = dir;

        match dir {
            Direction::Up => {
                if y == 0 {
                    wrap = true;
                } else {
                    y -= 1;
                }
            }
            Direction::Down => {
                if y == self.sides[side - 1].len() - 1 {
                    wrap = true;
                } else {
                    y += 1;
                }
            }
            Direction::Left => {
                if x == 0 {
                    wrap = true;
                } else {
                    x -= 1;
                }
            }
            Direction::Right => {
                if x == self.sides[side - 1][y].len() - 1 {
                    wrap = true;
                } else {
                    x += 1;
                }
            }
        };

        if wrap {
            let entry_dir;
            (side, entry_dir) = self.foldings[&(start_side, dir)];
            (x, y) = self.side_entry(side, (x, y), dir, entry_dir);
            new_dir = entry_dir.opposite();
        }

        // unwrap, because x and y should always be valid at this point
        let tile = self.sides[side - 1][y][x];
        if tile == Tile::Wall {
            (start_side, pos, dir)
        } else {
            (side, (x, y), new_dir)
        }
    }

    fn side_entry(&self, side: usize, (x, y): Pos, from_dir: Direction, to_dir: Direction) -> Pos {
        use Direction::*;

        // assume all sides are squares of the same size
        let size = self.sides[side - 1].len() - 1;
        match (from_dir, to_dir) {
            (Up, Up) => (size - x, 0),
            (Up, Down) => (x, size),
            (Up, Left) => (0, x),
            (Up, Right) => (size, size - x),
            (Down, Up) => (x, 0),
            (Down, Down) => (size - x, size),
            (Down, Left) => (0, size - x),
            (Down, Right) => (size, x),
            (Left, Up) => (y, 0),
            (Left, Down) => (size - y, size),
            (Left, Left) => (0, size - y),
            (Left, Right) => (size, y),
            (Right, Up) => (size - y, 0),
            (Right, Down) => (y, size),
            (Right, Left) => (0, y),
            (Right, Right) => (size, size - y),
        }
    }
}

fn get_target_pos(board: &Board, path: &str) -> (Pos, Direction) {
    let re = Regex::new(r"R|L|\d+").unwrap();

    let mut pos = (board.map[0].offset, 0);
    let mut dir = Direction::Right;

    for part in re.find_iter(path) {
        match part.as_str() {
            "R" => dir = dir.right(),
            "L" => dir = dir.left(),
            n => {
                for _ in 0..n.parse::<u32>().unwrap() {
                    pos = board.next_pos(pos, dir);
                }
            }
        }
    }

    (pos, dir)
}

fn get_cube_target_pos(board: &CubeBoard, path: &str) -> (usize, Pos, Direction) {
    let re = Regex::new(r"R|L|\d+").unwrap();

    let mut side = 1;
    let mut pos = (0, 0);
    let mut dir = Direction::Right;

    for part in re.find_iter(path) {
        match part.as_str() {
            "R" => dir = dir.right(),
            "L" => dir = dir.left(),
            n => {
                for _ in 0..n.parse::<u32>().unwrap() {
                    (side, pos, dir) = board.next_pos(side, pos, dir);
                }
            }
        }
    }

    (side, pos, dir)
}

fn main() {
    let input = include_str!("../input");

    let mut board = Board::new();
    let mut lines = input.lines();
    for line in &mut lines {
        if line == "" {
            break; // next line will be the path
        }

        board.add_row(line);
    }

    let path = lines.next().unwrap();
    let ((column, row), dir) = get_target_pos(&board, path);

    println!("{:?}", (1000 * (row + 1) + 4 * (column + 1) + dir.facing()));

    use Direction::*;
    let mut board = CubeBoard::new(
        [
            ((1, Up), (6, Left)),
            ((1, Down), (3, Up)),
            ((1, Left), (4, Left)),
            ((1, Right), (2, Left)),
            //
            ((2, Up), (6, Down)),
            ((2, Down), (3, Right)),
            ((2, Left), (1, Right)),
            ((2, Right), (5, Right)),
            //
            ((3, Up), (1, Down)),
            ((3, Down), (5, Up)),
            ((3, Left), (4, Up)),
            ((3, Right), (2, Down)),
            //
            ((4, Up), (3, Left)),
            ((4, Down), (6, Up)),
            ((4, Left), (1, Left)),
            ((4, Right), (5, Left)),
            //
            ((5, Up), (3, Down)),
            ((5, Down), (6, Right)),
            ((5, Left), (4, Right)),
            ((5, Right), (2, Right)),
            //
            ((6, Up), (4, Down)),
            ((6, Down), (2, Up)),
            ((6, Left), (1, Up)),
            ((6, Right), (5, Down)),
        ]
        .into_iter()
        .collect(),
    );

    let mut side = 1;
    let mut sides = 0;
    for (row, line) in input.lines().enumerate() {
        if line == "" {
            break;
        }

        if row % 50 == 0 {
            side += sides;
        }

        let pos = line.bytes().position(|c| c != b' ').unwrap();
        sides = (line.len() - pos) / 50;
        for i in 0..sides {
            let row = &line[pos + i * 50..pos + (i + 1) * 50];
            board.add_row(row, side + i);
        }
    }

    let (side, (column, row), dir) = get_cube_target_pos(&board, path);
    let offsets = [(50, 0), (100, 0), (50, 50), (0, 100), (50, 100), (0, 150)];
    let row = offsets[side - 1].1 + row + 1;
    let column = offsets[side - 1].0 + column + 1;

    println!("{:?}", (1000 * row + 4 * column + dir.facing()));
}
