/* Standard Library */
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::size_of;
use std::path::Path;
use std::convert::TryFrom;
use std::collections::HashSet;

/* Represent the directions so things don't get stringy */
#[derive(Debug)]
pub enum WIRE {
    UP(i64),
    DOWN(i64),
    LEFT(i64),
    RIGHT(i64),
}

/* Open and read file to string, break up by lines */
pub fn read_file<P: AsRef<Path>>(path: P) -> (String, String) {
    let file = File::open(path).unwrap();
    let mut buffer = BufReader::new(file);

    let mut line1 = String::with_capacity(1000);
    let mut line2 = String::with_capacity(1000);

    let _ = buffer.read_line(&mut line1);
    let _ = buffer.read_line(&mut line2);

    println!("{}", line2);

    (line1, line2)
}

/* Parses a wire into a Vec of WIRE enums */
pub fn parse_string_to_wire(wire: String) -> Vec<WIRE> {
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
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
        }
    }

    /* Takes a WIRE direction and returns a Vec of positions between this
     * position and the end of the wire. Terrible name.
    */
    pub fn new_position(&self, delta: WIRE) -> Vec<Self> {
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

        for i in 0..=difference {
            let i = i64::try_from(i).unwrap();
            let pos = match delta {
                WIRE::UP(_)    => Self { x: x,     y: y + i },
                WIRE::DOWN(_)  => Self { x: x,     y: y - i },
                WIRE::LEFT(_)  => Self { x: x - i, y: y },
                WIRE::RIGHT(_) => Self { x: x + i, y: y },
            };
            let test = Self { x: 2, y:0 };
            if pos == test {
                println!("{:?}", delta);
            }
            positions.push(pos);
        }
        positions
    }

    pub fn distance_from_origin(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

/* Takes a wire and returns a set of positions the wire passed through */
pub fn get_positions(wire: Vec<WIRE>) -> HashSet<Position> {
    let mut set = HashSet::new();     
    let mut current = Position::new(0,0);
    for x in wire {
       let mut positions = current.new_position(x); 
       current = positions.pop().unwrap();
       set.insert(current);
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

        let mut positions1 = get_positions(wire1);
        let mut positions2 = get_positions(wire2);

        let origin = Position::new(0,0);
        positions1.remove(&origin);
        positions2.remove(&origin);

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

    #[test]
    fn test2() {
        let w1 = String::from("R8,U5,L5,D3");
        let w2 = String::from("U7,R6,D4,L4");
        let distance = helper(w1, w2);
        assert_eq!(distance, 6);
    }

    #[test]
    fn test3() {
        let w1 = String::from("L1,D1");
        let w2 = String::from("D1,L1");
        let distance = helper(w1, w2);
        assert_eq!(distance, 2);
    }

    #[test]
    fn test4() {
        let w1 = String::from("R8,U8,R8,U8");
        let w2 = String::from("U8,R8,U8,R8");
        let distance = helper(w1, w2);
        assert_eq!(distance, 16);
    }

    #[test]
    fn test5() {
        let w1 = String::from("R8,U1,R8,U1");
        let w2 = String::from("U2,R16,D1");
        let distance = helper(w1, w2);
        assert_eq!(distance, 17);
    }

    #[test]
    fn test6() {
        let w1 = String::from("R8,U1,R8,U1");
        let w2 = String::from("U2,R16");
        let distance = helper(w1, w2);
        assert_eq!(distance, 18);
    }

    #[test]
    fn test7() {
        let w1 = String::from("R8");
        let w2 = String::from("R16");
        let distance = helper(w1, w2);
        assert_eq!(distance, 1);
    }

    #[test]
    fn test8() {
        let w1 = String::from("U1,R8,L16");
        let w2 = String::from("D16,L8,U17");
        let distance = helper(w1, w2);
        assert_eq!(distance, 9);
    }
}
