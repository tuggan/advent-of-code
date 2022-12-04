use std::{fs::File, io::BufReader};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("../../input/day2.").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut sum: i64 = 0;
    let mut correct_sum: i64 = 0;

    for line in buf_reader.lines() {
        let s: String = line?;
        let opponent = s.chars().nth(0).unwrap();
        let me = s.chars().nth(2).unwrap();
        
        sum += choise(me) + winner(me, opponent)*3;
        
        let c = outcome(opponent, me);
        
        correct_sum += choise_2(c) + score(c, opponent)*3;
        
        // println!("Me: {}, {} - {}", c, choise_2(c), score(c, opponent)*3)
        
    }
    
    println!("Sum: {}", sum);
    println!("Sum: {}", correct_sum);
    Ok(())
}


fn choise_2(mov: char) -> i64 {
    match mov {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    }
}

fn outcome(mov: char, res: char) -> char {
    match res {
        'X' => match mov {
            'A' => 'C',
            'B' => 'A',
            'C' => 'B',
            _ => 'E',
        },
        'Y' => mov,
        'Z' => match mov {
            'A' => 'B',
            'B' => 'C',
            'C' => 'A',
            _ => 'E',
        },
        _ => 'E',
    }
}

fn score(me: char, op: char) -> i64 {
    match me {
        'A' => match op {
            'A' => 1,
            'B' => 0,
            'C' => 2,
            _ => 0,
        },
        'B' => match op {
            'A' => 2,
            'B' => 1,
            'C' => 0,
            _ => 0,
        },
        'C' => match op {
            'A' => 0,
            'B' => 2,
            'C' => 1,
            _ => 0,
        },
        _ => 0,
    }
}

fn choise(me: char) -> i64 {
    match me {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}


/*
A,X for Rock 
B,Y for Paper
C,Z for Scissors
*/

fn winner(me: char, op: char) -> i64 {
    match me {
        'X' => match op {
            'A' => 1,
            'B' => 0,
            'C' => 2,
            _ => -1,
        },
        'Y' => match op {
            'A' => 2,
            'B' => 1,
            'C' => 0,
            _ => -1,
        },
        'Z' => match op {
            'A' => 0,
            'B' => 2,
            'C' => 1,
            _ => -1,
        }
        _ => -1,
    }
}
