/* Our Crate */
use day_2::IntcodeMachine;

/* Standard Library */
use std::fs::File;
use std::io::Read;

#[test]
/* Takes the input and calculates the final result of position 0 */
fn determine_inputs() -> Result<(), String> {
    /* Read and parse the file */
    let mut file = File::open("input.txt").unwrap();
    let mut s = String::with_capacity(100);
    file.read_to_string(&mut s).unwrap();
    let input: Vec<usize> = s.split(",")
                             .filter_map(|s| s.parse().ok())
                             .collect();

    let mut computer = IntcodeMachine::new(input);

    for i in 0..100 {
        for j in 0..100 {
            match computer.run_with_inputs(i, j)? {
                19690720 => println!("Noun: {}\nVerb: {}", i, j),
                _ => ()
            }
        }
    }

    Ok(())
}
