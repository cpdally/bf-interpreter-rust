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
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let mut bstack = vec![];

    let mut add_count = 0;
    let mut sub_count = 0;
    let mut left_count = 0;
    let mut right_count = 0;

    for b in fs::read(env::args().nth(1).unwrap())? {
        match b as char {
            '<' => {
                if right_count > 0 {
                    prog.push(Ops::Right(right_count));
                } else if add_count > 0 {
                    prog.push(Ops::Add(add_count as u8));
                } else if sub_count > 0 {
                    prog.push(Ops::Sub(sub_count as u8));
                }
                left_count += 1;
                right_count = 0;
                add_count = 0;
                sub_count = 0;
            }
            '>' => {
                if left_count > 0 {
                    prog.push(Ops::Left(left_count));
                } else if add_count > 0 {
                    prog.push(Ops::Add(add_count as u8));
                } else if sub_count > 0 {
                    prog.push(Ops::Sub(sub_count as u8));
                }
                right_count += 1;
                left_count = 0;
                add_count = 0;
                sub_count = 0;
            }
            '+' => {
                if left_count > 0 {
                    prog.push(Ops::Left(left_count));
                } else if right_count > 0 {
                    prog.push(Ops::Right(right_count));
                } else if sub_count > 0 {
                    prog.push(Ops::Sub(sub_count as u8));
                }
                add_count += 1;
                left_count = 0;
                right_count = 0;
                sub_count = 0;
            }
            '-' => {
                if left_count > 0 {
                    prog.push(Ops::Left(left_count));
                } else if right_count > 0 {
                    prog.push(Ops::Right(right_count));
                } else if add_count > 0 {
                    prog.push(Ops::Add(add_count as u8));
                }
                sub_count += 1;
                left_count = 0;
                right_count = 0;
                add_count = 0;
            }
            '[' => {
                if left_count > 0 {
                    prog.push(Ops::Left(left_count));
                } else if right_count > 0 {
                    prog.push(Ops::Right(right_count));
                } else if sub_count > 0 {
                    prog.push(Ops::Sub(sub_count as u8));
                } else if add_count > 0 {
                    prog.push(Ops::Add(add_count as u8));
                }
                prog.push(Ops::LBrack(usize::max_value()));
                left_count = 0;
                right_count = 0;
                add_count = 0;
                sub_count = 0;
            }
            ']' => {
                if left_count > 0 {
                    prog.push(Ops::Left(left_count));
                } else if right_count > 0 {
                    prog.push(Ops::Right(right_count));
                } else if sub_count > 0 {
                    prog.push(Ops::Sub(sub_count as u8));
                } else if add_count > 0 {
                    prog.push(Ops::Add(add_count as u8));
                }
                prog.push(Ops::RBrack(usize::max_value()));
                left_count = 0;
                right_count = 0;
                add_count = 0;
                sub_count = 0;
            }
            '.' => {
                if left_count > 0 {
                    prog.push(Ops::Left(left_count));
                } else if right_count > 0 {
                    prog.push(Ops::Right(right_count));
                } else if sub_count > 0 {
                    prog.push(Ops::Sub(sub_count as u8));
                } else if add_count > 0 {
                    prog.push(Ops::Add(add_count as u8));
                }
                prog.push(Ops::Output);
                left_count = 0;
                right_count = 0;
                add_count = 0;
                sub_count = 0;
            }
            ',' => {
                if left_count > 0 {
                    prog.push(Ops::Left(left_count));
                } else if right_count > 0 {
                    prog.push(Ops::Right(right_count));
                } else if sub_count > 0 {
                    prog.push(Ops::Sub(sub_count as u8));
                } else if add_count > 0 {
                    prog.push(Ops::Add(add_count as u8));
                }
                prog.push(Ops::Input);
                left_count = 0;
                right_count = 0;
                add_count = 0;
                sub_count = 0;
            }
            _ => {
                continue;
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

    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left(count) => {
                cc -= count;
            }
            Ops::Right(count) => {
                cc += count;
            }
            Ops::Add(count) => {
                cells[cc] = cells[cc].wrapping_add(count);
            }
            Ops::Sub(count) => {
                cells[cc] = cells[cc].wrapping_sub(count);
            }
            Ops::LBrack(jump_to) if cells[cc] == 0 => {
                pc = jump_to;
            }
            Ops::RBrack(jump_to) if cells[cc] != 0 => {
                pc = jump_to;
            }
            Ops::Output => {
                io::stdout().write_all(&cells[cc..cc + 1])?;
            }
            Ops::Input => {
                io::stdin().read_exact(&mut cells[cc..cc + 1])?;
            }
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
