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
    index:         String,
    previous_hash: RefCell<String>,
    data:          String,
    timestamp:     String,
    hash:          RefCell<String>,
    nonce:         RefCell<u32>,    
}

impl Block {
    pub fn new(index: &str, timestamp: &str, data: &str, previous_hash: &str) -> Block {
        let new_block = Block {
            index:         String::from(index),
            previous_hash: RefCell::new(String::from(previous_hash)),
            data:          String::from(data),            
            timestamp:     String::from(timestamp),
            hash:          RefCell::new(String::from("")),
            nonce:         RefCell::new(0),
        };

        new_block.hash.replace(new_block.calculate_hash());
        new_block
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index,
            self.previous_hash.borrow(),
            self.timestamp,
            self.data,
            self.nonce.borrow(),
        );

        let mut sha = Sha256::new();
        sha.input_str(&input);
        sha.result_str()
    }

    fn mine_block(&self, difficulty: usize) {
        let mut difficultyString = String::from("0");
        for i in 0..difficulty - 1{
            difficultyString.push_str("0");
        }
        while self.hash.borrow().as_str()[..difficulty] != difficultyString {
            *self.nonce.borrow_mut() += 1;
            self.hash.replace(self.calculate_hash());
        }

        println!("Block MINED: {}", self.hash.borrow());
    }
}

#[derive(Clone, Debug)]
pub struct Blockchain {
    chain:      RefCell<Vec<Block>>,
    difficulty: usize,
    // pendingTransactions: Vec<_>,
    // miningReward:        u32,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain:      RefCell::new(vec![Blockchain::create_genesis_block()]), //Blockchain::create_genesis_block(),
            difficulty: 2,
            // pendingTransactions: vec![],
            // miningReward:        100
        }
    }

    fn create_genesis_block() -> Block {
        Block::new("0", "01/01/2017", "Genesis block", "0")
    }

    fn get_latest_block(&self) -> Block {
        self.chain.borrow().last().unwrap().clone()
    }

    pub fn add_block(&self, new_block: Block) {
        new_block.previous_hash.replace(self.get_latest_block().hash.into_inner());
        // new_block.hash.replace(new_block.calculate_hash());
        new_block.mine_block(self.difficulty);
        self.chain.borrow_mut().push(new_block);
    }
    

}