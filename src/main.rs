use rustblockchain::core::block::new_block;
use rustblockchain::core::chain::init_chain;
use rustblockchain::core::transactions::Tx;

fn main() {
    let genesis = new_block(vec![], [0; 32], String::from("genesis"));
    let tx = Tx {
        from: [0; 20],
        to: [0; 20],
        amount: 100,
        data: String::from("Premiere transaction"),
        signature: None,
    };
    let tx2 = Tx {
        from: [0; 20],
        to: [0; 20],
        amount: 150,
        data: String::from("deuxieme transaction"),
        signature: None,
    };

    let txs = vec![tx, tx2];
    let block = new_block(txs, genesis.previous_hash, String::from("salut"));

    let mut blockchain = init_chain();
    blockchain.add_block(genesis);
    blockchain.add_block(block);

    println!("{:#?}", blockchain);

}
