use std::io;

use crate::error::Error;

pub fn run<R>(mut reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    buffer.pop();
    buffer.iter_mut().for_each(|b| *b -= 48);

    const ROWS: usize = 6;
    const COLS: usize = 25;

    let answer1 =
        match buffer
            .chunks(ROWS * COLS)
            .fold((std::usize::MAX, None), |mut state, layer| {
                let nzeros = layer.iter().filter(|&&b| b == 0).count();
                if nzeros < state.0 {
                    state = (nzeros, Some(layer));
                }
                state
            }) {
            (_, Some(layer)) => {
                let nones = layer.iter().filter(|&&b| b == 1).count();
                let ntwos = layer.iter().filter(|&&b| b == 2).count();
                nones * ntwos
            }
            (_, None) => bail!("Can't count layers"),
        };

    let image = buffer
        .chunks(ROWS * COLS)
        .fold([2u8; ROWS * COLS], |mut state, layer| {
            state.iter_mut().enumerate().for_each(|(i, b)| {
                if *b == 2 {
                    *b = layer[i];
                }
            });
            state
        });

    let mut iter = image.iter();
    let mut answer2 = String::new();
    for _row in 0..ROWS {
        for _col in 0..COLS {
            match iter.next() {
                Some(0) => answer2.push('\u{2585}'),
                Some(1) => answer2.push(' '),
                Some(_) => bail!("TODO"),
                None => bail!("TODO"),
            }
        }
        answer2.push('\n');
    }

    Ok((answer1.to_string(), answer2.to_string()))
}
