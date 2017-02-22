use std::ops::{Add, Sub};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Ord, PartialOrd)]
pub struct Point {
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

impl Point {
    #[allow(dead_code)]
    pub fn new(_x: i32, _y: i32) -> Point {
        Point { x: _x, y: _y}
    }

    #[allow(dead_code)]
    pub fn distance(&self, point: &Point) -> f32 {
        (((point.y - self.y).pow(2) + (point.x - self.x).pow(2)) as f32).sqrt()
    }

    #[allow(dead_code)]
    pub fn gradient(&self, point: &Point) -> f32 {
        ((point.x - self.x) / (point.y - self.y)) as f32
    }
}

#[test]
fn test_new() {
    let point = Point::new(1, 1);
    assert_eq!(Point {x: 1, y: 1}, point);
}

#[test]
fn test_add() {
    let p_one = Point { x: 1, y: 1 };
    let p_two = Point { x: 2, y: 2 };
    assert_eq!(Point { x: 3, y: 3 }, p_one + p_two);
}

#[test]
fn test_subtract() {
    let p_one = Point { x: 4, y: 5 };
    let p_two = Point { x: 12, y: 2 };
    assert_eq!(Point { x: 8, y: -3 }, p_one - p_two);
}

#[test]
fn test_distance() {
    let p_one = Point { x: 1, y: 1 };
    let p_two = Point { x: 12, y: 2 };
    assert_eq!(11.045361f32, p_one.distance(&p_two));
}

#[test]
fn test_distance() {
    let p_one = Point { x: 1, y: 1 };
    let p_two = Point { x: 12, y: 2 };
    assert_eq!(11.045361f32, p_one.distance(&p_two));
}

#[test]
fn test_distance_between_same_point() {
    let p_one = Point { x: 1, y: 1 };
    let p_two = Point { x: 1, y: 1 };
    assert_eq!(0f32, p_one.distance(&p_two));
}
