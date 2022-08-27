extern crate serde;
extern crate serde_json;
extern crate sha2;

use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write;

use chrono::prelude::*;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u32
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>
}

pub struct Chain {
    chain: Vec<Block>,
    current_tran: Vec<Transaction>,
    difficulty: u32,
    miner_address: String,
    reward: u32
}

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Chain {
        let mut chain = Chain{
            chain: Vec::new(),
            current_tran: Vec::new(),
            difficulty,
            miner_address,
            reward: 30
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: u32) -> bool {
        self.current_tran.push(Transaction{
            sender,
            receiver,
            amount
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last(){
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: u32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool{
        let header = Blockheader{
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.current_tran);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(current_tran: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &current_tran {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut hash_one = merkle.remove(0);
            let mut hash_two = merkle.remove(0);
            hash_one.push_str(&mut hash_two);
            let new_hash = Chain::hash(&hash_one);
            merkle.push(new_hash);
        }

        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[.. header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    }else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }

    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }
}