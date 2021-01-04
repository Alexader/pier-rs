mod key;

use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Key {
    pub typ: String,
    pub cipher: Cipher,
}

#[derive(Serialize, Deserialize)]
pub struct Cipher {
    pub cipher: String,
    pub data: String,
}

fn read_key(path: &String) -> Key {
    let jsonData = fs::read_to_string(path)?;
    let key: Key = serde_json::from_str(jsonData)?;
    return key;
}