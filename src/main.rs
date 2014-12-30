extern crate regex;

use regex::Regex;
use std::io;

static P: uint = 5u; // number of pawns
static C: uint = 8u; // number of colors

static MAX: uint = 32768u; // C^P

fn println_pawns(mut p: uint) {
    let mut str: String = "[".to_string();
    for i in range(0u, P) {
        str.push_str(match p % C {
            0u => " Bla",
            1u => " Whi",
            2u => " Red",
            3u => " Gre",
            4u => " Blu",
            5u => " Yel",
            6u => " Ora",
            7u => " Bro",
            _  => " ..."
        });
        p /= C;
    }
    println!("{} ]", str);
}

fn get_number_of_well_placed_pawn(mut a: uint, mut b: uint) -> uint {
    let mut c: uint = 0u;
    for i in range(0u, P) {
        if a % C == b % C {
            c += 1u;
        }
        a /= C;
        b /= C;
    }
    return c;
}

fn get_number_of_good_colors(mut a: uint, mut b: uint) -> uint {
    let mut x: Vec<bool> = Vec::with_capacity(C);
    let mut y: Vec<bool> = Vec::with_capacity(C);
    for i in range(0u, C) {
        x.push(false);
        y.push(false);
    }
    for i in range(0u, P) {
        x[a % C] = true;
        y[b % C] = true;
        a /= C;
        b /= C;
    }
    let mut c: uint = 0u;
    for i in range(0u, C) {
        if x[i] && y[i] {
            c += 1u;
        }
    }
    return c;
}

fn get_number_of_colors(mut p: uint) -> uint {
    let mut x: Vec<bool> = Vec::with_capacity(C);
    for i in range(0u, C) {
        x.push(false);
    }
    for i in range(0u, P) {
        x[p % C] = true;
        p /= C;
    }
    let mut c: uint = 0u;
    for i in range(0u, C) {
        if x[i] {
            c += 1u;
        }
    }
    return c;
}

fn main() {
    println!("# ## ### ##### ######## ############ ######## ##### ### ## #");
    println!("# ## ### ##### ########  MASTERMIND  ######## ##### ### ## #");
    println!("# ## ### ##### ######## ############ ######## ##### ### ## #");
    let mut vec: Vec<bool> = Vec::with_capacity(MAX);
    for i in range(0u, MAX) {
        vec.push(true);
    }
    print!("The code can not contain twice the same color? [Y/n] ");
    let input: String = io::stdin().read_line()
                                   .ok()
                                   .expect("Failed to read line");
    let re = match Regex::new(r"^[yY]?$") {
        Ok(re)   => re,
        Err(err) => panic!("{}", err)
    };
    if re.is_match(input.as_slice().trim()) {
        for i in range(0u, MAX) {
            if get_number_of_colors(i) != P {
                vec[i] = false;
            }
        }
    }
    let mut turn: uint = 1u;
    loop {
        let mut n: uint = 0u;
        let mut p: uint = 0u;
        for i in range(0u, MAX) {
            if vec[i] {
                n += 1u;
                p = i;
            }
        }
        if n == 0u {
            println!("\x1b[31m.. error:\x1b[0m sequence not found...");
            break;
        }
        println!("\x1b[33m.. sequences:\x1b[0m {}", n);
        print!("\x1b[33m.. turn {}:\x1b[0m ", turn);
        println_pawns(p);
        print!("\x1b[34m>> black:\x1b[0m ");
        let input = io::stdin().read_line().ok()
                               .expect("failed to read line");
        let b: uint = match input.trim().parse() {
            Some(b) => b,
            None    => 42u
        };
        print!("\x1b[34m>> white:\x1b[0m ");
        let input = io::stdin().read_line().ok()
                               .expect("failed to read line");
        let w: uint = match input.trim().parse() {
            Some(w) => w,
            None    => 42u
        };
        if b > 5u || w > 5u || b + w > 5u {
            println!("\x1b[31m.. error:\x1b[0m \
                     please input two numbers b, w in [0, 5] with b + w <= 5");
            continue;
        }
        if b == P {
            println!("\x1b[32m.. success:\x1b[0m I win!");
            break;
        }
        vec[p] = false;
        for i in range(0u, MAX) {
            if vec[i] && (get_number_of_well_placed_pawn(p, i) != b
                            || get_number_of_good_colors(p, i) != b + w) {
                vec[i] = false;
            }
        }
        turn += 1u;
    }
}
