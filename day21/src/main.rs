use core::panic;
use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Copy)]
enum Op {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Op {
    fn eval(&self, n1: i64, n2: i64) -> i64 {
        match self {
            Op::Plus => n1 + n2,
            Op::Minus => n1 - n2,
            Op::Times => n1 * n2,
            Op::Divide => n1 / n2,
        }
    }

    fn parse(expr: &str) -> Option<(Self, &str, &str)> {
        let re = Regex::new(r"(.*) (\+|-|\*|/) (.*)").unwrap();
        let cap = re.captures(expr)?;

        let n1 = cap.get(1).unwrap().as_str();
        let n2 = cap.get(3).unwrap().as_str();
        match &cap[2] {
            "+" => Some((Self::Plus, n1, n2)),
            "-" => Some((Self::Minus, n1, n2)),
            "*" => Some((Self::Times, n1, n2)),
            "/" => Some((Self::Divide, n1, n2)),
            o => panic!("unexpected operator {o}"),
        }
    }

    fn right_inverse(&self, result: i64, n2: i64) -> i64 {
        match self {
            Op::Plus => result - n2,
            Op::Minus => result + n2,
            Op::Times => result / n2,
            Op::Divide => result * n2,
        }
    }

    fn left_inverse(&self, result: i64, n1: i64) -> i64 {
        match self {
            Op::Plus => result - n1,
            Op::Minus => n1 - result,
            Op::Times => result / n1,
            Op::Divide => n1 / result,
        }
    }
}

#[derive(Clone, Copy)]
enum Monkey<'a> {
    Number(i64),
    Operation(Op, &'a str, &'a str),
}

impl Monkey<'_> {
    fn parse(line: &str) -> (&str, Monkey) {
        let (name, rest) = line.split_once(": ").unwrap();
        if let Some((op, n1, n2)) = Op::parse(rest) {
            (name, Monkey::Operation(op, n1, n2))
        } else {
            (name, Monkey::Number(rest.parse().unwrap()))
        }
    }
}

enum Tree {
    Leaf { number: i64, is_human: bool },
    Node(Op, Box<Tree>, Box<Tree>),
}

impl Tree {
    fn build<'a>(monkeys: &HashMap<&'a str, Monkey<'a>>, root: &'a str, human: &'a str) -> Self {
        match monkeys[root] {
            Monkey::Number(n) => Self::Leaf {
                number: n,
                is_human: root == human,
            },
            Monkey::Operation(op, n1, n2) => Self::Node(
                op,
                Box::new(Self::build(monkeys, n1, human)),
                Box::new(Self::build(monkeys, n2, human)),
            ),
        }
    }

    fn eval(&self) -> i64 {
        match self {
            Tree::Leaf { number, .. } => *number,
            Tree::Node(op, c1, c2) => op.eval(c1.eval(), c2.eval()),
        }
    }

    fn find_root_human_number(&self) -> i64 {
        match self {
            Tree::Leaf { .. } => panic!("root cannot be a leaf"),
            Tree::Node(_, c1, c2) => match (c1.eval_human(), c2.eval_human()) {
                (Some(n1), None) => c2.find_human_number(n1).unwrap(),
                (None, Some(n2)) => c1.find_human_number(n2).unwrap(),
                (Some(_), Some(_)) => panic!("no branch contains the human"),
                (None, None) => panic!("both branches contain the human"),
            },
        }
    }

    fn find_human_number(&self, target: i64) -> Result<i64, i64> {
        match self {
            Tree::Leaf {
                number, is_human, ..
            } => {
                if *is_human {
                    Ok(target)
                } else {
                    Err(*number)
                }
            }
            Tree::Node(op, c1, c2) => match (c1.eval_human(), c2.eval_human()) {
                (Some(n1), None) => c2.find_human_number(op.left_inverse(target, n1)),
                (None, Some(n2)) => c1.find_human_number(op.right_inverse(target, n2)),
                (Some(n1), Some(n2)) => Err(op.eval(n1, n2)),
                (None, None) => panic!("both branches need the human"),
            },
        }
    }

    fn eval_human(&self) -> Option<i64> {
        match self {
            Tree::Leaf { number, is_human } => {
                if *is_human {
                    None
                } else {
                    Some(*number)
                }
            }
            Tree::Node(op, c1, c2) => Some(op.eval(c1.eval_human()?, c2.eval_human()?)),
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let monkeys: HashMap<&str, Monkey> = input.lines().map(Monkey::parse).collect();
    let tree = Tree::build(&monkeys, "root", "humn");

    println!("{}", tree.eval());
    println!("{:?}", tree.find_root_human_number());
}
