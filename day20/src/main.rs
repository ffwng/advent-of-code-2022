fn mix(numbers: &[i64], rounds: usize) -> Vec<i64> {
    let mut result: Vec<(usize, i64)> = numbers.iter().cloned().enumerate().collect();
    let len = result.len() as i64;

    let mov = |result: &mut Vec<(usize, i64)>, pos: usize, offset: i64| {
        let mut target = (pos as i64 + offset).rem_euclid(len - 1) as usize;
        if target == 0 {
            target = len as usize - 1;
        }

        let elem = result.remove(pos);
        result.insert(target, elem);
    };

    for _ in 0..rounds {
        for (i, &n) in numbers.iter().enumerate() {
            let pos = result.iter().position(|&(p, _)| p == i).unwrap();
            mov(&mut result, pos, n);
        }
    }

    result.into_iter().map(|(_, n)| n).collect()
}

fn main() {
    let input = include_str!("../input");

    let numbers: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mixed = mix(&numbers, 1);
    let pos0 = mixed.iter().position(|&n| n == 0).unwrap();

    println!(
        "{}",
        mixed[(pos0 + 1000) % mixed.len()]
            + mixed[(pos0 + 2000) % mixed.len()]
            + mixed[(pos0 + 3000) % mixed.len()]
    );

    let numbers: Vec<i64> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * 811589153)
        .collect();
    let mixed = mix(&numbers, 10);
    let pos0 = mixed.iter().position(|&n| n == 0).unwrap();

    println!(
        "{}",
        mixed[(pos0 + 1000) % mixed.len()]
            + mixed[(pos0 + 2000) % mixed.len()]
            + mixed[(pos0 + 3000) % mixed.len()]
    );
}
