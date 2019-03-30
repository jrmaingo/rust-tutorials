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

fn main() {
    let mut ssb_secret: SsbSecret;

    // extract object for file, parse as JSON
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

    // parse secret key from file
    let secret_key_enc_bytes = json["private"].as_str()
        .expect("secret key is not a string")
        .splitn(2, ".").next().unwrap()
        .as_bytes();
    let secret_key_dec_bytes = decode(secret_key_enc_bytes)
        .expect("failed to decode secret key");
    ssb_secret.secret_key = SecretKey::from_slice(&secret_key_dec_bytes)
        .expect("failed to create secret key from binary data");

    // TODO: read public key and test!
}
