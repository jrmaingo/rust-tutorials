use base64::decode;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use sodiumoxide::crypto::sign::ed25519::*;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct SsbSecret {
    curve: String,
    secret_key: SecretKey,
    public_key: PublicKey,
    id: String,
}

fn get_json_secret(file_path: &str) -> Value {
    // extract object for file, parse as JSON
    let file = File::open(file_path)
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
    serde_json::from_str(&json_str)
        .expect("failed to parse secret file")
}

impl SsbSecret {
    fn parse_key(json_secret: &Value, key_name: &str) -> Vec<u8> {
        let key_enc_bytes = json_secret[key_name].as_str()
            .expect("key is not a string")
            .splitn(2, ".").next().unwrap()
            .as_bytes();
        decode(key_enc_bytes)
            .expect("failed to decode key")
    }

    fn new(file_path: &str) -> SsbSecret {
        let json_secret: Value = get_json_secret(file_path);

        // parse secret key from file
        let secret_key_dec_bytes = SsbSecret::parse_key(&json_secret, "private");
        let secret_key = SecretKey::from_slice(&secret_key_dec_bytes)
            .expect("failed to create secret key from binary data");

        // parse public key from file
        let public_key_dec_bytes = SsbSecret::parse_key(&json_secret, "public");
        let public_key = PublicKey::from_slice(&public_key_dec_bytes)
            .expect("failed to create public key from binary data");

        SsbSecret {
            curve: String::from(json_secret["curve"].as_str().unwrap()),
            secret_key: secret_key,
            public_key: public_key,
            id: String::from(json_secret["id"].as_str().unwrap()),
        }
    }
}

fn main() {
    let mut ssb_secret = SsbSecret::new("/Users/julian/.ssb/secret");
    println!("{:?}", ssb_secret);
}
