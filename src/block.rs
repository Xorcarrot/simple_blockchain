use std::error::Error;
use js_sys::Date;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::max_attempts_error::MaxAttemptsError;

const MAX_ATTEMPTS: usize = 10000000;
const DIFFICULTY: usize = 2;


#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    index: usize,
    time: u128,
    current_hash: [u8; 32],
    pub last_hash: [u8; 32],
    data: String,
}

impl Block {
    pub fn create(index: usize, last_hash: [u8; 32], data: String) -> Result<Block, Box<dyn Error>> {
        let time = Date::now() as u128;

        let mut attempt: usize = 0;
        let current_hash = loop {
            let hash_string: String = format!("{}{}{}{}", index, data, time, attempt);
            if let Some(hash) = hash_sha256(hash_string.as_str()) { break hash }

            attempt += 1;
            if attempt > MAX_ATTEMPTS { return Err(Box::new(MaxAttemptsError)) }
        };


        Ok(Block {
            index,
            time,
            current_hash,
            last_hash,
            data,
        })
    }
}

//Generates a new sha256 hash from a text.
pub fn hash_sha256(hash_text: &str) -> Option<[u8; 32]> {
    let mut hash = Sha256::new();
    hash.update(hash_text.as_bytes());

    let final_hash: [u8; 32] = hash.finalize().into();
    if !prove_of_work(final_hash, DIFFICULTY) { return None };

    Some(final_hash)
}

pub fn prove_of_work(hash: [u8; 32], difficulty: usize) -> bool {
    let mut index: usize = 0;

    while index < difficulty {
        if hash[index] != 0 { return false }
        index += 1;
    }

    true
}