use enum_map::{enum_map, Enum, EnumMap};
use good_lp::{
    constraint, default_solver, variable, Expression, ProblemVariables, Solution, SolverModel,
    Variable,
};

use regex::Regex;
use Resource::*;

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Blueprint {
    costs: EnumMap<Resource, EnumMap<Resource, u32>>,
}

impl Blueprint {
    fn parse(line: &str) -> (u32, Self) {
        let regex = Regex::new("Blueprint (.*): Each ore robot costs (.*) Each clay robot costs (.*) Each obsidian robot costs (.*). Each geode robot costs (.*).").unwrap();
        let cap = regex.captures(line).unwrap();

        (
            cap[1].parse().unwrap(),
            Self {
                costs: enum_map! {
                    Ore => parse_price(&cap[2]),
                    Clay => parse_price(&cap[3]),
                    Obsidian => parse_price(&cap[4]),
                    Geode => parse_price(&cap[5])
                },
            },
        )
    }
}

fn parse_price(price: &str) -> EnumMap<Resource, u32> {
    let mut result = enum_map! {
        _ => 0
    };

    let regex = Regex::new("(\\d+) (ore|clay|obsidian)").unwrap();
    for cap in regex.captures_iter(price) {
        let resource = match &cap[2] {
            "ore" => Ore,
            "clay" => Clay,
            "obsidian" => Obsidian,
            _ => panic!("unexpected resource"),
        };
        result[resource] = cap[1].parse().unwrap();
    }

    result
}

#[derive(Debug)]
struct ResourceVars {
    amounts: Vec<Variable>,
    robots: Vec<Variable>,
    robots_built: Vec<Variable>,
}

fn max_geodes(blueprint: &Blueprint, minutes: usize) -> u32 {
    let mut vars = ProblemVariables::new();

    let resource_vars: EnumMap<Resource, ResourceVars> = enum_map! {
        _ => ResourceVars {
            amounts: vars.add_vector(variable().integer().min(0), minutes),
            robots: vars.add_vector(variable().integer().min(0), minutes),
            robots_built: vars.add_vector(variable().binary(), minutes),
        }
    };

    let target = resource_vars[Geode].amounts[minutes - 1];

    let mut model = vars.maximise(target).using(default_solver);
    model.set_parameter("log", "0");

    for minute in 0..minutes {
        for (resource, vars) in &resource_vars {
            let amount = vars.amounts[minute];
            // a robot built in minute n only produces resources in minute n+2
            // note: in the original problem description, this corresponds to a robot built in minute n+1
            //       but we have to ensure that a robot cannot be build from resources that were just gathered
            // note: this does not apply to the starting ore robot
            let mut gathered: Expression = if minute > 1 {
                vars.robots[minute - 2].into()
            } else {
                0.into()
            };
            gathered += if resource == Ore { 1 } else { 0 };

            let amount_constraint = if minute == 0 {
                // start with no resources
                constraint!(amount == gathered)
            } else {
                // new resources are previous resources + resources gathered - resources spent
                let spent = blueprint
                    .costs
                    .iter()
                    .map(|(r, c)| resource_vars[r].robots_built[minute] * c[resource])
                    .sum::<Expression>();

                let new_value = vars.amounts[minute - 1] + gathered - spent;
                constraint!(amount == new_value)
            };
            model.add_constraint(amount_constraint);

            let robots = vars.robots[minute];
            let robots_constraint = if minute == 0 {
                // start with no additional robots (the one ore robot is incorporated in the gathered formula above)
                constraint!(robots == 0)
            } else {
                let new_value = vars.robots[minute - 1] + vars.robots_built[minute];
                constraint!(robots == new_value)
            };

            model.add_constraint(robots_constraint);
        }

        // only at most one robot can be built per step
        let robots_built = resource_vars
            .values()
            .map(|vars| vars.robots_built[minute])
            .sum::<Expression>();
        model.add_constraint(constraint!(robots_built <= 1));
    }

    let solution = model.solve().unwrap();
    solution.value(target) as u32
}

fn main() {
    let input = include_str!("../input");

    let mut result1 = 0;
    for line in input.lines() {
        let (id, blueprint) = Blueprint::parse(line);
        result1 += id * max_geodes(&blueprint, 24);
    }

    println!("{result1}");

    let result2 = input
        .lines()
        .take(3)
        .map(|line| {
            let (_, blueprint) = Blueprint::parse(line);
            max_geodes(&blueprint, 32)
        })
        .product::<u32>();

    println!("{result2}");
}
