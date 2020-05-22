use std::collections::HashMap;
use std::convert::TryFrom;

const ORIGIN: Point = Point(0, 0);

#[derive(PartialEq, Eq, Hash)]
struct Point(i32, i32);

pub struct Wire {
    directions: HashMap<Point, u32>,
}

impl<'a> Wire {
    pub fn closet_intersection(&self, other_wire: &Self) -> Option<u32> {
        let (smaller, larger) = self.order(&other_wire);

        let mut closet = None;

        for key in smaller.directions.keys() {
            if larger.directions.contains_key(key) {
                let distance = manhattan_distance(key, &ORIGIN);
                match closet {
                    Some(x) if distance < x => closet = Some(distance),
                    None => closet = Some(distance),
                    _ => (),
                }
            }
        }

        closet
    }

    pub fn fewest_steps(&self, other_wire: &Self) -> Option<u32> {
        let (smaller, larger) = self.order(&other_wire);

        let mut fewest = None;

        for key in smaller.directions.keys() {
            if let Some(larger_steps) = larger.directions.get(key) {
                let steps_taken = smaller.directions.get(key).unwrap() + larger_steps;
                match fewest {
                    Some(x) if steps_taken < x => fewest = Some(steps_taken),
                    None => fewest = Some(steps_taken),
                    _ => (),
                }
            }
        }

        fewest
    }

    fn order(&'a self, other_wire: &'a Self) -> (&'a Self, &'a Self) {
        if self.directions.len() < other_wire.directions.len() {
            (self, other_wire)
        } else {
            (other_wire, self)
        }
    }
}

impl TryFrom<String> for Wire {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut current = Point(ORIGIN.0, ORIGIN.1);

        let mut directions = HashMap::new();

        let mut steps = 0;

        for direction_str in value.split(",") {
            let direction = Direction::try_from(direction_str)?;

            let (x, y) = match direction.orientation {
                Orientation::U => (0, -1),
                Orientation::D => (0, 1),
                Orientation::L => (-1, 0),
                Orientation::R => (1, 0),
            };

            for _ in 0..direction.iterations {
                steps += 1;
                current.0 += x;
                current.1 += y;
                directions
                    .entry(Point(current.0, current.1))
                    .or_insert(steps);
            }
        }

        Ok(Wire { directions })
    }
}

#[derive(PartialEq, Debug)]
struct Direction {
    orientation: Orientation,
    iterations: u32,
}
impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let orientation = Orientation::try_from(&value[..1])?;
        let iterations = match value[1..].parse::<u32>() {
            Ok(iterations) => iterations,
            _ => {
                return Err(format!(
                    "Unable to parse iterations of Direction: {}",
                    value
                ))
            }
        };

        Ok(Direction {
            orientation,
            iterations,
        })
    }
}

#[derive(PartialEq, Debug)]
enum Orientation {
    U,
    D,
    L,
    R,
}
impl TryFrom<&str> for Orientation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Orientation::U),
            "D" => Ok(Orientation::D),
            "L" => Ok(Orientation::L),
            "R" => Ok(Orientation::R),
            _ => Err(format!("Unknown Orientation Type for Direction: {}", value)),
        }
    }
}

fn manhattan_distance(coord_1: &Point, coord_2: &Point) -> u32 {
    ((coord_1.0 - coord_2.0).abs() + (coord_1.1 - coord_2.1).abs()) as u32
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_closet() {
        let wire_1 = Wire::try_from("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string()).unwrap();
        let wire_2 = Wire::try_from("U62,R66,U55,R34,D71,R55,D58,R83".to_string()).unwrap();
        let expected = 159;
        assert_eq!(wire_1.closet_intersection(&wire_2).unwrap(), expected);

        let wire_1 =
            Wire::try_from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string()).unwrap();
        let wire_2 = Wire::try_from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()).unwrap();
        let expected = 135;
        assert_eq!(wire_1.closet_intersection(&wire_2).unwrap(), expected);
    }

    #[test]
    fn test_fewest() {
        let wire_1 = Wire::try_from("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string()).unwrap();
        let wire_2 = Wire::try_from("U62,R66,U55,R34,D71,R55,D58,R83".to_string()).unwrap();
        let expected = 610;
        assert_eq!(wire_1.fewest_steps(&wire_2).unwrap(), expected);

        let wire_1 =
            Wire::try_from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string()).unwrap();
        let wire_2 = Wire::try_from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()).unwrap();
        let expected = 410;
        assert_eq!(wire_1.fewest_steps(&wire_2).unwrap(), expected);
    }

    #[test]
    fn test_direction_parse() {
        let direction_str = "D127";
        let orientation = Orientation::D;
        let iterations = 127;
        let expected = Direction {
            orientation,
            iterations,
        };
        let actual = Direction::try_from(direction_str).unwrap();

        assert_eq!(actual, expected);

        let direction_str = "Q100";
        let actual = Direction::try_from(direction_str);
        assert_eq!(
            actual,
            Err("Unknown Orientation Type for Direction: Q".to_string())
        );

        let direction_str = "UP100";
        let actual = Direction::try_from(direction_str);
        assert_eq!(
            actual,
            Err("Unable to parse iterations of Direction: UP100".to_string())
        );
    }
}
