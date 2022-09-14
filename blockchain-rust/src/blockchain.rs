use super::*;
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            // index check
            if block.index != (i as u32) {
                println!("Index mismatch {} != {}", &block.index, &i);
                return false;
            } else if
                //Difficulty check
                !block::check_difficulty(&block.hash(), block.difficulty)
            {
                println!("difficulty fail");
                return false;
            } else if i != 0 {
                // Not genesis block
                // time check
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                } else if
                    // prev hash check
                    block.prev_block_hash != prev_block.hash
                {
                    println!("Hash mismatch");
                    return false;
                }
            } else {
                // genesis block
                // genesis block doest have prev blockhash
                if block.prev_block_hash != vec![0;32] {
                    println!("Genesis block prev_block_hash invalid");
                    return false;
                }
            }
        }
        true
    }
}