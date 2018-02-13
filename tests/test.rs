#![feature(proc_macro)]

extern crate derive_ops;
#[macro_use]
use derive_ops::derive_ops;

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use super::derive_ops;

    #[derive(Debug, PartialEq)]
    #[derive_ops(Add)]
    struct Row(i32);

    #[derive(Debug, PartialEq)]
    #[derive_ops(Add)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[test]
    fn add() {
        assert_eq!(Row(2) + Row(2), Row(4));

        assert_eq!(
            Point { x: 1.0, y: 1.0 } + Point { x: 2.0, y: 2.0 },
            Point { x: 3.0, y: 3.0 },
        );
    }
}
