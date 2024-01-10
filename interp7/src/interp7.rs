use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    LBrack(usize),
    RBrack(usize),
    Zero,
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let mut bstack = vec![];
    for b in fs::read(env::args().nth(1).unwrap())? {
        let opcode = match b as char {
            '<' => Ops::Left(usize::max_value()),
            '>' => Ops::Right(usize::max_value()),
            '+' => Ops::Add(u8::max_value()),
            '-' => Ops::Sub(u8::max_value()),
            '[' => Ops::LBrack(usize::max_value()),
            ']' => Ops::RBrack(usize::max_value()),
            '.' => Ops::Output,
            ',' => Ops::Input,
            _ => continue,
        };
        prog.push(opcode);
    }

    // Optimize
    /* Iterate through the program, in search of our "Zero" optimization */
    let mut zerostack = vec![];
    let mut count = 0;
    for position in 0..prog.len() {
        match &mut prog[position] {
            Ops::LBrack(_) if count == 0 => {
                count += 1;
                zerostack.push(position);
            }
            Ops::Sub(_) if count == 1 => {
                count += 1;
            }
            Ops::RBrack(_) if count == 2 => {
                if let Some(lbrack_position) = zerostack.pop() {
                    prog[lbrack_position] = Ops::Zero;
                    prog[lbrack_position+1] = Ops::Add(0);
                    prog[lbrack_position+2] = Ops::Add(0);
                    count = 0;
                }
            }
            _ => {   
                    count = 0;
            }
        }
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

    let mut addstack = vec![];
    let mut found = false;
    let mut count = 0;
    for position in 0..prog.len() {
        match &mut prog[position] {
            Ops::Add(amount) if *amount != 0 => {
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
    
    let mut substack = vec![];
    found = false;
    count = 0;
    for position in 0..prog.len() {
        match &mut prog[position] {
            Ops::Sub(_) => {
                count += 1;
                if !found {
                    substack.push(position);
                } else {
                    prog[position] = Ops::Sub(0);
                }
                found = true;
            }
            _ => {
                if let Some(first_sub_position) = substack.pop() {
                    prog[first_sub_position] = Ops::Sub(count);
                    count = 0;
                    found = false;
                }
            }
        }
    }

    let mut leftstack = vec![];
    found = false;
    let mut shift_count = 0;
    for position in 0..prog.len() {
        match &mut prog[position] {
            Ops::Left(_) => {
                shift_count += 1;
                if !found {
                    leftstack.push(position);
                } else {
                    prog[position] = Ops::Left(0);
                }
                found = true;
            }
            _ => {
                if let Some(first_left_position) = leftstack.pop() {
                    prog[first_left_position] = Ops::Left(shift_count);
                    shift_count = 0;
                    found = false;
                }
            }
        }
    }

    let mut rightstack = vec![];
    found = false;
    shift_count = 0;
    for position in 0..prog.len() {
        match &mut prog[position] {
            Ops::Right(_) => {
                shift_count += 1;
                if !found {
                    rightstack.push(position);
                } else {
                    prog[position] = Ops::Right(0);
                }
                found = true;
            }
            _ => {
                if let Some(first_left_position) = rightstack.pop() {
                    prog[first_left_position] = Ops::Right(shift_count);
                    shift_count = 0;
                    found = false;
                }
            }
        }
    }

    // Interpret / Evaluate
    let mut cells = vec![0u8; 10000];
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left(count) => {
                    cc -= count;
                pc += (count-1) as usize;
            }
            Ops::Right(count) => {
                    cc += count;
                pc += (count-1) as usize;
            }
            Ops::Add(count) => {
                cells[cc] = cells[cc].wrapping_add(count);
                pc += (count-1) as usize;
            }
            Ops::Sub(count) => {
                cells[cc] = cells[cc].wrapping_sub(count);
                pc += (count-1) as usize;
            }
            Ops::LBrack(jump_to) if cells[cc] == 0 => {
                pc = jump_to;
            }
            Ops::RBrack(jump_to) if cells[cc] != 0 => {
                pc = jump_to;
            }
            Ops::Zero => {
                cells[cc] = cells[cc].wrapping_sub(cells[cc]);
                pc += 2;
            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
