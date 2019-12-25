use std::io;

use crate::computer::{Computer, Rom};
use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let rom = Rom::from_reader(reader)?;
    let mut computer = Computer::default();

    computer.input_mut().push_back(1);
    computer.execute(&rom, None)?;
    let answer1 = computer.output_mut().pop_front()?;

    computer.input_mut().try_clear();
    computer.output_mut().try_clear();

    computer.input_mut().push_back(2);
    computer.execute(&rom, None)?;
    let answer2 = computer.output_mut().pop_front()?;

    Ok((answer1.to_string(), answer2.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_09() {
        utils::tests::test_full_problem(9, run, "3839402290", "35734");
    }
}
