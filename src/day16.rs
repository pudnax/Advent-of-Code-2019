use std::io;

use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let signal = parse_input(reader)?;
    println!("{:?}", signal);
    Ok(("answer1".to_string(), "answer2".to_string()))
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
