type Pair = (i32, i32);

fn parse_pair(pair: &str) -> Option<Pair> {
    let (l, u) = pair.split_once('-')?;
    Some((l.parse().ok()?, u.parse().ok()?))
}

fn parse_line(line: &str) -> Option<(Pair, Pair)> {
    let (p1, p2) = line.split_once(',')?;
    Some((parse_pair(p1)?, parse_pair(p2)?))
}

fn contains(p: i32, l: i32, u: i32) -> bool {
    p >= l && p <= u
}

fn main() {
    let input = include_str!("../input");

    let count1 = input
        .lines()
        .filter(|line| {
            if let Some((p1, p2)) = parse_line(line) {
                p1.0 >= p2.0 && p1.1 <= p2.1 || p1.0 <= p2.0 && p1.1 >= p2.1
            } else {
                false
            }
        })
        .count();

    println!("{}", count1);

    let count2 = input
        .lines()
        .filter(|line| {
            if let Some((p1, p2)) = parse_line(line) {
                contains(p1.0, p2.0, p2.1) || contains(p2.0, p1.0, p1.1)
            } else {
                false
            }
        })
        .count();

    println!("{}", count2);
}
