/* Our Crate */
use day_2::{IntcodeMachine, OPCODE};

/* Standard Library */
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[test]
/* Takes the input and calculates the final result of position 0 */
fn calc_position_0() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut s = String::with_capacity(100);
    file.read_to_string(&mut s)?;
    let mut input: Vec<usize> = s.split(",").map(|s| {
        println!("{}", s);
        s.parse::<usize>().unwrap_or_else(|_| 0)
    }).collect();

    input[1] = 12;
    input[2] = 2;

    let mut computer = IntcodeMachine::new(input);
    loop {
        match computer.step() {
            Ok(OPCODE::HALT) => {
                break;
            },
            Err(string) => {
                println!("{}", string);
                panic!();
            },
            Ok(code) => { println!("{:?}", code); } 
        }
    }

    if let Some(pos_0) = computer.get_intcode(0) {
        println!("OPCODE @ POSITION 0: {}", pos_0);
    }

    Ok(())
}
