/* Our Crate */
use day_2::IntcodeMachine;

/* Standard Library */
use std::fs::File;
use std::io::Read;

#[test]
/* Takes the input and calculates the final result of position 0 */
fn rebuild_intcode_computer() -> Result<(), String> {
    /* Read and parse the file */
    let mut file = File::open("input.txt").unwrap();
    let mut s = String::with_capacity(100);
    file.read_to_string(&mut s).unwrap();
    let input: Vec<usize> = s.split(",")
                             .filter_map(|s| s.parse().ok())
                             .collect();

    let result = IntcodeMachine::new(input).run_with_inputs(12, 2)?;
    println!("OPCODE @ POSITION 0: {}", result);

    Ok(())
}
