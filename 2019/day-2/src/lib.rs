/* Intcode Computer */
pub struct IntcodeMachine {
    pc: usize,
    bin: Vec<usize>,
    instr_ptr: usize,
}

#[derive(Debug)]
pub enum OPCODE {
    ADD(usize, usize, usize),
    MULTI(usize, usize, usize),
    HALT,
}

impl IntcodeMachine {
    pub fn new(bin: Vec<usize>) -> Self {
        IntcodeMachine { 
            pc: 0,
            bin,
            instr_ptr: 0,
        }
    }

    /* Performs the next opcode instruction. Mutates the bin. */
    pub fn step(&mut self) -> Result<OPCODE, String> {
        if let Some(opcode) = self.bin.get(self.instr_ptr) {
            let result = match opcode {
                1 => {
                    let (input1, input2, output_ptr) = self.get_full_opcode()?;
                    self.bin[output_ptr] = input1 + input2;
                    Ok(OPCODE::ADD(input1, input2, output_ptr))
                },
                2 => {
                    let (input1, input2, output_ptr) = self.get_full_opcode()?;
                    self.bin[output_ptr] = input1 * input2;
                    Ok(OPCODE::MULTI(input1, input2, output_ptr))
                },
                99 => {
                    return Ok(OPCODE::HALT);
                }
                _ => {
                    let error_msg = format!("Unknown OP code '{}' at position {}.",
                                            opcode, self.instr_ptr);
                    return Err(error_msg);
                }
            };

            /* Increment the program counter by 1 and the pointer by 4 */
            self.pc += 1;
            self.instr_ptr += 4;
            result

        } else {
            /* We somehow over indexed. Perhaps bad input? */
            let error_msg = format!("Attempted to read opcode at position {} \
                                     which does not exist!", self.instr_ptr);
            Err(error_msg)
        }
    }

    /* Returns the value of the INTCODE at the specified position */
    pub fn get_intcode(&self, position: usize) -> Option<usize> {
        self.bin.get(position).cloned()
    }

    /* Returns a copy of the input */
    pub fn get_state(&self) -> Vec<usize> {
        self.bin.clone()
    }

    /* Convenience Functions */
    fn get_full_opcode(&self) -> Result<(usize, usize, usize), String> {
        let start = self.instr_ptr + 1;
        let end   = self.instr_ptr + 4;
        let opcode = self.bin.get(start..end);

        if let Some(opcode) = opcode {
            let input1 = self.bin[opcode[0]];
            let input2 = self.bin[opcode[1]];
            let output_ptr = opcode[2];
            Ok((input1, input2, output_ptr))
        } else {
            let error_msg = format!("Attempted to grab arguments for opcode {} \
                                     at position {} but arguments do not exist!",
                                     self.pc, self.instr_ptr);
            Err(error_msg)
        
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn helper(input: Vec<usize>, answer: Vec<usize>) {
        let mut computer = IntcodeMachine::new(input);
        loop {
            match computer.step() {
                Ok(OPCODE::HALT) => {
                    assert_eq!(computer.get_state(), answer);
                    break;
                },
                Err(string) => {
                    println!("{}", string);
                    panic!();
                },
                Ok(code) => { println!("{:?}", code); } 
            }
        }

    }

    #[test]
    fn test0() {
        helper(vec![1,0,0,0,99], vec![2,0,0,0,99]);
    }

    #[test]
    fn test1() {
        helper(vec![2,3,0,3,99], vec![2,3,0,6,99]);
    }

    #[test]
    fn test2() {
        helper(vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test3() {
        helper(vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]);
    }
}
