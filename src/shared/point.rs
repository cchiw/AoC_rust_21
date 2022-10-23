use std::cmp;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x: x, y: y }
    }
    pub fn eq(p1: &Point, p2: &Point) -> bool {
        p1.x == p2.x && p1.y == p2.y
    }
    pub fn makeline(p1: &Point, p2: &Point, v: &mut Vec<Point>, is_diag: bool) {
        if p1.x == p2.x {
            let mut current = cmp::min(p1.y, p2.y);
            let end = cmp::max(&p1.y, &p2.y);
            while current <= *end {
                let p = Point::new(p1.x, current);
                v.push(p);
                current += 1;
            }
        } else if p1.y == p2.y {
            let mut current = cmp::min(p1.x, p2.x);
            let end = cmp::max(&p1.x, &p2.x);
            while current <= *end {
                let p = Point::new(current, p1.y);
                v.push(p);
                current += 1;
            }
        } else if is_diag {
            let mut x_current = p1.x;
            let mut y_current = p1.y;
            let x_end = &p2.x;
            let x_inc = if p1.x < p2.x { 1 } else { -1 };
            let y_inc = if p1.y < p2.y { 1 } else { -1 };

            if p1.x < p2.x {
                while x_current <= *x_end {
                    let p = Point::new(x_current, y_current);
                    v.push(p);
                    x_current += x_inc;
                    y_current += y_inc;
                }
            } else {
                while x_current >= *x_end {
                    let p = Point::new(x_current, y_current);
                    v.push(p);
                    x_current += x_inc;
                    y_current += y_inc;
                }
            }
        }
        //println!("v {:?}", v);
    }
}
