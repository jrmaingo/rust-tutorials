use std::io;
use std::env;

const NOARGS: usize = 0;

fn main() {
    let args = &env::args().collect::<Vec<String>>()[1..];
    let mut input = String::new();

    match args.len() {
        NOARGS => {
            println!("no args given, enter content now:");
            // get input if no args given
            match io::stdin().read_line(&mut input) {
                Err(error) => println!("error: {}", error),
                _ => (),
            }
        },
        _ => {
            for elem in args {
                input.push_str(elem.as_str());
            }
        },
    };

    println!("{}", input.trim());
}
