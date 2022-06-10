// Mostly Yvan stuff, some change by Axel
// Removing Regex because was not usefull.

use std::io;

const P: usize = 5; // number of pawns
const C: usize = 8; // number of colors

const MAX: usize = 32768; // C^P

fn println_pawns(mut p: usize) {
    let mut str = String::with_capacity(P * 4);
    for _ in 0..P {
        str.push_str(match p % C {
            0 => " Bla",
            1 => " Whi",
            2 => " Red",
            3 => " Gre",
            4 => " Blu",
            5 => " Yel",
            6 => " Ora",
            7 => " Bro",
            _ => " ...",
        });
        p /= C;
    }
    println!("[{} ]", str);
}

fn get_number_of_well_placed_pawn(mut a: usize, mut b: usize) -> usize {
    let mut c: usize = 0;
    for _ in 0..P {
        if a % C == b % C {
            c += 1;
        }
        a /= C;
        b /= C;
    }
    return c;
}

fn get_number_of_good_colors(mut a: usize, mut b: usize) -> usize {
    let mut x = [false; C];
    let mut y = [false; C];

    for _ in 0..P {
        x[a % C] = true;
        y[b % C] = true;
        a /= C;
        b /= C;
    }

    x.iter().zip(y).filter(|&(&m, p)| p && m).count()
}

fn get_number_of_colors(mut p: usize) -> usize {
    let mut x = [false; C];
    for _ in 0..P {
        x[p % C] = true;
        p /= C;
    }
    x.iter().filter(|&&c| c == true ).count()
}

fn main() {
    println!("# ## ### ##### ######## ############ ######## ##### ### ## #");
    println!("# ## ### ##### ########  MASTERMIND  ######## ##### ### ## #");
    println!("# ## ### ##### ######## ############ ######## ##### ### ## #");
    let mut vec: Vec<bool> = vec![true; MAX];

    println!("The code can not contain twice the same color? [Y/n] ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => println!("\x1b[31m.. error:\x1b[0m {}", err),
    }

    let input  = input.trim();
    if input == "y" || input == "Y" {
        for i in 0..MAX {
            if get_number_of_colors(i) != P {
                vec[i] = false;
            }
        }
    }

    let mut turn: usize = 1;
    loop {
        let mut n: usize = 0;
        let mut p: usize = 0;
        for i in 0..MAX {
            if vec[i] {
                n += 1;
                p = i;
            }
        }

        if n == 0 {
            println!("\x1b[31m.. error:\x1b[0m sequence not found...");
            break;
        }

        println!("\x1b[33m.. sequences:\x1b[0m {}", n);
        println!("\x1b[33m.. turn {}:\x1b[0m ", turn);
        println_pawns(p);
        println!("\x1b[34m>> black:\x1b[0m ");

        let mut input = String::new();
        if let Err(err)  = io::stdin().read_line(&mut input) {
            println!("\x1b[31m.. error:\x1b[0m {}", err);
        }

        let b: usize = input.trim().parse().expect("Input was not a number");

        println!("\x1b[34m>> white:\x1b[0m ");
        let mut input = String::new();
        if let Err(err)  = io::stdin().read_line(&mut input) {
            println!("\x1b[31m.. error:\x1b[0m {}", err);
        }

        let w: usize = input.trim().parse().expect("Input was not a number");

        if b + w > 5 {
            println!(
                "\x1b[31m.. error:\x1b[0m \
                 please input two numbers b, w in [0, 5] with b + w <= 5"
            );
            continue;
        }

        if b == P {
            println!("\x1b[32m.. success:\x1b[0m I win!");
            break;
        }

        vec[p] = false;
        for i in 0..MAX {
            if vec[i]
                && (get_number_of_well_placed_pawn(p, i) != b
                    || get_number_of_good_colors(p, i) != b + w)
            {
                vec[i] = false;
            }
        }
        turn += 1;
    }
}
