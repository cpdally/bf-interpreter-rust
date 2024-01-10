use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    // Notice: we drop bmap here, since it isn't needed.
    let mut bstack = vec![];
    
    for b in fs::read(env::args().nth(1).unwrap())? {
        let opcode = match b as char {
            '<' => Ops::Left,
            '>' => Ops::Right,
            '+' => Ops::Add,
            '-' => Ops::Sub,
            '[' => Ops::LBrack(usize::max_value()),
            ']' => Ops::RBrack(usize::max_value()),
            '.' => Ops::Output,
            ',' => Ops::Input,
            _ => continue,
        };
        prog.push(opcode);
    }

    for position in 0..prog.len() {
        match &mut prog[position] {
            Ops::LBrack(_) => {
                bstack.push(position);
            }
            Ops::RBrack(_) => {
                if let Some(open_bracket_pos) = bstack.pop() {
                    prog[open_bracket_pos] = Ops::LBrack(position);
                    prog[position] = Ops::RBrack(open_bracket_pos);
                }
            }
            _ => (),
        }
    }


    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left => {
                    cc -= 1;
            }
            Ops::Right => {
                    cc += 1;
            }
            Ops::Add => {
                cells[cc] = cells[cc].wrapping_add(1);
            }
            Ops::Sub => {
                cells[cc] = cells[cc].wrapping_sub(1);
            }
            Ops::LBrack(jump_to) if cells[cc] == 0 => {
                pc = jump_to;
            }
            Ops::RBrack(jump_to) if cells[cc] != 0 => {
                pc = jump_to;
            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
