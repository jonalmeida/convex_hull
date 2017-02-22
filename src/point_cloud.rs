use ::point::Point;

/*
fn furthest_points(cloud: &Vec<Point>) -> (Point, Point) {
    let size = cloud.iter().count();
    let mut distance = 0f32;
    let mut one = Point::new(0, 0);
    let mut two = Point::new(0, 0);
    for x in 0..size {
        for y in 0..size {
            let first  = cloud.iter().nth(x).unwrap();
            let second = cloud.iter().nth(y).unwrap();
            let compare = first.distance(&second);
            if distance > compare {
                one = first.clone();
                two = second.clone();
                distance = compare;
            }
        }
    }
    (one, two)
}
*/

fn furthest_points(cloud: &mut Vec<Point>) -> (Point, Point) {
    cloud.sort();
    cloud.dedup();
    for p in cloud.iter().enumerate().flat_map(|(i, a)| cloud[i+1..].iter().map(move |b| (a, b))) {
        println!("{:?}", p);
    }
    (Point::new(0,0), Point::new(0,0))
}

#[test]
fn test_furthest_points() {
}
