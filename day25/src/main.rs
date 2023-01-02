fn from_snafu(snafu: &str) -> i64 {
    let mut result = 0;
    let mut pos = 1;
    for b in snafu.bytes().rev() {
        let n = match b {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'-' => -1,
            b'=' => -2,
            _ => panic!("unexpected byte {b}"),
        };

        result += pos * n;
        pos *= 5;
    }

    result
}

fn to_snafu(mut n: i64) -> String {
    if n == 0 {
        return "0".to_owned();
    }

    let mut result = "".to_owned();
    while n > 0 {
        let digit = n % 5;
        if digit == 3 {
            result += "=";
            n += 2;
        } else if digit == 4 {
            result += "-";
            n += 1;
        } else {
            result += &digit.to_string();
        }

        n /= 5;
    }

    result.chars().rev().collect()
}

fn main() {
    let input = include_str!("../input");

    let n = input.lines().map(from_snafu).sum::<i64>();
    println!("{}", to_snafu(n));
}
