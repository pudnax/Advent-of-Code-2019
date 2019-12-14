use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io;

use crate::error::Error;
use crate::utils::{Point, F64};

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let points = parse_input(reader)?;

    let (answer1, laser) = part1(&points)?;
    Ok((answer1.to_string(), "bar".to_string()))
}

fn part2(laser: Point, points: &[Point]) -> Result<Asteroids, Error> {
    for point in points {
        if *point == laser {
            continue;
        }
    }
    unimplemented!()
}

struct Asteroids {}

impl Iterator for Asteroids {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

fn part1(points: &[Point]) -> Result<(usize, Point), Error> {
    let mut map = HashMap::new();

    for origin in points {
        for other in points {
            let direction = Direction::new(*origin, *other)?;
            map.entry(*origin)
                .or_insert_with(HashSet::new)
                .insert(direction);
        }
    }

    let (max, point) = map.iter().fold((0, None), |mut state, (point, set)| {
        let count = set.len();
        if count > state.0 {
            state = (count, Some(point));
        }
        state
    });

    let point = *point.ok_or_else(|| error!("TODO"))?;
    Ok((max, point))
}

#[derive(PartialEq, Eq, Hash)]
pub struct Direction(F64);

impl Direction {
    fn new(origin: Point, other: Point) -> Result<Self, Error> {
        let (x, y) = (other.x() - origin.x(), other.y() - origin.y());
        let angle = (y as f64).atan2(x as f64);
        Ok(Direction(F64::try_from(angle)?))
    }
}

fn parse_input<R>(mut reader: R) -> Result<Vec<Point>, Error>
where
    R: io::BufRead,
{
    let mut buff = String::new();
    let mut points = Vec::new();
    let mut y = 0;

    while reader.read_line(&mut buff)? > 0 {
        buff.trim().chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                let point = Point::new(x as i64, y);
                points.push(point);
            }
        });

        buff.clear();
        y += 1;
    }

    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_09() {
        utils::tests::test_full_problem(10, run, "260", "608");
    }
}
