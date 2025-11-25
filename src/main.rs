use rustblockchain::core::block::new_block;
use rustblockchain::core::transactions::Tx;

fn main() {
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

    let previous_hash = [0; 32];

    let block = new_block(txs, previous_hash);

    println!("Bloc créé avec succès !");
    println!("Hash du bloc: {:?}", hex::encode(block.hash));
    println!("Transactions: {:?}", block.txs);
}
