use std::io;

use crate::error::Error;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let image = parse_input(input)?;
    let image = construct_matrix(&image, 25, 6)?;
    let answer1 = find_uncorrupted(&image.0)?;
    let answer2 = decode(image.0, image.1, image.2);
    Ok((answer1.to_string(), answer2.to_string()))
}

fn find_uncorrupted(arr: &[&[u8]]) -> Result<usize, Error> {
    let min_index = arr
        .iter()
        .enumerate()
        .min_by_key(|(_, &arr)| bytecount::count(arr, 0))
        .ok_or_else(|| error!("Unable find array with 0"))?
        .0;
    let ones = bytecount::count(arr[min_index], 1);
    let twos = bytecount::count(arr[min_index], 2);

    Ok(ones * twos)
}

#[derive(Debug)]
struct Image((Vec<u8>, usize, usize));

impl std::ops::Deref for Image {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &(self.0).0
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::new();
        let mut iter = (self.0).0.iter();
        for _row in 0..(self.0).2 {
            for _col in 0..(self.0).1 {
                match iter.next() {
                    Some(0) => buff.push('\u{2585}'),
                    Some(1) => buff.push(' '),
                    Some(_) | None => {}
                }
            }
            buff.push('\n');
        }
        write!(f, "{}", buff)
    }
}

fn decode(arr: Vec<&[u8]>, w: usize, h: usize) -> Image {
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

    Image((buffer.iter().map(|x| x.0).collect::<Vec<_>>(), w, h))
}

fn construct_matrix(arr: &[u8], w: usize, h: usize) -> Result<(Vec<&[u8]>, usize, usize), Error> {
    if arr.len() % (w * h) != 0 {
        bail!("Uncorrect array size");
    }

    Ok((arr.chunks(w * h).collect::<Vec<_>>(), w, h))
}

fn parse_input<R>(mut reader: R) -> Result<Vec<u8>, Error>
where
    R: io::BufRead,
{
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    buffer.pop();
    buffer.iter_mut().for_each(|b| *b -= 48);

    Ok(buffer)
}
