use std::{io, iter};

fn main() {
    println!("Stringback Test!\nEnter A String");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Error Reading Input");

    //let chars: Vec<char> = input.chars().collect();

    let s: String = input
        .char_indices()
        .map(f)


    //println!("Here's your String!\n------\n{}\n------", );
        
    }
