mod blockchain;

use blockchain::{Blockchain, Block};
fn main() {
    let Coin = Blockchain::new();
    Coin.addBlock(Block::new("1", "10/07/2017", "data", ""));
    Coin.addBlock(Block::new("1", "10/07/2017", "data", ""));
    println!("{:?}", Coin);
}
