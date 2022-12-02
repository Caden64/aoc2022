#![feature(str_split_as_str)]
#![feature(str_split_whitespace_as_str)]

use std::fs;

fn main() {
    let data = read("input.txt".to_string());
    let rounds = data.split("\n");
    let mut total = 0;
    for round in rounds {
        let mut moves = round.split_ascii_whitespace();
        println!("{}", moves.as_str());
        let tm = moves.clone().nth(0).unwrap();
        let ym = moves.clone().nth(1).unwrap();
        let round_winnings = winner(tm.clone().parse().unwrap(), ym.clone().parse().unwrap());
        println!("Round winnings {}", round_winnings);
        total =  total + round_winnings as i32;

    }
    println!("Total {}", total)
}

// x lose
// y draw
// z = win

// rock (A,X)
// paper (B,Y)
// scissors (C,Z)

fn winner(tm: String, ym: String) -> u8 {
    let mmtm = mm(tm.clone());
    let mmym = mm(ym.clone());
    // draw
    if mmtm == mmym {
        return 3 + mmym
    }
    // rock scissors
    if mmym == 1 && mmtm == 3 {
        return 6 + mmym
    }
    // paper rock
    if mmym == 2 && mmtm == 1 {
        return 6 + mmym
    }
    // scissors paper
    if mmym == 3 && mmtm ==2 {
        return 6 + mmym
    }

    0 + mmym
}

fn mm(m: String) -> u8{
    match m.as_str() {
        "A" | "X" => {
            println!("Rock");
            1
        },
        "B" | "Y" => {
            println!("Paper");
            2
        },
        "C" | "Z" => {
            println!("Scissors");
            3
        },
        _ => {
            println!("WHY!");
            0
        }
    }
}

fn read(file_name: String) -> String {
    return fs::read_to_string(file_name).unwrap()
}