use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add(u8),
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    
    for b in fs::read(env::args().nth(1).unwrap())? {
        let opcode = match b as char {
            '<' => Ops::Left,
            '>' => Ops::Right,
            '+' => Ops::Add(u8::max_value()),
            '-' => Ops::Sub,
            '[' => Ops::LBrack(usize::max_value()),
            ']' => Ops::RBrack(usize::max_value()),
            '.' => Ops::Output,
            ',' => Ops::Input,
            _ => continue,
        };
        prog.push(opcode);
    }
    
    let mut bstack = vec![];
    let mut addstack = vec![];
    let mut found = false;
    let mut count = 0;

    for position in 0..prog.len() {
        match &mut prog[position] {
            Ops::LBrack(_) => bstack.push(position),
            Ops::RBrack(_) => {
                if let Some(open_bracket_pos) = bstack.pop() {
                    prog[open_bracket_pos] = Ops::LBrack(position);
                    prog[position] = Ops::RBrack(open_bracket_pos);
                }
            }
            Ops::Add(_) => {
                count += 1;
                if !found {
                    addstack.push(position);
                } else {
                    prog[position] = Ops::Add(0);
                }
                found = true;
            }
            _ => {
                if let Some(first_add_position) = addstack.pop() {
                    prog[first_add_position] = Ops::Add(count);
                    count = 0;
                    found = false;
                }
            }
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
            Ops::Add(count) => {
                cells[cc] = cells[cc].wrapping_add(count);
                pc += (count-1) as usize;
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
