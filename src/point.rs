use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: _rhs.x + (self.x * -1),
            y: _rhs.y + (self.y * -1),
        }
    }
}

#[test]
fn test_add() {
    let p_one = Point { x: 1, y: 1 };
    let p_two = Point { x: 2, y: 2 };
    assert_eq!(Point{ x:3, y:3 }, p_one + p_two);
}

#[test]
fn test_subtract() {
    let p_one = Point { x: 4, y: 5 };
    let p_two = Point { x: 12, y: 2 };
    assert_eq!(Point{ x: 8, y: -3}, p_one - p_two);
}
