use std::{
    env, error, fs,
    io::{self, Read, Write},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;

    // "b" is for bracket
    let mut bmap = vec![None; prog.len()]; // Vec to store jump locations
    let mut bstack = vec![]; // Used to track nested brackets
    for (position, &token) in prog.iter().enumerate() {
        match token as char {
            '[' => {
                bstack.push(position);
            }
            ']' => {
                if let Some(open_bracket_pos) = bstack.pop() {
                    bmap[open_bracket_pos] = Some(position);
                    bmap[position] = Some(open_bracket_pos);
                }
            }
            _ => (),
        }
    }

    let mut pc: usize = 0;
    let mut cells = vec![0u8; 10000];
    let mut cc: usize = 0;
    while pc < prog.len() {
        match prog[pc] as char {
            '<' => {
                    cc -= 1;
            }
            '>' => {
                    cc += 1;
            }
            '+' => {
                cells[cc] = cells[cc].wrapping_add(1);
            }
            '-' => {
                cells[cc] = cells[cc].wrapping_sub(1);
            }
            '[' if cells[cc] == 0 => {
                if let Some(jump_to) = bmap[pc] {
                    pc = jump_to;
                }
            }
            ']' if cells[cc] != 0 => {
                if let Some(jump_to) = bmap[pc] {
                    pc = jump_to;
                }
            }
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }

    Ok(())
}
