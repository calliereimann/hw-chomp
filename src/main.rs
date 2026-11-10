use chomp_ai::Chomp;
use clap::Parser;
use std::io;

#[derive(Parser)]
struct Args {
    rows: usize,
    cols: usize,
}

fn display(a: &Chomp) {
    let (r, c) = a.dimensions();
    let mut output = String::from("");
    for row in 0..r {
        for col in 0..c {
            if a.is_present(row, col) {
                output.push('X');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    println!("{}", output);
}

fn interact() -> Result<(usize, usize), io::Error> {
    let mut buf = String::from("");
    let _size = io::stdin().read_line(&mut buf)?;
    buf = buf.trim().to_string();
    let values = buf.split(' ');
    let mut pair: Vec<usize> = [].to_vec();
    for a in values {
        let b = a.parse();
        match b {
            Ok(val) => pair.push(val),
            Err(_e) => {
                return Err(io::Error::other("Your input must contain only digits."));
            }
        }
    }
    if pair.len() != 2 {
        Err(io::Error::other("You must provide exactly 2 values."))
    } else {
        Ok((pair[0], pair[1]))
    }
}

fn get_move() -> (usize, usize) {
    let (r, c);
    loop {
        let input = interact();
        match input {
            Ok((a, b)) => {
                (r, c) = (a, b);
                break;
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    (r, c)
}

fn main() {
    let args = Args::parse();
    let a = Chomp::new(args.rows, args.cols);
    let mut output = match a {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Invalid arguments: {}", e);
            panic!()
        }
    };
    let mut flag = true;
    println!("Let's play Chomp! The board size is {} x {}.", output.dimensions().0, output.dimensions().1);
    while flag {
        loop {
            let (r, c) = get_move();
            let yourmove = output.make_move(r, c);
            match yourmove {
                Ok(()) => break,
                Err(e) => eprintln!("{}",e)
            }
        }
        println!("Your Move: ");
        display(&output);
        if !output.is_present(0, 0) {
            println!("You Lose!");
            break;
        }
        let aimove = output.automate();
        match aimove {
            Ok(()) => continue, 
            Err(e) => {
                eprintln!("AI performed an illegal move, {}" ,e);
                println!("You Win!")
        }
        }
        println!("Computer Move: ");
        display(&output);
        flag = output.is_present(0, 0);
    }
}
