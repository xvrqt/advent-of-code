/* Standard Library */
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::path::Path;
use std::convert::TryFrom;
use std::collections::HashSet;

/* Represent the directions so things don't get stringy */
pub enum WIRE {
    UP(i64),
    DOWN(i64),
    LEFT(i64),
    RIGHT(i64),
}

/* Open and read file to string, break up by lines */
fn read_file<P: AsRef<Path>>(path: P) -> (String, String) {
    let mut file = File::open(path).unwrap();
    let mut s = String::with_capacity(1000);
    file.read_to_string(&mut s).unwrap();

    let line1 = s.lines().next().unwrap().to_owned();
    let line2 = s.lines().next().unwrap().to_owned();

    (line1, line2)
}

/* Parses a wire into a Vec of WIRE enums */
fn parse_string_to_wire(wire: String) -> Vec<WIRE> {
    /* Parse each wire, comma separated values */
    wire.split(",").filter_map(|s| {
        let (direction, distance) = s.split_at(1);
        distance.parse().ok().and_then(|d| {
            match direction {
                "U" => Some(WIRE::UP(d)),
                "D" => Some(WIRE::DOWN(d)),
                "L" => Some(WIRE::LEFT(d)),
                "R" => Some(WIRE::RIGHT(d)),
                _   => panic!()
            }
        })
        
    }).collect()
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
        }
    }

    /* Takes a WIRE direction and returns a Vec of positions between this
     * position and the end of the wire. Terrible name.
    */
    fn new_position(&self, delta: WIRE) -> Vec<Self> {
        /* Convenience shadowing */
        let x = self.x;
        let y = self.y;
        let difference = match delta {
            WIRE::UP(d)    => d,
            WIRE::DOWN(d)  => d,
            WIRE::LEFT(d)  => d, 
            WIRE::RIGHT(d) => d, 
        };

        /* Allocate the return vector */
        let num_elements = usize::try_from(difference).unwrap();
        let pos_size = size_of::<Position>();
        let mut positions = Vec::with_capacity(num_elements * pos_size) ;

        for i in 1..=difference {
            let i = i64::try_from(i).unwrap();
            let pos = match delta {
                WIRE::UP(_)    => Self { x: x,     y: y + i },
                WIRE::DOWN(_)  => Self { x: x,     y: y - i },
                WIRE::LEFT(_)  => Self { x: x - i, y: y },
                WIRE::RIGHT(_) => Self { x: x + i, y: y },
            };
            positions.push(pos);
        }
        positions
    }

    pub fn distance_from_origin(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

/* Takes a wire and returns a set of positions the wire passed through */
fn get_positions(wire: Vec<WIRE>) -> HashSet<Position> {
    let mut set = HashSet::new();     
    let mut current = Position::new(0,0);
    for x in wire {
       let mut positions = current.new_position(x); 
       current = positions[positions.len() - 1];
       for p in positions {
           set.insert(p);
       }
    }

    set
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(w1: String, w2: String) -> i64 {
        let wire1 = parse_string_to_wire(w1);
        let wire2 = parse_string_to_wire(w2);

        let positions1 = get_positions(wire1);
        let positions2 = get_positions(wire2);


        let intersection = positions1.intersection(&positions2);

        let mut closest = Position::new(1000000,1000000);
        for pos in intersection {
           if pos.distance_from_origin() < closest.distance_from_origin() {
                closest = *pos;
           }
        }

        closest.distance_from_origin()
    }

    #[test]
    fn test0() {
        let w1 = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2 = String::from("U62,R66,U55,R34,D71,R55,D58,R83");
        let distance = helper(w1, w2);
        assert_eq!(distance, 159);
    }

    #[test]
    fn test1() {
        let w1 = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let w2 = String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let distance = helper(w1, w2);
        assert_eq!(distance, 135);
    }
}
