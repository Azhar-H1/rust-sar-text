use std::io;

fn main() {
    println!("Stringback Test!\nEnter A String");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Error Reading Input");

    println!("Here's your String!\n--> {} <--", input);
        // 1235467890123456790132456789013245678901234567890
        /*  can't figure out why there's a hidden \n
         on the second indenting arrow - not necessary
            for the time being*/
    }
