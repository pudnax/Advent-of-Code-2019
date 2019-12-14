use std::io;

use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let mut system = parse_input(reader)?;

    system.wait(1000);
    println!("{:?}", system);
    // for _ in 0..5 {
    //     println!("{:?}", system);
    //     system.time_step();
    // }
    let answer1 = system.energy();
    Ok((answer1.to_string(), "answer2".to_string()))
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

struct PlanetSystem {
    planets: Vec<Moon>,
}

impl PlanetSystem {
    fn new(moons: Vec<Moon>) -> Self {
        PlanetSystem { planets: moons }
    }

    fn apply_gravity(&mut self) {
        let size = self.planets.len();
        for i in 0..size {
            for j in 0..size {
                for k in 0..self.planets[i].pos.len() {
                    match self.planets[i].pos[k].cmp(&self.planets[j].pos[k]) {
                        std::cmp::Ordering::Greater => {
                            self.planets[i].vel[k] += -1;
                        }
                        std::cmp::Ordering::Less => {
                            self.planets[i].vel[k] += 1;
                        }
                        std::cmp::Ordering::Equal => {}
                    }
                }
            }
        }
    }

    fn time_step(&mut self) {
        self.apply_gravity();
        for planet in &mut self.planets {
            for k in 0..planet.pos.len() {
                planet.pos[k] += planet.vel[k];
            }
        }
    }

    fn wait(&mut self, t: usize) {
        for _ in 0..t {
            self.time_step();
        }
    }

    fn energy(&self) -> i64 {
        self.planets
            .iter()
            .fold(0, |acc, planet| acc + planet.energy())
    }
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
        let kin_energy = self.pos.iter().fold(0, |acc, val| acc + val.abs());
        let pot_energy = self.vel.iter().fold(0, |acc, val| acc + val.abs());
        kin_energy * pot_energy
    }
}

#[derive(Clone, Copy)]
struct Vec3d([i64; 3]);

impl std::iter::FromIterator<i64> for Vec3d {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        let mut c: Vec3d = [0; 3].into();

        for (i, value) in iter.into_iter().enumerate() {
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

impl std::fmt::Debug for Vec3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {} {}]", self[0], self[1], self[2])
    }
}

impl std::fmt::Debug for PlanetSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "planets: {:?}", self.planets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_12() {
        utils::tests::test_full_problem(121, run, "1940", "answer2");
    }
}
