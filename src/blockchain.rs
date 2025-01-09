use std::error::Error;
use crate::block::Block;

const GENESIS_HASH: [u8; 32] = [0; 32];

pub struct Blockchain {
    vector: Vec<Block>,
}

impl Blockchain {
    pub fn create(data: String) -> Result<Blockchain, Box<dyn Error>> {
        let mut vector: Vec<Block> = Vec::new();
        let genesis_block = Block::create(0, GENESIS_HASH, data)?;

        vector.push(genesis_block);
        Ok(Blockchain {
            vector
        })
    }

    pub fn create_block(&self, data: String) -> Option<Block> {
        let index = self.vector.len();
        let last_hash = self.vector.last()?.last_hash;

        Block::create(index, last_hash, data).ok()
    }

    pub fn validate_blockchain(&self) -> bool {
        let mut last_hash: [u8; 32] = GENESIS_HASH;
        self.vector.iter().all(|block| {
            if last_hash != block.last_hash { return false };
            last_hash = block.last_hash;
            return true;
        })
    }

    pub fn insert_block(&mut self, block: Block) {
        self.vector.push(block)
    }
}
