#![allow(non_snake_case)]


use gramma_rs::{*, lexer::{Symbol::*, *}};
use gramma_rs::parser::{ParserBase, ParseError, ActionCallback, ParsedData};
fn create_parser_base() -> ParserBase
{
let dfa = {
    let states = vec![
        (Some(UnitId(5)), vec![]),
        (None, vec![(1, 2),(0, 0),]),
        (Some(UnitId(4)), vec![]),
    ];
    let start_state = 1;
    let symbols = vec![
        Str(StrEdge(String::from("b"))),
        Str(StrEdge(String::from("a"))),
        Eps(EpsEdge),
    ];
DFA::from_vecs(states, start_state, symbols)
};
let named_units = vec![
    (String::from("b"), UnitId(5)),
    (String::from("S"), UnitId(3)),
    (String::from("a"), UnitId(4)),
    (String::from("B"), UnitId(2)),
    (String::from(":PseudoToken:"), UnitId(6)),
    (String::from("Eps"), UnitId(0)),
    (String::from("EOI"), UnitId(1)),
];
let tokens = vec![
    (UnitId(0), true), 
    (UnitId(1), false), 
    (UnitId(4), false), 
    (UnitId(5), false), 
    (UnitId(6), false), 
];
let nterms = vec![
    (UnitId(2), vec![RuleId(1),RuleId(2),], String::from("usize"), false),
    (UnitId(3), vec![RuleId(0),], String::from("(usize, usize)"), true),
];
let rules = vec![
    (UnitId(3), vec![UnitId(2),UnitId(2),], vec![Some(String::from("f")),Some(String::from("s")),], String::from(r##"{ (f, s) }"##)),
    (UnitId(2), vec![UnitId(4),UnitId(2),], vec![None,Some(String::from("b")),], String::from(r##"{ 1 + b }"##)),
    (UnitId(2), vec![UnitId(5),], vec![None,], String::from(r##"{ 0 }"##)),
];
let reg = RegistryBuilder::from_vecs(named_units, tokens, nterms, rules);
ParserBase::from_parts(dfa, reg)
}

pub struct SParser
{
    base: ParserBase
}

impl SParser
{
    pub fn new() -> Self
    { SParser{ base: create_parser_base() } }

    pub fn parse<'this, 'input>(&'this self, to_parse: &'input str) -> Result<(usize, usize), ParseError<'this>>
    { self.base.create::<_Data, (usize, usize)>(to_parse).parse() }

    pub fn dump_info(&self)
    { self.base.registry().print_lalr_items() }
}

fn _B_0(
    _: String, // a
    b: usize, // B
) -> usize
{ 1 + b }

fn _B_1(
    _: String, // b
) -> usize
{ 0 }


fn _S_0(
    f: usize, // B
    s: usize, // B
) -> (usize, usize)
{ (f, s) }

// outer world

pub enum _Data
{
    _B(usize),
    _S((usize, usize)),
    _Token(String),
}

impl ActionCallback for _Data
{
    fn run_action(args: Vec<Self>, rule_id: RuleId) -> Self
    {
        let mut args = args.into_iter();
        match rule_id {
            RuleId(0) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_B(_0),Self::_B(_1)) => Self::_S(_S_0(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(1) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_B(_1)) => Self::_B(_B_0(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(2) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_B(_B_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            _ => unreachable!(),
        }
    }

    fn wrap_token(token_str: String) -> Self
    { Self::_Token(token_str) }
}


impl ParsedData<(usize, usize)> for _Data
{
    fn extract_data(self) -> (usize, usize)
    {
        match self {
            Self::_S(res) => res,
            _ => panic!("Couldn't extract (usize, usize)"),
        }
    }

    fn sym_id() -> UnitId
    { UnitId(3) }
}

