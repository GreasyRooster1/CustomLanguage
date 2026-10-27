use serde::Serialize;
use crate::ast::{Name, Node};
use crate::lexer::{NumberLiteralAssumption, Token};

#[derive(Debug,Serialize)]
pub struct BinOp{
    pub(crate) left: Node,
    pub(crate) op: Token,
    pub(crate) right: Node,
}
#[derive(Debug,Serialize)]
pub struct Number{
    pub(crate) assumption: NumberLiteralAssumption,
}
#[derive(Debug,Serialize)]
pub struct Function{
    pub(crate) name: Name,
    pub(crate) params: Vec<Parameter>,
    pub(crate) ret_type: Name,
    pub(crate) body: Node,
}
#[derive(Debug,Serialize)]
pub struct Parameter{
    pub(crate) name: Name,
    pub(crate) p_type: Name,
}
#[derive(Debug,Serialize)]
pub struct Identifier{
    pub(crate) name: Name,
}
#[derive(Debug,Serialize)]
pub struct BlockStatement{
    pub(crate) statements: Vec<Node>,
}
#[derive(Debug,Serialize)]
pub struct ReturnStatement{
    pub(crate) expression: Node,
}
#[derive(Debug,Serialize)]
pub struct Program{
    pub(crate) statements: Vec<Node>,
}
#[derive(Debug,Serialize)]
pub struct ForeverLoop{
    pub(crate) body: Node
}
#[derive(Debug,Serialize)]
pub struct ForLoop{
    pub(crate) name: Name,
    pub(crate) range: Node,
    pub(crate) body: Node,
}
#[derive(Debug,Serialize)]
pub struct Range{
    pub(crate) start: Node,
    pub(crate) stop: Node,
    pub(crate) step: Node,
}
#[derive(Debug,Serialize)]
pub struct If{
    pub(crate) condition: Node,
    pub(crate) body:Node,
}
#[derive(Debug,Serialize)]
pub struct Not {
    condition: Node,
}
#[derive(Debug,Serialize)]
pub struct Nop;