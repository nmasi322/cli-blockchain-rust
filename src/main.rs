#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain; // library

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("🛠 Enter a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);

    print!("🥲 Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);

    let diff = difficulty.trim().parse::<u32>().expect("❗️Difficulty must be an interger");
    println!("🛠 Generating genesis block, might take some time 🙂.....");

    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), diff);

    loop {
        println!("💬 Options: ");
        println!("💸 1. New Transaction");
        println!("🛠 2. Mine block");
        println!("😭 3. Change Difficulty");
        println!("💰 4. Change Reward");
        println!("🥲 0. Exit");

        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap(){
            0 => {
                println!("🥲 Exiting...");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("💰 Enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                print!("💸 Enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);

                print!("🤑 Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());

                match res {
                    true => println!("💸 New transaction added to block!"),
                    false => println!("😭 Failed to add transaction to block, please try again later!"),
                }
            },
            2 => {
                println!("🛠 Generating new block...");
                let res = chain.generate_new_block();
                match res {
                    true => println!("🚀 New block generated!"),
                    false => println!("🥲 Failed to add new block, please try again later!"),
                }
            },
            3 => {
                let mut new_diff = String::new();
                print!("😓 Enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);

                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("🛠 Difficulty updated successfully!"),
                    false => println!("😌 Failed to update difficulty, try again later!")
                }

            },
            4 => {
                let mut new_reward = String::new();
                print!("😁 Enter new reward: (Current reward is 30): ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);

                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("😁 Reward changed successfully!"),
                    false => println!("🥲 Failed to update reward, try again later!")
                }
            },
            _ => println!("\t 🫤 Invalid option, please retry"),
        }
    }

}
