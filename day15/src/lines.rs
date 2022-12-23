pub type Point = (i32, i32);

// a line with integer coordinates and a slope of 45 or -45 degrees
#[derive(Clone, Copy)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        // ensure that the line is oriented in positive x direction
        if p2.0 < p1.0 {
            Self { start: p2, end: p1 }
        } else {
            Self { start: p1, end: p2 }
        }
    }

    pub fn intersections(&self, other: &Self) -> Vec<Point> {
        if self.start == other.start {
            return vec![self.start];
        }

        let (top, bottom) = if self.start.1 > other.start.1 {
            (self, other)
        } else {
            (other, self)
        };

        if top.slope() != -1 || bottom.slope() != 1 {
            return vec![];
        }

        let steps = top.start.1 - bottom.start.1 + bottom.start.0 - top.start.0;
        if steps > top.max_steps() * 2 {
            return vec![];
        }

        let p1 = top.nth_point(steps / 2);
        if p1.0 < bottom.start.0 || p1.0 > bottom.end.0 {
            return vec![];
        }

        if steps % 2 == 0 {
            vec![p1]
        } else {
            let p2 = top.nth_point(steps / 2 + 1);
            vec![p1, p2]
        }
    }

    fn slope(&self) -> i32 {
        (self.end.1 - self.start.1).signum()
    }

    fn nth_point(&self, n: i32) -> Point {
        (self.start.0 + n, self.start.1 + self.slope() * n)
    }

    fn max_steps(&self) -> i32 {
        self.end.0 - self.start.0
    }
}
