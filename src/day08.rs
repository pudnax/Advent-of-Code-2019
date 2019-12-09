use std::io;

use crate::error::Error;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let ans = parse_input(input, 25, 6)?;
    println!("{:?}", ans);
    Ok(("ans".to_string(), "bar".to_string()))
}

fn parse_input<R>(mut input: R, w: usize, h: usize) -> Result<Vec<u8>, Error>
where
    R: io::BufRead,
{
    let mut buffer = String::new();
    let mut acc = Vec::new();
    while input.read_line(&mut buffer)? > 0 {
        acc = buffer
            .trim()
            .chars()
            .map(|x| x as u8 - 48)
            .collect::<Vec<_>>();
        buffer.clear();
    }

    Ok(acc)
}
