use regex::Regex;

use crate::aoc::load_lines;

pub fn a() -> usize {
    _a(load_lines(15), 2_000_000)
}

pub fn b() -> isize {
    _b(load_lines(15), 4_000_000)
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    sensor: Point,
    beacon: Point,
    dist: isize,
}

impl Sensor {
    fn in_range(self: &Self, point: Point) -> Option<isize> {
        if point.y < self.sensor.y - self.dist || point.y > self.sensor.y + self.dist {
            return None
        }

        let diff = self.dist - (self.sensor.y.abs_diff(point.y) as isize);
        let upto = self.sensor.x + diff;
        if point.x >= self.sensor.x - diff && point.x <= upto {
            Some(upto)
        } else {
            None
        }
    }

    fn is_clear(self: &Self, point: Point) -> bool {
        if point.y < self.sensor.y - self.dist || point.y > self.sensor.y + self.dist {
            return false;
        }

        if point.x == self.beacon.x && point.y == self.beacon.y {
            return false;
        }

        let diff = self.dist - (self.sensor.y.abs_diff(point.y) as isize);

        point.x >= self.sensor.x - diff && point.x <= self.sensor.x + diff
    }
}

fn parse_sensors(lines: &Vec<String>) -> (Vec<Sensor>, Point, Point) {
    let mut out = Vec::new();
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let dist = |s: &Sensor| s.sensor.x.abs_diff(s.beacon.x) + s.sensor.y.abs_diff(s.beacon.y);

    let (mut min_p, mut max_p) = (Point { x: 0, y: 0 }, Point { x: 0, y: 0 });
    for (i, line) in lines.iter().enumerate() {
        let captures = re.captures(&line).unwrap();
        let mut sensor = Sensor {
            sensor: Point {
                x: captures.get(1).unwrap().as_str().parse().unwrap(),
                y: captures.get(2).unwrap().as_str().parse().unwrap(),
            },
            beacon: Point {
                x: captures.get(3).unwrap().as_str().parse().unwrap(),
                y: captures.get(4).unwrap().as_str().parse().unwrap(),
            },
            dist: 0,
        };
        let dist = dist(&sensor) as isize;
        sensor.dist = dist;
        if i == 0 || sensor.sensor.x - dist < min_p.x {
            min_p.x = sensor.sensor.x - dist;
        }
        if i == 0 || sensor.sensor.x + dist > max_p.x {
            max_p.x = sensor.sensor.x + dist;
        }
        if i == 0 || sensor.sensor.y - dist < min_p.y {
            min_p.y = sensor.sensor.y - dist;
        }
        if i == 0 || sensor.sensor.y + dist > max_p.y {
            max_p.y = sensor.sensor.y + dist;
        }
        out.push(sensor);
    }
    (out, min_p, max_p)
}

fn _a(lines: Vec<String>, target_y: isize) -> usize {
    let (sensors, min, max) = parse_sensors(&lines);

    let mut count = 0;
    let y = target_y;
    'x: for x in min.x..max.x {
        let point = Point { x, y };
        for sensor in sensors.iter() {
            if sensor.is_clear(point) {
                count += 1;
                continue 'x;
            }
        }
    }

    count
}

fn _b(lines: Vec<String>, max: isize) -> isize {
    let (sensors, _, _) = parse_sensors(&lines);
    let tune_freq = 4_000_000;

    let mut beacon = Point { x: 0, y: 0 };
    'y: for y in 0..=max {
        let mut x = 0;
        'x: while x <= max {
            let point = Point{x, y};
            for sensor in sensors.iter() {
                if let Some(upto) = sensor.in_range(point) {
                    x = upto+1;
                    continue 'x;
                }
            }
            beacon = point;
            break 'y;
        }
    }

    beacon.x * tune_freq + beacon.y
}

#[cfg(test)]
mod tests {
    use crate::aoc::load_lines_suffix;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(26, _a(load_lines_suffix(15, "_test"), 10))
    }

    #[test]
    fn a2() {
        assert_eq!(4665948, _a(load_lines(15), 2_000_000))
    }

    #[test]
    fn b() {
        assert_eq!(56000011, _b(load_lines_suffix(15, "_test"), 20))
    }

    #[test]
    fn b2() {
        assert_eq!(13543690671045, _b(load_lines(15), 4_000_000))
    }
}
