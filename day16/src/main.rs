use std::{collections::HashMap, hash::Hash, time::SystemTime};

use bitset_core::BitSet;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbors: Vec<String>,
    index: usize,
}

impl Valve {
    fn parse(line: &str, index: usize) -> (String, Self) {
        let re =
            Regex::new("^Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)$").unwrap();
        let caps = re.captures(line).unwrap();

        let name = caps[1].to_owned();
        let flow_rate = caps[2].parse().unwrap();
        let neighbors = caps[3].split(", ").map(|n| n.to_owned()).collect();

        let valve = Self {
            flow_rate,
            neighbors,
            index,
        };
        (name, valve)
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct State1<'a> {
    pos: &'a str,
    closed_valves: u64,
}

fn max_pressure1<'a>(
    valves: &'a HashMap<String, Valve>,
    valve_combinations: &[u64],
    minute: u32,
) -> HashMap<State1<'a>, u32> {
    if minute == 30 {
        return HashMap::new();
    }

    let next_pressure = max_pressure1(valves, valve_combinations, minute + 1);

    println!("Minute {minute}");

    valves
        .iter()
        .flat_map(|(pos, valve)| {
            valve_combinations.iter().filter_map(|&closed_valves| {
                // staying at position
                let mut pressure = *next_pressure
                    .get(&State1 { pos, closed_valves })
                    .unwrap_or(&0);

                // current valve is opened
                if closed_valves.bit_test(valve.index) {
                    let mut new_closed_values = closed_valves;
                    new_closed_values.bit_reset(valve.index);

                    pressure = pressure.max(
                        valve.flow_rate * (30 - minute)
                            + *next_pressure
                                .get(&State1 {
                                    pos,
                                    closed_valves: new_closed_values,
                                })
                                .unwrap_or(&0),
                    );
                }

                // move to some neighbor
                for n in &valve.neighbors {
                    pressure = pressure.max(
                        *next_pressure
                            .get(&State1 {
                                pos: n,
                                closed_valves,
                            })
                            .unwrap_or(&0),
                    );
                }

                (pressure > 0).then_some((State1 { pos, closed_valves }, pressure))
            })
        })
        .collect()
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct State2 {
    pos1: usize,
    pos2: usize,
    closed_valves: u64,
}

struct States2 {
    valves_count: usize,
    pressures: Vec<HashMap<u64, u32>>,
}

impl States2 {
    fn new(valves_count: usize) -> Self {
        Self {
            valves_count,
            pressures: vec![HashMap::new(); valves_count * valves_count],
        }
    }

    fn get(&self, pos1: usize, pos2: usize, closed_valves: u64) -> u32 {
        *self.pressures[self.index(pos1, pos2)]
            .get(&closed_valves)
            .unwrap_or(&0)
    }

    fn set(&mut self, pos1: usize, pos2: usize, closed_valves: u64, pressure: u32) {
        let idx = self.index(pos1, pos2);
        self.pressures[idx].insert(closed_valves, pressure);
    }

    fn index(&self, pos1: usize, pos2: usize) -> usize {
        pos2 * self.valves_count + pos1
    }
}

fn max_pressure2<'a>(
    valves: &'a HashMap<String, Valve>,
    valve_combinations: &[u64],
    minute: u32,
    result: &mut States2,
    next_pressure: &mut States2,
) {
    if minute == 30 {
        *result = States2::new(valves.len());
        return;
    }

    let now = SystemTime::now();
    max_pressure2(
        valves,
        valve_combinations,
        minute + 1,
        next_pressure,
        result,
    );

    println!(
        "Minute {minute} in {}s",
        now.elapsed().unwrap().as_secs_f32()
    );

    for valve1 in valves.values() {
        let pos1 = valve1.index;
        for valve2 in valves.values() {
            let pos2 = valve2.index;
            for &closed_valves in valve_combinations {
                // both staying at position
                let mut pressure = next_pressure.get(pos1, pos2, closed_valves);

                let mut check = |new_pressure: u32,
                                 new_pos1: usize,
                                 new_pos2: usize,
                                 new_closed_valves: u64| {
                    pressure = pressure.max(
                        new_pressure + next_pressure.get(new_pos1, new_pos2, new_closed_valves),
                    );
                };

                // both valves are opened
                if closed_valves.bit_test(valve1.index) && closed_valves.bit_test(valve2.index) {
                    let new_pressure = if pos1 == pos2 {
                        valve1.flow_rate * (30 - minute)
                    } else {
                        valve1.flow_rate * (30 - minute) + valve2.flow_rate * (30 - minute)
                    };

                    let mut new_closed_values = closed_valves;
                    new_closed_values.bit_reset(valve1.index);
                    new_closed_values.bit_reset(valve2.index);

                    check(new_pressure, pos1, pos2, new_closed_values);
                }

                // only first valve opened and moving to neighbor
                if closed_valves.bit_test(valve1.index) {
                    let new_pressure = valve1.flow_rate * (30 - minute);

                    let mut new_closed_values = closed_valves;
                    new_closed_values.bit_reset(valve1.index);

                    for n in &valve2.neighbors {
                        check(new_pressure, pos1, valves[n].index, new_closed_values);
                    }
                }

                // only second valve opened and moving to neighbor
                if closed_valves.bit_test(valve2.index) {
                    let new_pressure = valve2.flow_rate * (30 - minute);

                    let mut new_closed_values = closed_valves;
                    new_closed_values.bit_reset(valve2.index);

                    for n in &valve1.neighbors {
                        check(new_pressure, valves[n].index, pos2, new_closed_values);
                    }
                }

                // both moving to neighbors
                for n1 in &valve1.neighbors {
                    for n2 in &valve2.neighbors {
                        check(0, valves[n1].index, valves[n2].index, closed_valves);
                    }
                }

                if pressure > 0 {
                    result.set(pos1, pos2, closed_valves, pressure)
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let valves: HashMap<String, Valve> = input
        .lines()
        .enumerate()
        .map(|(i, l)| Valve::parse(l, i))
        .collect();

    let nonzero_valves = valves.values().filter(|v| v.flow_rate > 0).map(|v| v.index);

    let valve_combinations: Vec<u64> = nonzero_valves
        .clone()
        .powerset()
        .map(|indices| {
            let mut closed_valves = 0;
            for idx in indices {
                closed_valves.bit_set(idx);
            }

            closed_valves
        })
        .collect();

    let mut initial_closed_valves = 0;
    for idx in nonzero_valves {
        initial_closed_valves.bit_set(idx);
    }

    println!("PART 1");
    let result1 = max_pressure1(&valves, &valve_combinations, 1);

    println!(
        "{:?}",
        result1
            .get(&State1 {
                pos: "AA",
                closed_valves: initial_closed_valves
            })
            .unwrap_or(&0)
    );

    println!("PART 2");
    let mut result2 = States2::new(valves.len());
    let mut next_pressure = States2::new(valves.len());
    max_pressure2(
        &valves,
        &valve_combinations,
        5,
        &mut result2,
        &mut next_pressure,
    );

    let start_pos = valves["AA"].index;
    println!(
        "{:?}",
        result2.get(start_pos, start_pos, initial_closed_valves)
    );
}
