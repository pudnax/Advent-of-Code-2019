use crate::error::Error;
use std::io;

pub fn run<R>(input: R) -> Result<(), Error>
where
    R: io::BufRead,
{
    let (answer1, answer2) = task(input)?;
    println!("{}", answer1);
    println!("{}", answer2);
    Ok(())
}

pub fn task<R>(input: R) -> Result<(usize, usize), Error>
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

    Ok((res1, res2))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let test_cases = &[
            ("12", 2, 2),
            ("14", 2, 2),
            ("1969", 654, 966),
            ("100756", 33583, 50346),
        ];

        for (input, expected1, expected2) in test_cases {
            let reader = io::BufReader::new(input.as_bytes());
            let (actual1, actual2) = task(reader).unwrap();

            assert_eq!(*expected1, actual1);
            assert_eq!(*expected2, actual2);
        }
    }
}
