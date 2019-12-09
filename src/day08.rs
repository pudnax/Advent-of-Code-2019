use std::io;

use crate::error::Error;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let ans = parse_input(input)?;
    let ans = construct_matrix(&ans, 25, 6)?;
    let ans = find_uncorrupted(ans)?;
    println!("{:?}", ans);
    Ok(("ans".to_string(), "bar".to_string()))
}

fn find_uncorrupted(arr: Vec<&[u8]>) -> Result<usize, Error> {
    let min_index = arr
        .iter()
        .enumerate()
        .min_by_key(|(_, &arr)| arr.iter().filter(|&&x| x == 0).count())
        .ok_or_else(|| error!("Unable find array with 0"))?
        .0;
    let ones = arr[min_index].iter().filter(|&&x| x == 1).count();
    let twos = arr[min_index].iter().filter(|&&x| x == 2).count();

    Ok(ones * twos)
}

fn construct_matrix(arr: &[u8], w: usize, h: usize) -> Result<Vec<&[u8]>, Error> {
    if arr.len() % (w * h) != 0 {
        bail!("Uncorrect array size");
    }

    Ok(arr.chunks(w * h).collect::<Vec<_>>())
}

fn parse_input<R>(mut input: R) -> Result<Vec<u8>, Error>
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
