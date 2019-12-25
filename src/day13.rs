use std::io;

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
    fn test_12() {
        utils::tests::test_full_problem(121, run, "14645", "answer2");
    }
}
