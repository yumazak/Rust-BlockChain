extern crate crypto;
extern crate chrono;

use crypto::sha2::Sha256;
use chrono::prelude::*;
use std::time::{Duration, Instant};

struct Transaction {
    fromAddress: String,
    toAddress:   String,
    amout:       u32
}

struct Block {
    previousHash: String,
    timestamp:    String,
    transactions: Transaction,
    hash:         Cell<String>,
    nonce:        Cell<u32>,
}

impl Block {
    fn new(previousHash: String, timestamp: String, transactions: Transaction) -> Block {
        Block {
            previousHash: previousHash,
            timestamp:    timestamp,
            transactions: transactions,
            hash:         Cell::new(Block::calculateHash()),
            nonce:        Cell::new(0)
        }
    }

    fn calculateHash(&self) -> String {
        let intput = format!(
            "{}{}{}{}",
            self.previousHash,
            self.timestamp,
            self.transactions,
            self.nonce.get()
        );

        let mut sha = Sha256::new();
        sha.input_str(input);
        sha.result_str()
    }

    fn mineBlock(&self, difficulty: u32) {
        while self.hash[..difficulty] != 0 {
            self.nonce.set(self.nonce.set + 1);
            self.hash.set(Block::calculateHash());
        }

        println!("Block MINED: {}", self.hash);
    }
}

#[derive(Debug)]
struct Blockchain {
    chain:               Vec<Block>,
    difficulty:          u32,
    pendingTransactions: Vec<_>,
    miningReward:        u32,
}

impl Blockchain {
    fn new(difficulty: u32, miningReward: u32) -> Blockchain {
        Blockchain {
            chain:               Blockchain::createGenesisBlock(),
            difficulty:          2,
            pendingTransactions: vec![],
            miningReward:        100
        }
    }

    fn createGenesisBlock() -> Block{
        let now = Instant::now();
        Block::new(Instant::now(), vec![], "0");
    }

    fn getLatestBlock(&self) -> Vec<Block> {
        let end = &self.chain.length - 1;
        self.chain[..end]
    }

}