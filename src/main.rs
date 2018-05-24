mod blockchain;

use blockchain::{Blockchain, Block};
fn main() {
    let coin = Blockchain::new();

    println!("Mining block 1...");
    coin.add_block(Block::new("1", "10/07/2017", "data", ""));

    println!("Mining block 2");
    coin.add_block(Block::new("1", "10/07/2017", "data", ""));
    println!("{:?}", coin);
}
