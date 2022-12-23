use std::collections::HashSet;

use intervals::{Interval, IntervalSet};
use itertools::Itertools;
use lines::{Line, Point};
use text_io::scan;

mod intervals;
mod lines;

struct Sensor {
    pos: (i32, i32),
    range: i32,
}

impl Sensor {
    fn parse(line: &str) -> (Self, (i32, i32)) {
        let x: i32;
        let y: i32;
        let beacon_x: i32;
        let beacon_y: i32;
        scan!(line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", x, y, beacon_x, beacon_y);

        let sensor = Self {
            pos: (x, y),
            range: (x - beacon_x).abs() + (y - beacon_y).abs(),
        };

        (sensor, (beacon_x, beacon_y))
    }

    fn y_interval(&self, y: i32) -> Interval {
        let diff = (y - self.pos.1).abs();
        if diff <= self.range {
            Interval::new(
                self.pos.0 - self.range + diff,
                self.pos.0 + self.range - diff,
            )
        } else {
            Interval::empty()
        }
    }

    fn exterior(&self) -> [Line; 4] {
        // make sure to not duplicate points (otherwise they would be checked multiple times)
        [
            Line::new(
                (self.pos.0, self.pos.1 - self.range - 1),
                (self.pos.0 + self.range, self.pos.1 - 1),
            ),
            Line::new(
                (self.pos.0 - 1, self.pos.1 - self.range),
                (self.pos.0 - self.range - 1, self.pos.1),
            ),
            Line::new(
                (self.pos.0 - self.range, self.pos.1 + 1),
                (self.pos.0, self.pos.1 + self.range + 1),
            ),
            Line::new(
                (self.pos.0 + self.range + 1, self.pos.1),
                (self.pos.0 + 1, self.pos.1 + self.range),
            ),
        ]
    }

    fn covers(&self, p: Point) -> bool {
        self.distance(p) <= self.range
    }

    fn distance(&self, p: Point) -> i32 {
        (p.0 - self.pos.0).abs() + (p.1 - self.pos.1).abs()
    }
}

fn find_uncovered_point(sensors: &[Sensor], size: i32) -> Option<Point> {
    // instead of checking all points in the square if they are not covered by any sensor, we can restrict the search
    // to a few candidate points using the assumption that there at most one solution
    // this single point must either be a corner of the square or it must be next to the covered area of two different
    // sensors (i.e., it must be on the exterior of two different sensors), because each point that is not in the corner
    // has at least 5 neighbors that are also within the square (even if the point is on the edge), which all must be
    // covered by a sensor, otherwise the point would not be the only solution, but the exterior of one sensor can only
    // cover at most three neighbors of a point (if the point is not covered by the sensor)
    // thus, the possible candidates are only the points in the corners of the square or the intersection points of
    // exteriors of the sensors
    let mut candidates = HashSet::new();
    candidates.extend(&[(0, 0), (0, size), (size, 0), (size, size)]);
    candidates.extend(
        sensors
            .iter()
            .flat_map(|s| s.exterior())
            .tuple_combinations()
            .flat_map(|(e1, e2)| e1.intersections(&e2)),
    );

    candidates.into_iter().find(|&p| {
        p.0 >= 0 && p.0 <= size && p.1 >= 0 && p.1 <= size && !sensors.iter().any(|s| s.covers(p))
    })
}

fn main() {
    let input = include_str!("../input");

    let mut intervals = IntervalSet::new();
    let mut beacons = HashSet::new();
    for line in input.lines() {
        let (sensor, beacon) = Sensor::parse(line);
        if beacon.1 == 2000000 {
            beacons.insert(beacon);
        }

        intervals.insert(sensor.y_interval(2000000));
    }

    println!("{}", intervals.area() - beacons.len() as i32);

    let mut sensors = Vec::new();
    for line in input.lines() {
        let (sensor, _) = Sensor::parse(line);
        sensors.push(sensor);
    }

    if let Some(p) = find_uncovered_point(&sensors, 4000000) {
        println!("{:?} {}", p, p.0 as i64 * 4000000 + p.1 as i64);
    } else {
        println!("no uncovered point found");
    }
}
