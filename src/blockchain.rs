extern crate crypto;
extern crate chrono;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use std::time::{Duration, Instant};
use std::cell::{RefCell, Cell};


struct Transaction {
    fromAddress: String,
    toAddress:   String,
    amout:       u32
}
#[derive(Clone, Debug)]
pub struct Block {
    index:        String,
    previousHash: RefCell<String>,
    data:         String,
    timestamp:    String,
    hash:         RefCell<String>,
}

impl Block {
    pub fn new(index: &str, timestamp: &str, data: &str, previousHash: &str) -> Block {
        let newBlock = Block {
            index:        String::from(index),
            previousHash: RefCell::new(String::from(previousHash)),
            data:         String::from(data),            
            timestamp:    String::from(timestamp),
            hash:         RefCell::new(String::from("")),
        };

        newBlock.hash.replace(newBlock.calculateHash());
        newBlock
    }

    fn calculateHash(&self) -> String {
        let input = format!(
            "{}{}{}{}",
            self.index,
            self.clone().previousHash.into_inner(),
            self.timestamp,
            self.data
        );

        let mut sha = Sha256::new();
        sha.input_str(&input);
        sha.result_str()
    }

    // fn mineBlock(&self, difficulty: u32) {
    //     while self.hash.get()[..difficulty] != 0 {
    //         self.nonce.set(self.nonce.set + 1);
    //         self.hash.set(Block::calculateHash());
    //     }

    //     println!("Block MINED: {}", self.hash);
    // }
}

#[derive(Clone, Debug)]
pub struct Blockchain {
    chain: RefCell<Vec<Block>>,
    // difficulty:          u32,
    // pendingTransactions: Vec<_>,
    // miningReward:        u32,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: RefCell::new(vec![Blockchain::createGenesisBlock()]), //Blockchain::createGenesisBlock(),
            // difficulty:          2,
            // pendingTransactions: vec![],
            // miningReward:        100
        }
    }

    fn createGenesisBlock() -> Block {
        Block::new("0", "01/01/2017", "Genesis block", "0")
    }

    fn getLatestBlock(&self) -> Block {
        let cloneChain = self.clone();
        cloneChain.chain.into_inner().last().unwrap().clone()
    }

    pub fn addBlock(&self, newBlock: Block) {
        newBlock.previousHash.replace(self.getLatestBlock().hash.into_inner());
        newBlock.hash.replace(newBlock.calculateHash());
        self.chain.borrow_mut().push(newBlock);
    }

}