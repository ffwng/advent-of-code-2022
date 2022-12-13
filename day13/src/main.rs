use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Input {
    Integer(i32),
    List(Vec<Input>),
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Input {}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Integer(b)) => (&**a).cmp(&[Self::Integer(*b)]),
            (a, b) => b.cmp(a).reverse(),
        }
    }
}

impl Input {
    fn parse(line: &str) -> Result<Input, std::num::ParseIntError> {
        Ok(Self::parse_one(line)?.0)
    }

    fn parse_one(part: &str) -> Result<(Input, &str), std::num::ParseIntError> {
        if part.starts_with('[') {
            let mut items = vec![];
            let mut part = &part[1..]; // skip [
            loop {
                if part.starts_with(']') {
                    break;
                } else if part.starts_with(',') {
                    // ignore the ,
                    part = &part[1..];
                }

                let (input, rest) = Input::parse_one(part)?;
                items.push(input);
                part = rest;
            }

            Ok((Self::List(items), &part[1..])) // skip final ]
        } else {
            let (number, rest) = match part.find(|c| c == ',' || c == ']') {
                None => (part, ""),
                Some(i) => (&part[..i], &part[i..]),
            };
            let number = number.parse()?;

            Ok((Self::Integer(number), rest))
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let mut lines = input.lines();
    let mut idx = 1;
    let mut sum = 0;
    while let (Some(p1), Some(p2)) = (lines.next(), lines.next()) {
        let p1 = Input::parse(p1).unwrap();
        let p2 = Input::parse(p2).unwrap();

        if p1 <= p2 {
            sum += idx;
        }
        idx += 1;

        // skip empty line
        lines.next();
    }

    println!("{sum}");

    let divider1 = Input::parse("[[2]]").unwrap();
    let divider2 = Input::parse("[[6]]").unwrap();
    let mut packets = vec![divider1.clone(), divider2.clone()];
    for line in input.lines() {
        if !line.is_empty() {
            packets.push(Input::parse(line).unwrap());
        }
    }

    packets.sort();

    let idx1 = packets.iter().position(|p| p == &divider1).unwrap_or(0) + 1;
    let idx2 = packets.iter().position(|p| p == &divider2).unwrap_or(0) + 1;

    println!("{}", idx1 * idx2)
}
