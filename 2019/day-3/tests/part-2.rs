/* Our Crate */
use day_3::*;

/* Standard Library */
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[test]
fn calc_shortest_wire_crossing() {
    let (wire1, wire2) = read_file("input.txt");

    let wire1 = parse_string_to_wire(wire1);
    let wire2 = parse_string_to_wire(wire2);

    let mut positions1 = get_positions(wire1);
    let mut positions2 = get_positions(wire2);

    let origin = (0, 0);
    positions1.remove(&origin);
    positions2.remove(&origin);

    let mut closest = 1000000;
    for (key, val) in positions1 {
        if let Some(steps) = positions2.get(&key) {
            if val + steps < closest { closest = val + steps; }
        }
    }
    println!("Shortest crossing is at: {}", closest);
}

#[test]
fn shortest_crossing_1() {
    let wire1 = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = String::from("U62,R66,U55,R34,D71,R55,D58,R83");

    let wire1 = parse_string_to_wire(wire1);
    let wire2 = parse_string_to_wire(wire2);

    let mut positions1 = get_positions(wire1);
    let mut positions2 = get_positions(wire2);

    let mut closest = 1000000;
    for (key, val) in positions1 {
        if let Some(steps) = positions2.get(&key) {
            if val + steps < closest { closest = val + steps; }
        }
    }
    assert_eq!(closest, 610);
}

#[test]
fn shortest_crossing_2() {
    let wire1 = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

    let wire1 = parse_string_to_wire(wire1);
    let wire2 = parse_string_to_wire(wire2);

    let mut positions1 = get_positions(wire1);
    let mut positions2 = get_positions(wire2);

    let mut closest = 1000000;
    for (key, val) in positions1 {
        if let Some(steps) = positions2.get(&key) {
            if val + steps < closest { 
                closest = val + steps;
            }
        }
    }
    assert_eq!(closest, 410);
}


fn get_positions(wires: Vec<WIRE>) -> HashMap<(i64, i64), i64> {
    let mut current_pos = (0,0);
    let mut steps = 0;

    let mut positions = HashMap::new();

    for wire in wires {
        let delta = match wire {
            WIRE::UP(d) => d,
            WIRE::DOWN(d) => d,
            WIRE::LEFT(d) => d,
            WIRE::RIGHT(d) => d,
        };

        for i in 1..=delta {
            let (x, y) = current_pos;
            let current_pos = match wire {
                WIRE::UP(_)    => (x, y + 1),
                WIRE::DOWN(_)  => (x, y - 1),
                WIRE::LEFT(_)  => (x - 1, y),
                WIRE::RIGHT(_) => (x + 1, y),
            };
            steps += 1;
            if positions.get(&current_pos) == None {
                positions.insert(current_pos, steps);
            }
        }
    }
    println!("{}", steps);
    positions
}
