#![allow(non_snake_case)]
use crate::tree::Tree;

fn parens_tree(name: &str, sub: Tree) -> Tree
{
    let l_par = Tree::singleton("(");
    let r_par = Tree::singleton(")");
    Tree::new(name, vec![l_par, sub, r_par])
}


use gramma_rs::{*, lexer::{Symbol::*, *}};
use gramma_rs::parser::{ParserBase, ParseError, ActionCallback, ParsedData};
fn create_parser_base() -> ParserBase
{
let dfa = {
    let states = vec![
        (Some(UnitId(11)), vec![]),
        (Some(UnitId(4)), vec![]),
        (None, vec![(4, 3),(5, 0),(0, 4),(6, 1),(2, 6),(1, 5),]),
        (Some(UnitId(10)), vec![]),
        (Some(UnitId(8)), vec![]),
        (Some(UnitId(9)), vec![]),
        (Some(UnitId(9)), vec![]),
    ];
    let start_state = 2;
    let symbols = vec![
        Str(StrEdge(String::from("*"))),
        Range(RangeEdge('a', 'z')),
        Range(RangeEdge('0', '9')),
        Eps(EpsEdge),
        Str(StrEdge(String::from("("))),
        Str(StrEdge(String::from(")"))),
        Str(StrEdge(String::from("|"))),
    ];
DFA::from_vecs(states, start_state, symbols)
};
let named_units = vec![
    (String::from("rPar"), UnitId(11)),
    (String::from("Concat"), UnitId(5)),
    (String::from("Eps"), UnitId(0)),
    (String::from(":PseudoToken:"), UnitId(12)),
    (String::from("Kleene"), UnitId(6)),
    (String::from("Regex"), UnitId(3)),
    (String::from("EOI"), UnitId(1)),
    (String::from("NonEmpty"), UnitId(2)),
    (String::from("Or"), UnitId(4)),
    (String::from("Star"), UnitId(8)),
    (String::from("char"), UnitId(9)),
    (String::from("lPar"), UnitId(10)),
    (String::from("SubConcat"), UnitId(7)),
];
let tokens = vec![
    (UnitId(0), true), 
    (UnitId(1), false), 
    (UnitId(4), false), 
    (UnitId(8), false), 
    (UnitId(9), false), 
    (UnitId(10), false), 
    (UnitId(11), false), 
    (UnitId(12), false), 
];
let nterms = vec![
    (UnitId(2), vec![RuleId(1),RuleId(2),], String::from("Tree"), false),
    (UnitId(3), vec![RuleId(0),], String::from("Tree"), true),
    (UnitId(5), vec![RuleId(3),RuleId(4),], String::from("Vec<Tree>"), false),
    (UnitId(6), vec![RuleId(5),RuleId(6),], String::from("Tree"), false),
    (UnitId(7), vec![RuleId(7),RuleId(8),], String::from("Tree"), false),
];
let rules = vec![
    (UnitId(3), vec![UnitId(2),], vec![Some(String::from("ne")),], String::from(r##"{ Tree::new("Regex", vec![ne]) }"##)),
    (UnitId(2), vec![UnitId(2),UnitId(4),UnitId(5),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{
        Tree::new("NonEmpty", vec![
            lhs,
            Tree::singleton("|"),
            Tree::new("Concat", rhs)
        ])
    }"##)),
    (UnitId(2), vec![UnitId(5),], vec![Some(String::from("cc")),], String::from(r##"{ Tree::new("NonEmpty", cc) }"##)),
    (UnitId(5), vec![UnitId(5),UnitId(6),], vec![Some(String::from("cc")),Some(String::from("next")),], String::from(r##"{
        let mut cc = cc;
        cc.push(next);
        cc
    }"##)),
    (UnitId(5), vec![UnitId(6),], vec![Some(String::from("sub")),], String::from(r##"{ vec![sub] }"##)),
    (UnitId(6), vec![UnitId(7),UnitId(8),], vec![Some(String::from("sub")),None,], String::from(r##"{ Tree::new("Kleene", vec![sub, Tree::singleton("*")]) }"##)),
    (UnitId(6), vec![UnitId(7),], vec![Some(String::from("sub")),], String::from(r##"{ Tree::new("Kleene", vec![sub]) }"##)),
    (UnitId(7), vec![UnitId(9),], vec![Some(String::from("ch")),], String::from(r##"{ Tree::singleton(&ch) }"##)),
    (UnitId(7), vec![UnitId(10),UnitId(2),UnitId(11),], vec![None,Some(String::from("sub")),None,], String::from(r##"{ parens_tree("SubConcat", sub) }"##)),
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
    { RegexParser{ base: create_parser_base().set_sym::<_Data, Tree>() } }

    pub fn parse<'this, 'input>(&'this self, to_parse: &'input str) -> Result<Tree, ParseError<'this>>
    { self.base.create::<_Data, Tree>(to_parse).parse() }

    pub fn dump_info(&self)
    { self.base.registry().print_lalr_items() }
}

fn _NonEmpty_0(
    lhs: Tree, // NonEmpty
    _: String, // Or
    rhs: Vec<Tree>, // Concat
) -> Tree
{
        Tree::new("NonEmpty", vec![
            lhs,
            Tree::singleton("|"),
            Tree::new("Concat", rhs)
        ])
    }

fn _NonEmpty_1(
    cc: Vec<Tree>, // Concat
) -> Tree
{ Tree::new("NonEmpty", cc) }


fn _Regex_0(
    ne: Tree, // NonEmpty
) -> Tree
{ Tree::new("Regex", vec![ne]) }


fn _Concat_0(
    cc: Vec<Tree>, // Concat
    next: Tree, // Kleene
) -> Vec<Tree>
{
        let mut cc = cc;
        cc.push(next);
        cc
    }

fn _Concat_1(
    sub: Tree, // Kleene
) -> Vec<Tree>
{ vec![sub] }


fn _Kleene_0(
    sub: Tree, // SubConcat
    _: String, // Star
) -> Tree
{ Tree::new("Kleene", vec![sub, Tree::singleton("*")]) }

fn _Kleene_1(
    sub: Tree, // SubConcat
) -> Tree
{ Tree::new("Kleene", vec![sub]) }


fn _SubConcat_0(
    ch: String, // char
) -> Tree
{ Tree::singleton(&ch) }

fn _SubConcat_1(
    _: String, // lPar
    sub: Tree, // NonEmpty
    _: String, // rPar
) -> Tree
{ parens_tree("SubConcat", sub) }

// outer world

pub enum _Data
{
    _NonEmpty(Tree),
    _Regex(Tree),
    _Concat(Vec<Tree>),
    _Kleene(Tree),
    _SubConcat(Tree),
    _Token(String),
}

impl ActionCallback for _Data
{
    fn run_action(args: Vec<Self>, rule_id: RuleId) -> Self
    {
        let mut args = args.into_iter();
        match rule_id {
            RuleId(0) => match args.next().unwrap() {
                Self::_NonEmpty(_0) => Self::_Regex(_Regex_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(1) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_NonEmpty(_0),Self::_Token(_1),Self::_Concat(_2)) => Self::_NonEmpty(_NonEmpty_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(2) => match args.next().unwrap() {
                Self::_Concat(_0) => Self::_NonEmpty(_NonEmpty_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(3) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_Concat(_0),Self::_Kleene(_1)) => Self::_Concat(_Concat_0(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(4) => match args.next().unwrap() {
                Self::_Kleene(_0) => Self::_Concat(_Concat_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(5) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_SubConcat(_0),Self::_Token(_1)) => Self::_Kleene(_Kleene_0(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(6) => match args.next().unwrap() {
                Self::_SubConcat(_0) => Self::_Kleene(_Kleene_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(7) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_SubConcat(_SubConcat_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(8) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_NonEmpty(_1),Self::_Token(_2)) => Self::_SubConcat(_SubConcat_1(_0,_1,_2)),
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

