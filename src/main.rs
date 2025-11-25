use rustblockchain::core::block::new_block;
use rustblockchain::core::chain::init_chain;
use rustblockchain::core::transactions::{new_tx};

fn main() {
    let genesis = new_block(vec![], [0; 32], String::from("genesis"));

    let tx = new_tx([0; 20], [1; 20], 100, String::from("test1"));
    let tx2 = new_tx([0; 20], [1; 20], 100, String::from("test2"));

    let txs = vec![tx, tx2];
    let block = new_block(txs, genesis.previous_hash, String::from("salut"));

    let mut blockchain = init_chain();
    blockchain.add_block(genesis);
    blockchain.add_block(block);

    println!("{:#?}", blockchain);

}
