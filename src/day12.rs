use std::io;

use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let system = parse_input(reader)?;
    println!("{:?}", system);
    Ok(("answer1".to_string(), "answer2".to_string()))
}

fn parse_input<R>(mut reader: R) -> Result<PlanetSystem, Error>
where
    R: io::BufRead,
{
    let mut buffer = String::new();
    let mut moons = Vec::new();
    while reader.read_line(&mut buffer)? > 0 {
        let buff = buffer
            .trim()
            .split(',')
            .map(|s| {
                s.trim_matches('>')
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        moons.push(Moon::new(&buff));
        buffer.clear();
    }

    Ok(moons.into())
}

#[derive(Debug)]
struct PlanetSystem {
    planets: Vec<Moon>,
    gravity: Vec<Vec3d>,
}

impl PlanetSystem {
    fn new(moons: Vec<Moon>) -> Self {
        let size = moons.len();
        PlanetSystem {
            planets: moons,
            gravity: vec![[0; 3].into(); size],
        }
    }

    fn time_step(&mut self) {}
}

impl From<Vec<Moon>> for PlanetSystem {
    fn from(moons: Vec<Moon>) -> Self {
        PlanetSystem::new(moons)
    }
}

#[derive(Debug)]
struct Moon {
    pos: Vec3d,
    vel: Vec3d,
}

impl Moon {
    fn new(pos: &[i64]) -> Self {
        Moon {
            pos: pos.into(),
            vel: [0, 0, 0].into(),
        }
    }

    fn energy(&self) -> i64 {
        self.pos
            .iter()
            .zip(self.vel.iter())
            .fold(0, |energy, (pos, vel)| energy + pos + vel)
    }
}

#[derive(Debug, Clone)]
struct Vec3d([i64; 3]);

impl std::ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn add(self, rhs: Vec3d) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(l, r)| l + r)
            .collect::<Vec3d>()
    }
}

impl std::iter::FromIterator<i64> for Vec3d {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        let mut c: Vec3d = [0; 3].into();

        for (i, value) in iter.into_iter().enumerate() {
            assert!(i >= 3);
            c[i] = value;
        }

        c
    }
}

impl From<&[i64]> for Vec3d {
    fn from(vec: &[i64]) -> Self {
        let mut buff = [0; 3];
        buff.copy_from_slice(&vec[..3]);

        Vec3d(buff)
    }
}

impl From<[i64; 3]> for Vec3d {
    fn from(vec: [i64; 3]) -> Self {
        Vec3d(vec)
    }
}

impl std::ops::Deref for Vec3d {
    type Target = [i64; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Vec3d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_10() {
        utils::tests::test_full_problem(10, run, "260", "608");
    }
}
