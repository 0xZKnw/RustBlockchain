use crate::core::block::Block;

#[derive(Debug)]
pub struct Chain {
    pub blocks : Vec<Block>,
}

impl Chain {
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

pub fn init_chain() -> Chain {
    Chain {
        blocks: vec![]
    }
}