use std::io;

use crate::error::Error;
use std::collections::{HashMap, VecDeque};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run<R>(mut reader: R) -> std::result::Result<(String, String), Error>
where
    R: io::BufRead,
{
    let input = {
        let mut buff = String::new();
        reader.read_to_string(&mut buff)?;
        buff
    };

    let answer1 = part1(&input).unwrap();
    let answer2 = part2(&input).unwrap();

    Ok((answer1.to_string(), answer2.to_string()))
}

fn part1(input: &str) -> Result<u64> {
    produce_fuel(1, &parse_input(input)?, &mut HashMap::new())
}

fn part2(input: &str) -> Result<u64> {
    let reactions = parse_input(input)?;
    let mut extra = HashMap::new();

    let ore_per_fuel = produce_fuel(1, &reactions, &mut extra)?;
    let mut produced_fuel = 1;
    if extra.contains_key("FUEL") {
        // Performance fix if input contains a reaction that produces more than 1 FUEL
        produced_fuel += extra.remove("FUEL").unwrap();
    }
    let produced_fuel_multiplier = produced_fuel;

    *extra.entry("ORE").or_insert(0) = 1_000_000_000_000 - ore_per_fuel;

    loop {
        let produceable_fuel = std::cmp::max(
            1,
            extra.get("ORE").unwrap() / ore_per_fuel * produced_fuel_multiplier,
        );
        let required_ore = produce_fuel(produceable_fuel, &reactions, &mut extra)?;
        if required_ore != 0 {
            return Ok(produced_fuel);
        }
        produced_fuel += produceable_fuel;
    }
}

type ProductChain<'a> = HashMap<&'a str, (u64, Vec<(u64, &'a str)>)>;

fn produce_fuel<'a>(
    fuel_quantity: u64,
    reactions: &ProductChain<'a>,
    extra_chems: &mut HashMap<&'a str, u64>,
) -> Result<u64> {
    let mut need_chems = VecDeque::new();
    need_chems.push_back(("FUEL", fuel_quantity));
    let mut required_ore = 0;

    loop {
        match need_chems.pop_front() {
            Some(("ORE", mut quantity)) => {
                let extra_ore = extra_chems.entry("ORE").or_insert(0);
                let extra_used = std::cmp::min(quantity, *extra_ore);
                quantity -= extra_used;
                *extra_ore -= extra_used;
                required_ore += quantity;
            }
            Some((need, mut quantity)) => {
                let extra = extra_chems.entry(need).or_insert(0);
                let extra_used = std::cmp::min(quantity, *extra);
                quantity -= extra_used;
                *extra -= extra_used;
                if quantity > 0 {
                    let (reaction_quantity, reagents) = reactions
                        .get(need)
                        .ok_or_else(|| "Missing reaction for a reagent")?;
                    let reaction_multiplier = (quantity - 1) / reaction_quantity + 1;
                    *extra = reaction_quantity * reaction_multiplier - quantity;
                    for &(reagent_quantity, reagent_chem) in reagents {
                        need_chems
                            .push_back((reagent_chem, reagent_quantity * reaction_multiplier));
                    }
                }
            }
            None => return Ok(required_ore),
        }
    }
}

fn parse_input(input: &str) -> Result<ProductChain> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" => ");
            let reagents = split
                .next()
                .ok_or_else(|| "Reagents expected")?
                .split(", ")
                .map(parse_chemical)
                .collect::<Result<_>>()?;
            let (product_quantity, product_type) =
                parse_chemical(split.next().ok_or_else(|| "Products expected")?)?;
            Ok((product_type, (product_quantity, reagents)))
        })
        .collect::<Result<_>>()
}

fn parse_chemical(input: &str) -> Result<(u64, &str)> {
    let mut split = input.split(' ');
    let quantity = split
        .next()
        .ok_or_else(|| "Chemical quantity expected")?
        .parse()
        .map_err(|_| "Chemical quantity not an integer")?;
    let chem_type = split.next().ok_or_else(|| "Chemical quantity expected")?;
    Ok((quantity, chem_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_14() {
        utils::tests::test_full_problem(14, run, "248794", "4906796");
    }
}
