use crate::Error;
use std::hash::Hash;
use std::ops::Deref;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct F64(f64);

impl std::convert::TryFrom<f64> for F64 {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            bail!("Cannot convert {} into f64", value);
        }
        Ok(F64(value))
    }
}

impl F64 {}

impl Deref for F64 {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Eq for F64 {}

impl Hash for F64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) struct Point(i64, i64);

impl Point {
    pub(crate) fn new(x: i64, y: i64) -> Self {
        Point(x, y)
    }

    pub(crate) fn x(&self) -> i64 {
        self.0
    }

    pub(crate) fn y(&self) -> i64 {
        self.1
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::fs;
    use std::io;

    use crate::error::Error;

    pub(crate) fn test_full_problem<F>(day: usize, run_func: F, expected1: &str, expected2: &str)
    where
        F: Fn(io::BufReader<fs::File>) -> Result<(String, String), Error>,
    {
        let path = format!("data/day{:02}", day);
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let (actual1, actual2) = run_func(reader).unwrap();
        assert_eq!(&actual1, expected1);
        assert_eq!(&actual2, expected2);
    }
}
