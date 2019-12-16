use std::io;

use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let signal = parse_input(reader)?;
    println!("{:?}", signal);

    let mut fft = FFT::new(&[0, 1, 0, -1]);
    fft.step();
    fft.step();
    let cycled = signal.iter().zip(fft.into_iter().cycle());
    println!("{:?}", cycled.collect::<Vec<_>>());
    Ok(("answer1".to_string(), "answer2".to_string()))
}

struct FFT<'a> {
    pattern: &'a [i64],
    phase: usize,
}

impl<'a> FFT<'a> {
    fn new(pattern: &[i64]) -> FFT {
        FFT { pattern, phase: 1 }
    }

    fn step(&mut self) {
        self.phase += 1;
    }
}

impl<'a> IntoIterator for FFT<'a> {
    type Item = i64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut iter = self
            .pattern
            .iter()
            .flat_map(|x| vec![*x; self.phase])
            .collect::<Vec<_>>()
            .into_iter();
        iter.next();
        iter
    }
}

fn parse_input<R>(mut reader: R) -> Result<Vec<u8>, Error>
where
    R: io::BufRead,
{
    let mut buff = Vec::new();
    reader.read_to_end(&mut buff)?;
    buff.pop();
    buff.iter_mut().for_each(|b| *b -= 48);

    Ok(buff)
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
