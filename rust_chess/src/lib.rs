#![allow(clippy::all)]
pub mod board;

pub mod moves;

pub mod units;

#[cfg(test)]
mod tests {
    #[test]
    fn working() {
        assert_eq!(2 + 2, 4);
    }
}
