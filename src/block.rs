use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use crate::max_attempts_error::MaxAttemptsError;

const MAX_ATTEMPTS: usize = 10000000;
const DIFFICULTY: usize = 3;


#[derive(Debug)]
pub struct Block {
    index: usize,
    time: u128,
    current_hash: [u8; 32],
    pub last_hash: [u8; 32],
    data: String,
}

impl Block {
    pub fn create(index: usize, last_hash: [u8; 32], data: String) -> Result<Block, Box<dyn Error>> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

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

mod tests {
    use super::*;

    const VALID_SHA256: [u8; 32] = [
        0, 0, 0, 0, 0,
        123, 45, 67, 89, 12, 34, 56, 78, 90, 123, 145, 167, 189, 210, 234, 255,
        33, 67, 99, 101, 111, 131, 151, 171, 191, 211, 231
    ];

    const INVALID_SHA256: [u8; 32] = [
        241, 123, 64, 86, 9,
        123, 45, 67, 89, 12, 34, 56, 78, 90, 123, 145, 167, 189, 210, 234, 255,
        33, 67, 99, 101, 111, 131, 151, 171, 191, 211, 231
    ];

    #[test]
    fn pof_is_true() {
        let result = prove_of_work(VALID_SHA256, DIFFICULTY);
        assert!(result);
    }

    #[test]
    fn pof_is_false() {
        let result = prove_of_work(INVALID_SHA256, DIFFICULTY);
        assert!(!result);
    }
}