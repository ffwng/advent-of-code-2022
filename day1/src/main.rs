use std::cmp::Reverse;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input");

    let calories = input.split("\n\n").map(|group| {
        group
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum::<u32>()
    });

    let result1 = calories.clone().max();
    println!("{}", result1.unwrap());

    let result2 = calories
        .map(Reverse)
        .k_smallest(3)
        .map(|v| v.0)
        .sum::<u32>();
    println!("{}", result2);
}
