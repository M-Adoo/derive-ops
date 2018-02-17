#![feature(proc_macro)]

extern crate derive_ops;
#[macro_use]
use derive_ops::derive_ops;

#[cfg(test)]
mod tests {
    use std::ops::{Add, AddAssign};
    use super::derive_ops;

    #[derive(Debug, PartialEq)]
    #[derive_ops(Add, AddAssign)]
    struct Row(i32);

    #[derive(Debug, PartialEq)]
    #[derive_ops(Add, AddAssign)]
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

    #[test]
    fn add_assian() {
        let mut row = Row(2);
        row += Row(3);
        assert_eq!(row, Row(5));

        let mut pt = Point { x: 2.0, y: 3.0 };
        pt += Point { x: 3.0, y: 3.0 };
        assert_eq!(pt, Point { x: 5.0, y: 6.0 });
    }
}
