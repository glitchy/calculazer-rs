extern crate exitcode;

use std::io;

fn main() {
    welcome();
    let v1 = collect();
    let v2 = collect();
    let total = operation(v1, v2);
    format_results(total);
}

fn collect() -> i32 {
    println!("Please enter a unary number between 0-255:");
    let mut unary_input = String::new();
    io::stdin()
        .read_line(&mut unary_input)
        .expect("Invalid input, please start over.");

    match unary_input.trim().parse::<i32>() {
        Ok(unary_input) => {
            return unary_input;
        }
        Err(e) => {
            eprintln!(
                "Sorry, '{}' is an invalid input. Please run again.",
                unary_input.trim()
            );
            std::process::exit(exitcode::DATAERR);
        }
    }
}

fn operation(v1: i32, v2: i32) -> i32 {
    println!("\nSelect operator:\n(+) addition\n(-) subtraction\n(*) multiplication\n(/) division");
    let mut operator_input = String::new();
    io::stdin()
        .read_line(&mut operator_input)
        .expect("Invalid input, please start over.");

    match operator_input.trim() {
        "+" => addition(v1, v2),
        "-" => subtraction(v1, v2),
        "*" => multiplication(v1, v2),
        "/" => match v1 % v2 {
            0 => division(v1, v2),
            _ => {
                eprintln!("Sorry, {} % {} != 0. Please run again.", v1, v2);
                std::process::exit(exitcode::DATAERR);
            }
        },
        _ => {
            eprintln!(
                "Sorry, '{}' is an invalid input. Please run again.",
                operator_input.trim()
            );
            std::process::exit(exitcode::DATAERR);
        }
    }
}
// just think connect 4 and all of this will make sense
fn addition(mut v1: i32, mut v2: i32) -> i32 {
    loop {
        if v1 == 0 {
            break;
        };
        // finds the carryover bit and shifts it over
        let z = (v1 & v2) << 1;
        // replicates addition (ones fall through)
        v2 ^= v1;
        // assigns new carry over value to v1 which applies in the next loop
        v1 = z;
    }
    return v2;
}

fn subtraction(mut v1: i32, mut v2: i32) -> i32 {
    // !!!--the short way--!!! //
    //return addition(v1, addition(!v2, 1));

    // !!!--the long way--!!! //
    loop {
        if v2 == 0 {
            break;
        };
        // inverts v1 giving a negative representation of a signed integer (though it is
        // technically not!) and finds the carry over bit
        let z = (!v1 & v2) << 1;
        // XOR bitwise operator essentially replicates addition
        v1 ^= v2;
        v2 = z;
    }
    return v1;
}

fn multiplication(mut v1: i32, mut v2: i32) -> i32 {
    let mut total = 0;
    let mut count = 0;
    loop {
        if v2 != 0 {
            if v2 % 2 == 1 {
                total = addition(total, v1 << count);
            };
            count = addition(count, 1);
            v2 = division(v2, 2)
        } else {
            break;
        }
    }
    return total;
}

fn division(mut v1: i32, v2: i32) -> i32 {
    let mut sum = 0;
    loop {
        if v1 >= v2 {
            v1 = subtraction(v1, v2);
            sum = addition(sum, 1);
        } else {
            break;
        }
    }
    return sum;
}

fn format_results(v: i32) {
    match v {
        v if v < 0 && v >= -128 => {
            println!("\nUnary total: {:?}", v);
            println!("Binary total: {:#b}", v as i8);
        },
        _ => {
            println!("\nUnary total: {:?}", v);
            println!("Binary total: {:#b}", v);
        }
    }
}

fn welcome() {
    // Clears the screen.
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!(
        "
                                         88                       88                                            88
          Welcome to                     88                       88                                            88
                                         88                       88                                            88
                    ,adPPYba, ,adPPYYba, 88  ,adPPYba, 88      88 88 ,adPPYYba, 888888888  ,adPPYba, 8b,dPPYba, 88
                   a8P     “” \"\"     `Y8 88 a8P     “” 88      88 88 \"\"     `Y8      ,aP” a8P_____88 88P'   \"Y8 88
                   8PP        ,adPPPPP88 88 8PP        88      88 88 ,adPPPPP88    ,aP”   8PP\"\"\"\"\"\"\" 88         88
                   \"8b,   ,aa 88,    ,88 88 \"8b,   ,aa 8b,    ,d8 88 88,    ,88  ,aP”     \"8b,   ,aa 88         ``
                    `\"Ybbd8\"' `\"8bbdP\"Y8 88  `\"Ybbd8\"' `\"YbbddY\"` 88 `\"8bbdP\"Y8 888888888  `\"Ybbd8\"' 88         69\n")
}
