use std::io;

use crate::computer::{Channel, Computer, Rom};
use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    Ok(("answer1".to_string(), "answer2".to_string()))
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
