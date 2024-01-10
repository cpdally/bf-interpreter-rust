use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack,
    RBrack,
    Output,
    Input,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    /* Notice: prog is now a vec of OpCodes, not a string */
    let mut prog = vec![];

    /* First parse the program into a sequence of opcodes */
    for b in fs::read(env::args().nth(1).unwrap())? {
        let opcode = match b as char {
            '<' => Ops::Left,
            '>' => Ops::Right,
            '+' => Ops::Add,
            '-' => Ops::Sub,
            '[' => Ops::LBrack,
            ']' => Ops::RBrack,
            '.' => Ops::Output,
            ',' => Ops::Input,
            _ => continue,
        };
        prog.push(opcode);
    }

    let mut pc = 0;
    let mut bmap = vec![None; prog.len()]; // Vec to store jump locations
    let mut bstack = vec![]; // Used to track nested brackets
    for (position, &ref token) in prog.iter().enumerate() {
        match *token {
            Ops::LBrack => {
                bstack.push(position);
            }
            Ops::RBrack => {
                if let Some(open_bracket_pos) = bstack.pop() {
                    bmap[open_bracket_pos] = Some(position);
                    bmap[position] = Some(open_bracket_pos);
                }
            }
            _ => (),
        }
    }

    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
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
            Ops::LBrack if cells[cc] == 0 => {
                if let Some(jump_to) = bmap[pc] {
                    pc = jump_to;
                }
            }
            Ops::RBrack if cells[cc] != 0 => {
                if let Some(jump_to) = bmap[pc] {
                    pc = jump_to;
                }
            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
