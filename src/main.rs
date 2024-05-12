use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hex::encode as hex_encode;

#[derive(Debug, Clone)]
struct Block {
    index: usize,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: usize, timestamp: u64, previous_hash: String, data: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            previous_hash: previous_hash.clone(),
            hash: String::new(), // We'll calculate this later
            data,
            nonce: 0,
        };

        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let headers = format!("{}:{}:{}:{}:{}", self.index, self.timestamp, self.previous_hash, self.data, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(headers);
        let result = hasher.finalize();
        hex_encode(result)
    }

    fn mine(&mut self, difficulty: usize) {
        loop {
            self.hash = self.calculate_hash();
            if &self.hash[..difficulty] == &"0".repeat(difficulty) {
                break;
            } else {
                self.nonce += 1;
            }
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, now(), String::from("0"), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let mut new_block = Block::new(
            self.chain.len(),
            now(),
            self.chain.last().unwrap().hash.clone(),
            data,
        );

        new_block.mine(4); // Assuming a difficulty of 4 leading zeros
        self.chain.push(new_block);
    }

    fn validate_chain(&self) -> bool {
        if self.chain.len() < 2 {
            return true;
        }

        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Recalculate the current block's hash (considering its transactions)
            // and ensure it matches the stored hash.
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            // Check if the current block's previous hash matches the previous block's hash
            if current_block.previous_hash != previous_block.hash {
                return false;
            }

            // Here, you would also verify the Merkle root if it were explicitly stored and used
            // This example does not yet include a Merkle root in the block structure.
        }

        true
    }
}

#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

impl Transaction {
    fn create_merkle_root(transactions: &[Transaction]) -> String {
        let mut hashes = transactions.iter()
            .map(|transaction| {
                let transaction_data = format!("{:?}{:?}{:?}", transaction.sender, transaction.receiver, transaction.amount);
                let mut hasher = Sha256::new();
                hasher.update(transaction_data);
                hex::encode(hasher.finalize())
            })
            .collect::<Vec<String>>();
    
        while hashes.len() > 1 {
            let mut temp_hashes = Vec::new();
            for i in (0..hashes.len()).step_by(2) {
                if i + 1 < hashes.len() {
                    let combined = format!("{}{}", hashes[i], hashes[i + 1]);
                    let mut hasher = Sha256::new();
                    hasher.update(combined);
                    temp_hashes.push(hex::encode(hasher.finalize()));
                } else {
                    temp_hashes.push(hashes[i].clone());
                }
            }
            hashes = temp_hashes;
        }
    
        hashes[0].clone()
    }
    
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(String::from("Block 1"));
    blockchain.add_block(String::from("Block 2"));

    // Placeholder to display the blockchain
    println!("{:?}", blockchain.chain);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mining() {
        let mut block = Block::new(0, now(), String::from("0"), String::from("Test Block"));
        block.mine(2); // Adjust difficulty as needed
        assert!(block.hash.starts_with("00"), "Block wasn't mined correctly: {}", block.hash);
    }

    #[test]
    fn test_blockchain_integrity() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("First Block"));
        blockchain.add_block(String::from("Second Block"));

        assert!(blockchain.validate_chain(), "Blockchain integrity compromised!");
    }
}
