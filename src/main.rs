mod blockchain;

use blockchain::{Blockchain, Block, Transaction};
fn main() {
    let coin = Blockchain::new();

    coin.create_transaction(Transaction::new("address1", "address2", 100));
    coin.create_transaction(Transaction::new("address2", "address1", 100));

    println!("\n Starting the miner...");
    coin.mine_pending_transactions("xaviers-address");

    println!("\nBalance of xavier is {}", coin.get_balance_of_address("xaviers-address"));

    println!("\n Starting the miner again...");
    coin.mine_pending_transactions("xaviers-address");

    println!("\nBalance of xavier is {}", coin.get_balance_of_address("xaviers-address"));
}
