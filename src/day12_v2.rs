use std::cell::RefCell;
use std::io;

use crate::error::Error;
use crate::utils::Vec3;
pub fn run<R>(reader: R) -> std::result::Result<(String, String), Error>
where
    R: io::BufRead,
{
    let moons = parse_input(reader)?;
    println!("{:?}", moons);
    Ok(("answer1".to_string(), "answer2".to_string()))
}

fn parse_input<R>(reader: R) -> Result<Moons, Error>
where
    R: io::BufRead,
{
    let mut moons: [RefCell<Moon>; 4] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() }; // undefined behavior!
    let mut count = 0;
    for res in reader.lines() {
        if count > 3 {
            bail!("Can only support exactly 4 moons")
        }
        let line = res?;
        let line = line.trim();

        let mut pos: [i64; 3] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() }; // undefined behavior!
        let mut j = 0;
        for part in line.split(',') {
            let coord = part
                .split('=')
                .nth(1)
                .ok_or_else(|| error!("TODO"))?
                .chars()
                .take_while(|&c| c != '>')
                .collect::<String>()
                .parse::<i64>()?;
            pos[j] = coord;
            j += 1;
        }
        if j != 3 {
            bail!("Found too many coordinates")
        }

        let moon = Moon {
            pos: pos.into(),
            vel: Vec3::new(0, 0, 0),
        };
        moons[count] = RefCell::new(moon);
        count += 1;
    }
    if count != 4 {
        bail!("Can only exactly support 4 moons")
    }

    Ok(Moons(moons))
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Moons([RefCell<Moon>; 4]);

impl Moons {
    fn step(&mut self) {
        for moon_i in &self.0 {
            for moon_j in &self.0 {
                if moon_i == moon_j {
                    continue;
                }
                for k in 0..3 {
                    let pos_i = moon_i.borrow().pos()[k];
                    let pos_j = moon_j.borrow().pos()[k];
                    if pos_i < pos_j {
                        moon_i.borrow_mut().vel_mut()[k] += 1;
                    } else if pos_i > pos_j {
                        moon_i.borrow_mut().vel_mut()[k] -= 1;
                    }
                }
            }
        }
        for moon in self.iter_mut() {
            for k in 0..3 {
                let vel = { moon.borrow().vel()[k] };
                moon.borrow_mut().pos_mut()[k] += vel;
            }
        }
    }
}

impl std::ops::Deref for Moons {
    type Target = [RefCell<Moon>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Moons {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Copy, Clone, Eq, Default, PartialEq, Debug)]
struct Moon {
    pos: Vec3<i64>,
    vel: Vec3<i64>,
}

impl Moon {
    pub(crate) fn pos(&self) -> &Vec3<i64> {
        &self.pos
    }

    pub(crate) fn pos_mut(&mut self) -> &mut Vec3<i64> {
        &mut self.pos
    }

    pub(crate) fn vel(&self) -> &Vec3<i64> {
        &self.vel
    }

    pub(crate) fn vel_mut(&mut self) -> &mut Vec3<i64> {
        &mut self.vel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_14() {
        utils::tests::test_full_problem(9, run, "3460311188", "42202");
    }
}
