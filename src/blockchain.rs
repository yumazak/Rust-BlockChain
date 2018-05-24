extern crate crypto;
extern crate chrono;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::chrono::prelude::*;
use self::chrono::Utc;

use std::cell::{RefCell};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Transaction {
    from_address: String,
    to_address:   String,
    amount:       u32
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.from_address, self.to_address, self.amount)
    }
}

impl Transaction {
    pub fn new(from_address: &str, to_address: &str, amount: u32) -> Transaction{
        Transaction {
            from_address: String::from(from_address),
            to_address:   String::from(to_address),
            amount:      amount,
        }
    }
}

#[derive(Clone)]
pub struct Block {
    previous_hash: RefCell<String>,
    transactions:  RefCell<Vec<Transaction>>,
    timestamp:     String,
    hash:          RefCell<String>,
    nonce:         RefCell<u32>,    
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[previous_hash: {}, transactions: {:?}, timestamp: {}, hash: {}, nonce: {}]",
            self.previous_hash.borrow(), self.transactions.borrow(), self.timestamp, self.hash.borrow(), self.nonce.borrow()
        )
    }
}
impl Block {
    pub fn new(timestamp: &str, transactions: Vec<Transaction>, previous_hash: &str) -> Block {
        let new_block = Block {
            previous_hash: RefCell::new(String::from(previous_hash)),
            transactions:  RefCell::new(transactions),            
            timestamp:     String::from(timestamp),
            hash:          RefCell::new(String::from("")),
            nonce:         RefCell::new(0),
        };

        new_block.hash.replace(new_block.calculate_hash());
        new_block
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{:?}{}",
            self.previous_hash.borrow(),
            self.timestamp,
            self.transactions.borrow(),
            self.nonce.borrow(),
        );

        let mut sha = Sha256::new();
        sha.input_str(&input);
        sha.result_str()
    }

    fn mine_block(&self, difficulty: usize) {
        let mut difficulty_string = String::from("0");

        for _ in 0..difficulty - 1{
            difficulty_string.push_str("0");
        }

        while self.hash.borrow().as_str()[..difficulty] != difficulty_string {
            *self.nonce.borrow_mut() += 1;
            self.hash.replace(self.calculate_hash());
        }

        println!("Block MINED: {}", self.hash.borrow());
    }
}

#[derive(Clone, Debug)]
pub struct Blockchain {
    chain:                RefCell<Vec<Block>>,
    difficulty:           usize,
    pending_transactions: RefCell<Vec<Transaction>>,
    mining_reward:        u32,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain:                RefCell::new(vec![Blockchain::create_genesis_block()]),
            difficulty:           2,
            pending_transactions: RefCell::new(vec![]),
            mining_reward:        100
        }
    }

    fn create_genesis_block() -> Block {
        Block::new(Utc.ymd(2017, 1, 1).to_string().as_str(), vec![], "0")
    }

    fn get_latest_block(&self) -> Block {
        self.chain.borrow().last().unwrap().clone()
    }

    pub fn mine_pending_transactions(&self, mining_reward_address: &str) {
        let block = Block::new(
            Utc::now().to_string().as_str(),
            self.pending_transactions.borrow().to_vec(),
            self.get_latest_block().hash.into_inner().as_str()
        );
        println!("{}", Utc::now().timestamp());
        block.mine_block(self.difficulty);

        println!("Block succesfully mined!");
        self.chain.borrow_mut().push(block);

        self.pending_transactions.replace(vec![Transaction::new("", mining_reward_address, self.mining_reward)]);
    }

    pub fn create_transaction(&self, transaction: Transaction) {
        self.pending_transactions.borrow_mut().push(transaction);
    }

    pub fn get_balance_of_address(&self, address: &str) -> u32{
        let mut balance = 0;

        for block in self.chain.borrow().iter()  {
            for trans in block.transactions.borrow().iter() {
                if trans.from_address == address {
                    balance -= trans.amount;
                }

                if trans.to_address == address {
                    balance += trans.amount;
                }
            }
        }

        balance
    }

    pub fn get_chain(&self) -> Vec<Block> {
        self.chain.borrow().to_vec()
    }

    fn is_chain_valid(&self) -> bool {
        for i in 0..self.chain.borrow().len()  {
            let current_block = &self.chain.borrow()[i];
            let previous_block = &self.chain.borrow()[i - 1];

            if current_block.hash.borrow().as_str() != current_block.calculate_hash().as_str() {
                return false
            }
            
            if current_block.previous_hash.borrow().as_str() != previous_block.hash.borrow().as_str() {
                return false
            }
        }

        true
    }
}