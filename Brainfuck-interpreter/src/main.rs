#![allow(dead_code)]

enum Commands {
    Increment,
    Decrement,
    InvalidToken,
    Forward,
    Backward,
    Output,
}   
use std::{
    fs::File,
    path::Path,
    io::*
};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide a file to execute")
    } 
    let filepath = &args[1];
    let path = Path::new(filepath);
    let display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open file {display} : {why}")
    };
    let mut code = String::new();
    match file.read_to_string(&mut code) {
        Ok(_) => execute(code),
        Err(why) => panic!("couldn't read file {display} : {why}")
    }

}

fn parse(c: char) -> Commands{
    match c {
        '<' => Commands::Backward,
        '>' => Commands::Forward,
        '+' => Commands::Increment,
        '-' => Commands::Decrement,
        ':' => Commands::Output,
        _ => Commands::InvalidToken
    }
}

fn execute(code: String) {

    let mut indx = 0;
    let mut regs: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    for c in code.chars() {
        let val = parse(c);
        match val {
            Commands::Backward => {
                if indx == 0 {
                    continue
                }
                indx -= 1;
            },
            Commands::Forward => {
                if indx == 7 {
                    continue
                }
                indx += 1
            },
            Commands::Increment => regs[indx] += 1,
            Commands::Decrement => {
                if regs[indx] == 0 {
                    continue
                }
                regs[indx] -= 1
            },
            Commands::InvalidToken => {
                println!("Invalid token {c} was found in code");
                break;
            },
            Commands::Output => {
                for reg in regs {
                    println!("{reg}");
                }
            }


        }
    }
}