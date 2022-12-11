use itertools::{self, Itertools};
use lalrpop_util::lalrpop_mod;
use std::cmp::Reverse;

use monkeys::MonkeysParser;
use state::{ModuloMonkey, Monkey};

mod state;
lalrpop_mod!(monkeys);

fn run1(mut monkeys: Vec<Monkey>) -> u32 {
    let mut inspected = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let actions = monkeys[i].turn();
            inspected[i] += actions.len() as u32;
            for act in actions {
                monkeys[act.target].items.push(act.item);
            }
        }
    }

    let result: u32 = inspected
        .iter()
        .map(Reverse)
        .k_smallest(2)
        .map(|r| r.0)
        .product();

    result
}

fn run2(mut monkeys: Vec<ModuloMonkey>) -> u64 {
    let mut inspected = vec![0; monkeys.len()];
    let len = monkeys.len();

    for _ in 0..10000 {
        for i in 0..len {
            let actions = monkeys[i].turn();
            inspected[i] += actions.len() as u32;
            for act in actions {
                monkeys[act.target].items.push(act.item);
            }
        }
    }

    let result: u64 = inspected
        .iter()
        .map(Reverse)
        .k_smallest(2)
        .map(|r| *r.0 as u64)
        .product();

    result
}

fn main() {
    let input = include_str!("../input");
    let monkeys = MonkeysParser::new().parse(input).unwrap();

    println!("{}", run1(monkeys.clone()));

    let moduli = monkeys.iter().map(|m| m.test.divisor).collect_vec();
    let modulo_monkey = monkeys
        .into_iter()
        .map(|m| ModuloMonkey::new(m, &moduli))
        .collect();
    println!("{}", run2(modulo_monkey));
}
