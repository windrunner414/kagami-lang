use crate::ast::Block;

#[derive(Debug)]
pub struct FuncDef {
    pub ident: String,
    pub func_block: FuncBlock,
}

#[derive(Debug)]
pub struct FuncBlock {
    pub ret_type: String,
    pub block: Block,
}
