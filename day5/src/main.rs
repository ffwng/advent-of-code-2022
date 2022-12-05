struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    pub fn parse(line: &str) -> Option<Self> {
        let mut words = line.split_ascii_whitespace();
        let count = words.nth(1)?.parse().ok()?;
        let from = words.nth(1)?.parse().ok()?;
        let to = words.nth(1)?.parse().ok()?;

        Some(Self { count, from, to })
    }
}

fn main() {
    let mut stacks1 = [
        b"RGHQSBTN".to_vec(),
        b"HSFDPZJ".to_vec(),
        b"ZHV".to_vec(),
        b"MZJFGH".to_vec(),
        b"TZCDLMSR".to_vec(),
        b"MTWVHZJ".to_vec(),
        b"TFPLZ".to_vec(),
        b"QVWS".to_vec(),
        b"WHLMTDNC".to_vec(),
    ];

    let input = include_str!("../input");

    for line in input.lines() {
        if let Some(Move { count, from, to }) = Move::parse(line) {
            for _ in 0..count {
                if let Some(last) = stacks1[from - 1].pop() {
                    stacks1[to - 1].push(last)
                }
            }
        }
    }

    let top1 = String::from_utf8(
        stacks1
            .iter()
            .filter_map(|s| s.last().cloned())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    println!("{}", top1);

    let mut stacks2 = [
        b"RGHQSBTN".to_vec(),
        b"HSFDPZJ".to_vec(),
        b"ZHV".to_vec(),
        b"MZJFGH".to_vec(),
        b"TZCDLMSR".to_vec(),
        b"MTWVHZJ".to_vec(),
        b"TFPLZ".to_vec(),
        b"QVWS".to_vec(),
        b"WHLMTDNC".to_vec(),
    ];

    for line in input.lines() {
        if let Some(Move { count, from, to }) = Move::parse(line) {
            let mut removed = stacks2[from - 1]
                .drain(stacks2[from - 1].len() - count..)
                .collect();
            stacks2[to - 1].append(&mut removed);
        }
    }

    let top2 = String::from_utf8(
        stacks2
            .iter()
            .filter_map(|s| s.last().cloned())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    println!("{}", top2);
}
