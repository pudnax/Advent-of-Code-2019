use std::io;

use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    Ok(("answer1".to_string(), "answer2".to_string()))
}
