use num_format::{Locale, ToFormattedString};
use std::{env, process::exit};

fn main() {
    let number: i8;
    if let Some(arg) = env::args().nth(1) {
        number = arg.trim().parse::<i8>().unwrap_or_default();
    } else {
        println!("Usage: ./nth_fibonacci <number>");
        exit(0);
    }
    if number >= 93 {
        println!("The number you entered is too large for this program to handle.");
        println!("FYI, the 93rd fibonacci number is 12,200,160,415,121,876,738.");
        exit(0);
    }
    println!(
        "The {number}{} fibonacci number is {}.",
        ordinal_suffix(number),
        fibonacci(number).to_formatted_string(&Locale::en),
    )
}

fn fibonacci(num: i8) -> i64 {
    if num == 0 {
        return 0;
    // } else if num == 1 {
    //     return 1;
    } else {
        let mut previous: i64 = 0;
        let mut current: i64 = 1;
        let mut next: i64 = 1;
        for _ in 1..num {
            next = previous + current;
            previous = current;
            current = next;
        }
        next
    }
}

fn ordinal_suffix(number: i8) -> String {
    if number % 100 == 11 || number % 100 == 12 || number % 100 == 13 {
        return "th".to_string();
    }
    match number % 10 {
        1 => "st".to_string(),
        2 => "nd".to_string(),
        3 => "rd".to_string(),
        _ => "th".to_string(),
    }
}
