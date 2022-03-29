#![allow(non_snake_case)]
use crate::tree::Tree;

fn mb_push<T>(mut vec: Vec<T>, mb_el: Option<T>) -> Vec<T>
{
    if let Some(el) = mb_el {
        vec.push(el);
    }
    vec
}


use gramma_rs::{*, lexer::{Symbol::*, *}};
use gramma_rs::parser::{ParserBase, ParseError, ActionCallback, ParsedData};
fn create_parser_base() -> ParserBase
{
let dfa = {
    let states = vec![
        (Some(UnitId(6)), vec![]),
        (Some(UnitId(8)), vec![]),
        (None, vec![(4, 4),(1, 0),(3, 1),(2, 3),(5, 5),]),
        (Some(UnitId(7)), vec![]),
        (Some(UnitId(5)), vec![]),
        (Some(UnitId(4)), vec![]),
    ];
    let start_state = 2;
    let symbols = vec![
        Eps(EpsEdge),
        Str(StrEdge(String::from(")"))),
        Str(StrEdge(String::from("|"))),
        Str(StrEdge(String::from("*"))),
        Str(StrEdge(String::from("("))),
        Range(RangeEdge('a', 'z')),
    ];
DFA::from_vecs(states, start_state, symbols)
};
let named_units = vec![
    (String::from("rPar"), UnitId(6)),
    (String::from("Chars"), UnitId(2)),
    (String::from(":PseudoToken:"), UnitId(9)),
    (String::from("char"), UnitId(4)),
    (String::from("EOI"), UnitId(1)),
    (String::from("Eps"), UnitId(0)),
    (String::from("lPar"), UnitId(5)),
    (String::from("Star"), UnitId(8)),
    (String::from("Or"), UnitId(7)),
    (String::from("Regex"), UnitId(3)),
];
let tokens = vec![
    (UnitId(0), true), 
    (UnitId(1), false), 
    (UnitId(4), false), 
    (UnitId(5), false), 
    (UnitId(6), false), 
    (UnitId(7), false), 
    (UnitId(8), false), 
    (UnitId(9), false), 
];
let nterms = vec![
    (UnitId(2), vec![RuleId(1),RuleId(2),], String::from("Tree"), false),
    (UnitId(3), vec![RuleId(0),], String::from("Tree"), true),
];
let rules = vec![
    (UnitId(3), vec![UnitId(2),], vec![Some(String::from("c")),], String::from(r##"{ Tree::new("Regex", vec![c]) }"##)),
    (UnitId(2), vec![UnitId(4),], vec![Some(String::from("single")),], String::from(r##"{ Tree::new("Chars", vec![Tree::singleton(&single)]) }"##)),
    (UnitId(2), vec![UnitId(4),UnitId(2),], vec![Some(String::from("fst")),Some(String::from("rest")),], String::from(r##"{ Tree::new("Chars", vec![Tree::singleton(&fst), rest]) }"##)),
];
let reg = RegistryBuilder::from_vecs(named_units, tokens, nterms, rules);
ParserBase::from_parts(dfa, reg)
}

pub struct RegexParser
{
    base: ParserBase
}

impl RegexParser
{
    pub fn new() -> Self
    { RegexParser{ base: create_parser_base() } }

    pub fn parse<'this, 'input>(&'this self, to_parse: &'input str) -> Result<Tree, ParseError<'this>>
    { self.base.create::<_Data, Tree>(to_parse).parse() }

    pub fn dump_info(&self)
    { self.base.registry().print_lalr_items() }
}

fn _Chars_0(
    single: String, // char
) -> Tree
{ Tree::new("Chars", vec![Tree::singleton(&single)]) }

fn _Chars_1(
    fst: String, // char
    rest: Tree, // Chars
) -> Tree
{ Tree::new("Chars", vec![Tree::singleton(&fst), rest]) }


fn _Regex_0(
    c: Tree, // Chars
) -> Tree
{ Tree::new("Regex", vec![c]) }

// outer world

pub enum _Data
{
    _Chars(Tree),
    _Regex(Tree),
    _Token(String),
}

impl ActionCallback for _Data
{
    fn run_action(args: Vec<Self>, rule_id: RuleId) -> Self
    {
        let mut args = args.into_iter();
        match rule_id {
            RuleId(0) => match args.next().unwrap() {
                Self::_Chars(_0) => Self::_Regex(_Regex_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(1) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_Chars(_Chars_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(2) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_Chars(_1)) => Self::_Chars(_Chars_1(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            _ => unreachable!(),
        }
    }

    fn wrap_token(token_str: String) -> Self
    { Self::_Token(token_str) }
}


impl ParsedData<Tree> for _Data
{
    fn extract_data(self) -> Tree
    {
        match self {
            Self::_Regex(res) => res,
            _ => panic!("Couldn't extract Tree"),
        }
    }

    fn sym_id() -> UnitId
    { UnitId(3) }
}

