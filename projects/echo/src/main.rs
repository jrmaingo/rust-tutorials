use std::io;
use std::env;

const NOARGS: usize = 0;

fn main() {
    // need to use turbofish (::<>) to sepcify type when getting slice
    // no need to specify type of Vec elements
    let args = &env::args().collect::<Vec<_>>()[1..];
    let mut input = String::new();

    match args.len() {
        NOARGS => {
            println!("no args given, enter content now:");
            // get input if no args given
            match io::stdin().read_line(&mut input) {
                Err(error) => println!("error: {}", error),
                _ => (), // need to return 'unit'
            }
        },
        _ => {
            // add all args to output string (does not preserve spacing)
            for elem in args {
                input.push_str(elem.as_str());
                input.push_str(" ");
            }
        },
    };

    println!("{}", input.trim());
}
