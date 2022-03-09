use crate::ast::Block;

#[derive(Debug)]
pub enum FuncBody {
    Block(Block),
    Native(String),
}

#[derive(Debug)]
pub struct FuncDef {
    pub ident: String,
    pub ret_type: Option<String>,
    pub body: FuncBody,
}
