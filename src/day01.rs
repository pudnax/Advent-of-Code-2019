use crate::error::Error;
use std::io;

pub fn run<R>(input: R) -> Result<(), Error>
where
    R: io::BufRead,
{
    let mut res1 = 0;
    let mut res2 = 0;

    for line in input.lines() {
        let n = line?.parse::<usize>()?;

        res1 += part_one(n);
        res2 += part_two(n);
    }

    println!("{}", res1);
    println!("{}", res2);
    Ok(())
}

fn part_one(n: usize) -> usize {
    match (n / 3).checked_sub(2) {
        Some(m) => m,
        None => 0,
    }
}

fn part_two(mut n: usize) -> usize {
    let mut total = 0;
    loop {
        let m = match (n / 3).checked_sub(2) {
            Some(m) => m,
            None => break total,
        };
        total += m;
        n = m;
    }
}
