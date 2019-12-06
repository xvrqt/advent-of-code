/* Our Crate */
use day_2::{IntcodeMachine, OPCODE};

/* Standard Library */
use std::fs::File;
use std::io::{self, prelude::*};

#[test]
/* Takes the input and calculates the final result of position 0 */
fn calc_position_0() -> Result<(), String> {
    let mut file = File::open("input.txt").map_err(|e| e.to_string())?;
    let mut s = String::with_capacity(100);
    file.read_to_string(&mut s).map_err(|e| e.to_string())?;
    let mut input: Vec<usize> = s.split(",").filter_map(|s| s.parse().ok())
                                            .collect();

    /* Set Noun and Verb */
    input[1] = 12;
    input[2] = 2;

    let result = IntcodeMachine::new(input).run()?;
    println!("OPCODE @ POSITION 0: {}", result);

    Ok(())
}
