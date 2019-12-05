/* Our Crate */
use day_1::calc_fuel;

/* Standard Library */
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[test]
fn calc_launch_fuel() -> io::Result<()> {
    let file = File::open("component_mass_input.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel: usize = 0;
    for line in reader.lines() {
        let mass = line.unwrap().parse::<usize>().unwrap();
        total_fuel += calc_fuel(mass);
    }
    println!("Total Fuel Required: {}", total_fuel);

    Ok(())
}
