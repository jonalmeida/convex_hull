use ::point::Point;

fn furthest_points(cloud: &Vec<Point>, sorted: bool) -> (Point, Point) {
    // TODO: Write more tests: if vec is empty, if already sorted, edge cases, vec of only 1 item
    if cloud.len() < 2 {
        panic!("There are less that two points to compare.");
    }
    let mut max_distance = 0_f32;
    let mut max_points: (Point, Point) = (Point::new(-1, -1), Point::new(-1, -1));
    let mut sorted_cloud = cloud.clone();
    if !sorted {
        sorted_cloud.sort();
    }
    sorted_cloud.dedup();
    for p in sorted_cloud.iter().enumerate().flat_map(|(i, a)| sorted_cloud[i+1..].iter().map(move |b| (a, b))) {
        let dist = p.0.distance(&p.1);
        if max_distance < dist {
            max_distance = dist;
            max_points = (p.0.clone(), p.1.clone());
            //println!("possible: {:?}, {:?}", p, dist);
        }
    }
    //println!("Max dist: {}, pair: {:?}", max_distance, max_points);
    max_points
}

#[test]
fn test_furthest_points() {
    let vec = vec![Point::new(624, 176), Point::new(476, 2232), Point::new(912, 218),
        Point::new(708, 2385), Point::new(337, 779), Point::new(134, 358),
        Point::new(96, 1241), Point::new(1442, 1141), Point::new(706, 368),
        Point::new(1037, 781), Point::new(2421, 1708), Point::new(1945, 2149)];
    let (first, second) = furthest_points(&vec, false);
    assert_eq!(first, Point::new(134, 358));
    assert_eq!(second, Point::new(2421, 1708));
}
