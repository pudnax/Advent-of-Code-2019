use crate::error::Error;
use std::io::BufRead;

pub fn run<R>(mut input: R) -> Result<(), Error>
where
    R: BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    let mut reader = std::io::BufReader::new(&content[..]);
    run_part(&mut reader, part_one)?;

    let mut reader = std::io::BufReader::new(&content[..]);
    run_part(&mut reader, part_two)?;
    Ok(())
}

pub fn run_part<F, R>(input: &mut R, func: F) -> Result<(), Error>
where
    R: BufRead,
    F: Fn(usize) -> usize,
{
    let mut res = 0;

    for line in &mut input.lines() {
        let n = line?.parse::<usize>()?;

        let fuel = func(n);

        res += fuel;
    }

    println!("{}", res);
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
