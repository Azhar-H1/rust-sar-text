use std::{io, iter};

fn main() {
    println!("Stringback Test!\nEnter A String");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Error Reading Input");

    let mut chars: Vec<char> = input.chars().collect();

    for n  in 0..(chars.len()-1) {
        println!("==> Current index {}", n);
    //for n in chars.iter() {
        if n % 2 == 0{
            println!("==> Got Even Index @{} with {}", n, chars[n]);
            if chars[n] as u32 >= 41 && chars[n] as u32 <= 90{
                let prints: String =chars.clone().into_iter().collect();
                println!("==> Attempting to modify: {}", prints )  ;
                 chars[n].to_ascii_uppercase();
            }
        }
    }

    let s: String = chars.into_iter().collect();
    println!("Here's your String!\n------\n{}\n------", s);
        
    }
