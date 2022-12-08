use std::collections::HashMap;

enum Item {
    File {
        size: u32,
    },
    Dir {
        content: HashMap<&'static str, Item>,
    },
}

impl Item {
    fn root() -> Self {
        Self::Dir {
            content: HashMap::new(),
        }
    }

    fn get_item(&mut self, mut path: &[&str]) -> Option<&mut Self> {
        let mut current = self;
        while path.len() > 0 {
            match current {
                Self::File { .. } => return None,
                Self::Dir { content } => {
                    let name = path[0];
                    path = &path[1..];

                    if let Some(next) = content.get_mut(name) {
                        current = next;
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(current)
    }

    fn add_file(&mut self, name: &'static str, size: u32) {
        if let Self::Dir { content } = self {
            content.insert(name, Self::File { size });
        }
    }

    fn add_dir(&mut self, name: &'static str) {
        if let Self::Dir { content } = self {
            content.insert(
                name,
                Self::Dir {
                    content: HashMap::new(),
                },
            );
        }
    }

    fn reduce_size<F: FnMut(u32)>(&self, cb: &mut F) -> u32 {
        match self {
            &Self::File { size } => size,
            Self::Dir { content } => {
                let size = content.values().map(|i| i.reduce_size(cb)).sum();
                cb(size);
                size
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let mut root = Item::root();
    let mut path = Vec::new();

    // skip the first “cd /” line, because we are already at the root
    for line in input.lines().skip(1) {
        if line == "$ cd .." {
            path.pop();
        } else if line.starts_with("$ cd ") {
            path.push(&line[5..])
        } else if line.starts_with("dir ") {
            if let Some(item) = root.get_item(&path) {
                item.add_dir(&line[4..]);
            }
        } else if !line.starts_with("$") {
            // ignore “$ ls”
            if let Some((size, name)) = line.split_once(' ') {
                if let Ok(size) = size.parse() {
                    if let Some(item) = root.get_item(&path) {
                        item.add_file(name, size)
                    }
                }
            }
        }
    }

    let mut sum = 0;
    let total_size = root.reduce_size(&mut |size| {
        if size <= 100_000 {
            sum += size;
        }
    });

    println!("{}", sum);

    let mut min = total_size;
    root.reduce_size(&mut |size| {
        if size + 40000000 >= total_size && size < min {
            min = size;
        }
    });

    println!("{}", min);
}
