use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tree
{
    Primal(Primal),
    BinOp(BinOper, Box<Tree>, Box<Tree>),
    UnOp(UnOper, Box<Tree>),
    IfThenElse(Box<Tree>, Box<Tree>, Option<Box<Tree>>),
    Let(String, Box<Tree>, Box<Tree>),
    Action(Action),
    Look(Action, Box<Tree>),
    FunCall(Box<Tree>, Box<Tree>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Primal
{
    Bool(bool),
    Numeric(i32),
    Variable(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action
{
    Print(Box<Tree>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinOper
{
    // arithmetic
    Add,
    Sub,
    Mul,
    Div,
    // compare
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    // logic
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnOper
{
    Not
}

impl Display for Primal
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primal::Bool(true) => f.pad("true"),
            Primal::Bool(false) => f.pad("false"),
            Primal::Numeric(num) => f.pad(&format!("{}", num)),
            Primal::Variable(name) => f.pad(&name),
        }
    }
}

impl Display for BinOper
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOper::Add => f.write_char('+'),
            BinOper::Sub => f.write_char('-'),
            BinOper::Mul => f.write_char('*'),
            BinOper::Div => f.write_char('/'),
            BinOper::Eq => f.write_str("=="),
            BinOper::Ne => f.write_str("!="),
            BinOper::Lt => f.write_char('<'),
            BinOper::Gt => f.write_char('>'),
            BinOper::Le => f.write_str("<="),
            BinOper::Ge => f.write_str(">="),
            BinOper::And => f.write_str("&"),
            BinOper::Or => f.write_str("|"),
        }
    }
}

impl Display for UnOper
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOper::Not => f.write_char('!'),
        }
    }
}
