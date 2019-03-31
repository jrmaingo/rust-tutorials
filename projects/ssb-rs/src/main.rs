use base64::decode;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use sodiumoxide::crypto::sign::ed25519::*;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct SsbSecret {
    curve: String,
    secret_key: SecretKey,
    public_key: PublicKey,
    feed_id: String,
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
    fn new(file_path: &str) -> SsbSecret {
        let json: Value = get_json_secret(file_path);
        let ssb_secret: SsbSecret;

        // parse secret key from file
        let secret_key_enc_bytes = json["private"].as_str()
            .expect("secret key is not a string")
            .splitn(2, ".").next().unwrap()
            .as_bytes();
        let secret_key_dec_bytes = decode(secret_key_enc_bytes)
            .expect("failed to decode secret key");
        let secret_key = SecretKey::from_slice(&secret_key_dec_bytes)
            .expect("failed to create secret key from binary data");

        // TODO: read public key and test!

        SsbSecret {
            curve: String::from(json["curve"].as_str().unwrap()),
            secret_key: secret_key,
            // TODO: public_key
            // TODO: feed_id
        }
    }
}

fn main() {
    let mut ssb_secret = SsbSecret::new("/Users/julian/.ssb/secret");
}
