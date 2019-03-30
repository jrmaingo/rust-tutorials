use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct SsbSecret {
    curve: String,
    //keypair: Keypair,
    feed_id: String,
}

fn main() {
    let file = File::open("/Users/julian/.ssb/secret")
        .expect("Unable to read secret file");
    let json_str = BufReader::new(file)
        .lines()
        .filter_map(|line_res| {
            let line = line_res.unwrap();
            if !line.starts_with("#") {
                Some(line)
            } else {
                None
            }
        })
        .fold(String::new(), |mut acc, line| {
            acc.push_str(&line);
            acc
        });
    let json: Value = serde_json::from_str(&json_str)
        .expect("failed to parse secret file");
    println!("curve: {}", json["curve"]);
    // let key = Keypair::from_bytes(
}
