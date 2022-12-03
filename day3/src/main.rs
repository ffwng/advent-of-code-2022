#![feature(iter_array_chunks)]

fn priority(item: u8) -> Option<u32> {
    match item {
        b'a'..=b'z' => Some((item - b'a' + 1) as u32),
        b'A'..=b'Z' => Some((item - b'A' + 27) as u32),
        _ => None,
    }
}

fn main() {
    let input = include_bytes!("../input");

    let mut sum1 = 0;
    for rucksack in input.split(|&b| b == b'\n') {
        let (c1, c2) = rucksack.split_at(rucksack.len() / 2);

        if let Some(&common) = c1.iter().find(|b| c2.contains(b)) {
            if let Some(p) = priority(common) {
                sum1 += p;
            }
        }
    }

    println!("{}", sum1);

    let mut sum2 = 0;
    for [r1, r2, r3] in input.split(|&b| b == b'\n').array_chunks() {
        if let Some(&common) = r1.iter().find(|b| r2.contains(b) && r3.contains(b)) {
            if let Some(p) = priority(common) {
                sum2 += p;
            }
        }
    }

    println!("{}", sum2);
}
