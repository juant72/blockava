use blockchainlib::*;

fn main() {
    let difficulty=0x00ffffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0, 
        now(),
        vec![0;32],
        vec![
            Transaction{
                inputs: vec![],
                outputs: vec![
                    transaction::Output{
                        to_addr: "Alice".to_owned(),
                        value: 50,
                    },
                    transaction::Output{
                        to_addr: "Bob".to_owned(),
                        value: 7,
                    },
                ]

        }
        ],
        difficulty
    );

    genesis_block.mine();
    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash=genesis_block.hash.clone();

    let mut blockchain = Blockchain{
        blocks: vec![genesis_block],
    };

    println!("Verify: {}", &blockchain.verify());

    for _i in 1..=10 {
        let mut block = Block::new(
            _i, 
            now(),  
            last_hash,
            0,
            "Another block".to_owned(),
            difficulty
        );

        block.mine();
        println!("Mined block {:?}", &block);

        last_hash=block.hash.clone();

        blockchain.blocks.push(block);

        println!("Verify: {}", &blockchain.verify());

    }


}
