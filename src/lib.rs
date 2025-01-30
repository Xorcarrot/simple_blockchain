mod block;
mod blockchain;
mod max_attempts_error;

use wasm_bindgen::prelude::*;
use web_sys::Storage;
use crate::blockchain::Blockchain;

const KEY_BLOCKCHAIN: &str = "chain";

#[wasm_bindgen]
pub fn create_blockchain(data: &str) -> Option<String> {
    Blockchain::create(data.to_string()).map_or(None, |chain| {
        if let Some(storage) = build_session_storage() {
            let is_done = insert_blockchain(storage, &chain);
            if is_done {
                return serde_json::to_string(&chain).ok();
            }
        }
        return None;
    })
}

#[wasm_bindgen]
pub fn create_block(data: &str) -> Option<String> {
    if let Some(blockchain) = build_session_storage().map_or(None, |storage| {
        return get_block_chain(&storage);
    }) {
        return blockchain.create_block(data.to_string()).and_then(|block| {
            if let Some(json) = serde_json::to_string(&block).ok() {
                return Some(json)
            }
            None
        });
    }
    None
}

#[wasm_bindgen]
pub fn insert_block(block_json: &str) -> bool {
    if let Ok(block) = serde_json::from_str(block_json) {
        return build_session_storage().map_or(false, |storage| {
            if let Some(mut blockchain) = get_block_chain(&storage) {
                blockchain.insert_block(block);

                if blockchain.validate_blockchain() {
                    return insert_blockchain(storage, &blockchain);
                }
            }
            return false;
        });
    }
    false
}

fn build_session_storage() -> Option<Storage> {
    if let Some(window) = web_sys::window() {
         if let Ok(Some(storage)) = window.session_storage() {
             return Some(storage);
         }
    }
    None
}

fn get_block_chain(storage: &Storage) -> Option<Blockchain> {
    if let Ok(Some(json)) = storage.get(KEY_BLOCKCHAIN) {
        return serde_json::from_str(&json).ok();
    }
    None
}

fn insert_blockchain(storage: Storage, blockchain: &Blockchain) -> bool {
    if let Ok(json) = serde_json::to_string(blockchain) {
        return match storage.set_item(KEY_BLOCKCHAIN, &json) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    false
}