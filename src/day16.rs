use crate::error::Error;
use std::fmt::Write;
use std::io;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let mut signal = parse_input(reader)?;

    let prossesed_signal = part1(&mut signal);

    let answer1 = join(&prossesed_signal[0..8]);
    Ok((answer1, "answer2".to_string()))
}

fn join(a: &[i64]) -> String {
    a.iter().fold(String::new(), |mut s, &n| {
        write!(s, "{}", n).ok();
        s
    })
}

fn part1(signal: &mut [i64]) -> &[i64] {
    let mut fft = FFT::new(&[0, 1, 0, -1]);
    let mut buff = vec![0i64; signal.len()];

    for _ in 0..100 {
        for elem in &mut buff {
            *elem = signal
                .iter()
                .zip(fft)
                .fold(0, |acc, (s, wave)| acc + s * wave)
                .abs()
                % 10;
            fft.step();
        }
        signal.clone_from_slice(&buff[..]);
        fft.flush();
    }

    signal
}

#[derive(Clone, Copy)]
struct FFT<'a> {
    pattern: &'a [i64],
    phase: usize,
    pattern_size: usize,
    counter: usize,
}

impl<'a> Iterator for FFT<'a> {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.pattern[self.counter / self.phase % self.pattern_size];
        self.counter += 1;
        Some(res)
    }
}

impl<'a> FFT<'a> {
    fn new(pattern: &[i64]) -> FFT {
        FFT {
            pattern,
            phase: 1,
            pattern_size: pattern.len(),
            counter: 1,
        }
    }

    fn step(&mut self) {
        self.phase += 1;
    }

    fn flush(&mut self) {
        self.phase = 1;
    }
}

fn parse_input<R>(mut reader: R) -> Result<Vec<i64>, Error>
where
    R: io::BufRead,
{
    let mut buff = Vec::new();
    reader.read_to_end(&mut buff)?;
    buff.pop();
    Ok(buff.iter().map(|b| (*b - 48) as i64).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_16() {
        utils::tests::test_full_problem(16, run, "76795888", "answer2");
    }
}
