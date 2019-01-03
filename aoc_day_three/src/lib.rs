use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::string::String;

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    top: u32,
    left: u32,
}

impl Point {
    fn new(top: u32, left: u32) -> Point {
        Point { top, left }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn new(width: u32, height: u32) -> Size {
        Size { width, height }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Claim {
    start: Point,
    end: Point,
    size: Size,
}

impl Claim {
    fn new(start: Point, size: Size) -> Claim {
        let end = Point::new(start.top + size.height, start.left + size.width);
        Claim { start, end, size }
    }

    fn point_inside(&self, point: Point) -> bool {
        self.start.left < point.left && point.left < self.end.left && self.start.top < point.top && point.top < self.end.top
    }

    fn area(&self) -> u32 {
        self.size.width * self.size.height
    }

    fn intersect(left: Claim, right: Claim) -> Option<Claim> {
        let left_x: u32 = max(left.start.left, right.start.left);
        let right_x: u32 = min(left.end.left, right.end.left);
        let top_y: u32 = max(left.start.top, right.start.top);
        let bottom_y: u32 = min(left.end.top, right.end.top);

        if left_x < right_x && top_y < bottom_y {
            return Some(Claim::new(Point::new(top_y, left_x), Size::new(bottom_y - top_y, right_x - left_x)));
        }
        None
    }
}

//pub fn find_overlapping_claims_area(claims: &HashMap<u32, Claim>) -> u64 {
//    let claim_iter = claims.iter();
//    for (index, claim) in claim_iter {
//        claim_iter.skip(index + 1).find_map(||)
//    }
//    4
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_intersection() {
        let claim_first = Claim::new(Point::new(0, 0), Size::new(2, 2));
        let claim_second = Claim::new(Point::new(1, 1), Size::new(2, 2));
        assert_eq!(Some(Claim::new(Point::new(1, 1), Size::new(1, 1))), Claim::intersect(claim_first, claim_second));
    }

//    #[test]
//    fn it_finds_all_overlapping_claims() {
//        let mut example_input: HashMap<u32, Claim> = HashMap::new();
//
//        example_input.insert(1, Claim::new(Point::new(1, 3), Size::new(4, 4)));
//        example_input.insert(2, Claim::new(Point::new(1, 3), Size::new(4, 4)));
//        example_input.insert(3, Claim::new(Point::new(5, 5), Size::new(2, 2)));
//
//
//        assert_eq!(find_overlapping_claims_area(&example_input), 4);
//    }
}


