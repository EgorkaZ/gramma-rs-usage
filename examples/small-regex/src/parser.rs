use std::iter::Peekable;

use crate::{tree::Tree, lexer::{Token::{self, *}, Tokenize, TokenizeError}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ParseError
{
    Unexpected(Token),
    UnexpectedEOI,
    ExpectedEOI,
    TokError(TokenizeError),
}

type MbToken = Result<Token, TokenizeError>;

struct State<It: Iterator<Item = MbToken>>
{
    iter: It,
    parsed: Tree
}

pub fn parse<It>(iter: It) -> Result<Tree, ParseError>
    where It: Iterator<Item = char>
{
    let iter = iter.tokenize().peekable();
    p_reg(iter).and_then(|State{ mut iter, parsed }| match iter.peek() {
        Some(_) => Err(ParseError::ExpectedEOI),
        None => Ok(parsed),
    })
}

fn p_reg<It>(mut iter: Peekable<It>) -> Result<State<Peekable<It>>, ParseError>
    where It: Iterator<Item = MbToken>
{
    match iter.peek() {
            // S -> NE
        Some(Ok(Char(_) | LParen)) => {
            p_non_empty(iter).map(|State{ iter, parsed }| {
                let tree = Tree::new("S", vec![parsed]);
                State { iter, parsed: tree }
            })
        },
        None | Some(Ok(RParen)) => {
            // S -> eps
            if iter.peek().is_none() {
                iter.next();
            }
            let eps_tree = Tree::singleton("eps");
            Ok(State { iter, parsed: Tree::new("S", vec![eps_tree]) })
        },
        Some(Ok(other)) => Err(ParseError::Unexpected(*other)),
        Some(Err(tok_err)) => Err(ParseError::TokError(*tok_err)),
    }
}

fn p_non_empty<It>(mut iter: Peekable<It>) -> Result<State<Peekable<It>>, ParseError>
    where It: Iterator<Item = MbToken>
{
    match iter.peek() {
        Some(Ok(Char(ch))) => {
            // NE -> a K
            let ch = *ch;
            iter.next();
            // a
            let char_tree = Tree::singleton(&ch.to_string());
            // K
            p_kleene(iter)
                .map(|State{ iter, parsed: kl_tree }| {
                    State { iter, parsed: Tree::new("NE", vec![char_tree, kl_tree]) }
                })
        },
        Some(Ok(LParen)) => {
            // NE -> (NE) K
            iter.next();
            // (
            let lp_tree = Tree::singleton("(");
            // NE
            p_non_empty(iter)
                .and_then(|State{ mut iter, parsed: in_ne_tree }| {
            // )
                    match iter.next() {
                        Some(Ok(RParen)) => Ok((iter, in_ne_tree, Tree::singleton(")"))),
                        Some(Ok(other)) => Err(ParseError::Unexpected(other)),
                        Some(Err(tok_err)) => Err(ParseError::TokError(tok_err)),
                        None => Err(ParseError::UnexpectedEOI)
                    }
                }).and_then(|(iter, in_ne_tree, rp_tree)| {
            // K
                    p_kleene(iter)
                        .map(|State { iter, parsed: kl_tree }| {
                            let tree = Tree::new("NE", vec![lp_tree, in_ne_tree, rp_tree, kl_tree]);
                            State { iter, parsed: tree }
                        })
                })
        },
        Some(Ok(other)) => Err(ParseError::Unexpected(*other)),
        Some(Err(tok_err)) => Err(ParseError::TokError(*tok_err)),
        None => Err(ParseError::UnexpectedEOI)
    }
}

fn p_tail<It>(mut iter: Peekable<It>) -> Result<State<Peekable<It>>, ParseError>
    where It: Iterator<Item = MbToken>
{
    match iter.peek() {
        Some(Ok(Pipe)) => {
            // T -> | NE
            iter.next();
            // |
            let p_tree = Tree::singleton("|");
            // NE
            p_non_empty(iter)
                .map(|State{ iter, parsed: ne_tree }| {
                    let tree = Tree::new("T", vec![p_tree, ne_tree]);
                    State { iter, parsed: tree }
                })
        },
            // T -> S
        None | Some(Ok(Char(_) | LParen | RParen)) => {
            p_reg(iter).map(|State{ iter, parsed }| {
                let tree = Tree::new("T", vec![parsed]);
                State { iter, parsed: tree }
            })
        },
        Some(Ok(other)) => Err(ParseError::Unexpected(*other)),
        Some(Err(tok_err)) => Err(ParseError::TokError(*tok_err)),
    }
}

fn p_kleene<It>(mut iter: Peekable<It>) -> Result<State<Peekable<It>>, ParseError>
    where It: Iterator<Item = MbToken>
{
    match iter.peek() {
        Some(Ok(Star)) => {
            // K -> * T
            iter.next();
            // *
            let s_tree = Tree::singleton("*");
            // T
            p_tail(iter)
                .map(|State{ iter, parsed: in_kl_tree }| {
                    let tree = Tree::new("K", vec![s_tree, in_kl_tree]);
                    State { iter, parsed: tree }
                })
        },
            // K -> T
        None | Some(Ok(Pipe | Char(_) | LParen | RParen)) => {
            p_tail(iter).map(|State{ iter, parsed }| {
                let tree = Tree::new("K", vec![parsed]);
                State { iter, parsed: tree }
            })
        },
        Some(Err(tok_err)) => Err(ParseError::TokError(*tok_err)),
    }
}
