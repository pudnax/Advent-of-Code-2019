use std::io;

use crate::error::Error;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let image = parse_input(input)?;
    let image = construct_matrix(&image, 25, 6)?;
    let answer1 = find_uncorrupted(&image)?;
    let answer2 = decode(image);
    Ok((answer1.to_string(), answer2.to_string()))
}

fn find_uncorrupted(arr: &[&[u8]]) -> Result<usize, Error> {
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

#[derive(Debug)]
struct Image(Vec<u8>);

impl std::ops::Deref for Image {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::new();
        for i in 0..self.len() {
            buff.push((self[i] + 48) as char);
        }
        write!(f, "{}", buff)
    }
}

fn decode(arr: Vec<&[u8]>) -> Image {
    let size = arr[0].len();
    let mut buffer = vec![(0, true); size];

    arr.iter().enumerate().for_each(|(i, &arr)| {
        arr.iter().for_each(|&x| {
            if buffer[i].1 {
                match x {
                    2 => {
                        buffer[i].1 = true;
                    }
                    1 => {
                        buffer[i] = (1, false);
                    }
                    0 => {
                        buffer[i] = (0, false);
                    }
                    _ => unreachable!(),
                }
            }
        })
    });

    Image(buffer.iter().map(|x| x.0).collect::<Vec<_>>())
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
