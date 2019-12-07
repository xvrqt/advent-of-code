/* Our Crate */
use day_3::*;

/* Standard Library */
use std::fs::File;
use std::io::Read;

#[test]
fn calc_nearest_wire_crossing() {
    let (wire1, wire2) = read_file("input.txt");

    let wire1 = parse_string_to_wire(wire1);
    let wire2 = parse_string_to_wire(wire2);

    let mut positions1 = get_positions(wire1);
    let mut positions2 = get_positions(wire2);

    let origin = Position::new(0, 0);
    positions1.remove(&origin);
    positions2.remove(&origin);

    let intersection = positions1.intersection(&positions2);

    let mut closest = Position::new(1000000, 1000000);
    for pos in intersection {
        if pos.distance_from_origin() < closest.distance_from_origin() {
            closest = *pos;
        }
    }
    println!("{:?}", closest);
    return;

    let result = closest.distance_from_origin();
    println!("Closet crossing is at: {}", result);
}
