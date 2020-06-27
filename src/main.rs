mod lexer;
use lexer::{lexer};
#[warn(unused_imports)]

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    let path = Path::new("test/main.rao");
    let display = path.display();

    let mut file = match File::open(&path)  {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s)   {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => println!(""),//print!("{} contains: \n{}", display, s), 
    }
    lexer(s)
}

