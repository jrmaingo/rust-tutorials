use std::io;
use std::io::Write;

// convert F to C
fn convert_f(value: f64) -> f64 {
    (value - 32.0) * 5.0 / 9.0
}

// convert C to F
fn convert_c(value: f64) -> f64 {
    value * 9.0 / 5.0 + 32.0
}

// convert value according to units
fn convert(unit_str: &str, value: f64) -> f64 {
    match unit_str {
        "F" => convert_f(value),
        "C" => convert_c(value),
        _   => panic!("not a valid unit!")
    }
}

// get other unit
fn get_other_unit(unit_str: &str) -> &str {
    match unit_str {
        "F" => "C",
        "C" => "F",
        _   => panic!("not a valid unit!")
    }
}

fn main() {
    loop {
        // gets input string for temp unit
        let mut input_str = String::new();
        print!("Input unit: ");
        io::stdout().flush()
            .expect("Failed to flush output");
        io::stdin().read_line(&mut input_str)
            .expect("Failed to read line");
        let unit_str = String::from(input_str.trim());

        // determines what unit was input
        if unit_str == "F" {
            println!("Converting F -> C");
        } else if unit_str == "C" {
            println!("Converting C -> F");
        } else {
            println!("unknown unit, please select C or F");
            continue;
        }

        // get value to be converted
        let value: f64;
        input_str.clear();
        loop {
            print!("Input value: ");
            io::stdout().flush()
                .expect("Failed to flush output");
            io::stdin().read_line(&mut input_str)
                .expect("Failed to read line");
            value = match input_str.trim().parse() {
                Ok(num) => num,
                Err(_)  => {
                    println!("Got <{}>", input_str.trim());
                    println!("Not a number!");
                    continue;
                }
            };
            break;
        }

        println!("Got value: {}", value);

        let result = convert(&unit_str, value);
        println!("Result: {}{}", result, get_other_unit(&unit_str));
    }
}
