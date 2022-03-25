#![allow(non_snake_case)]

use gramma_rs::{*, lexer::{Symbol::*, *}};
use gramma_rs::parser::{ParserBase, ParseError, ActionCallback, ParsedData};
fn create_parser_base() -> ParserBase
{
let dfa = {
    let states = vec![
        (None, vec![(0, 8),(6, 7),(4, 6),(8, 1),(7, 5),(1, 2),(5, 4),(3, 3),]),
        (Some(UnitId(4)), vec![]),
        (Some(UnitId(14)), vec![]),
        (Some(UnitId(13)), vec![(3, 3),]),
        (Some(UnitId(9)), vec![]),
        (Some(UnitId(6)), vec![]),
        (Some(UnitId(7)), vec![]),
        (Some(UnitId(11)), vec![]),
        (Some(UnitId(10)), vec![]),
    ];
    let start_state = 0;
    let symbols = vec![
        Str(StrEdge(String::from("("))),
        Str(StrEdge(String::from("."))),
        Eps(EpsEdge),
        Range(RangeEdge('0', '9')),
        Str(StrEdge(String::from("*"))),
        Str(StrEdge(String::from("/"))),
        Str(StrEdge(String::from(")"))),
        Str(StrEdge(String::from("-"))),
        Str(StrEdge(String::from("+"))),
    ];
DFA::from_vecs(states, start_state, symbols)
};
let named_units = vec![
    (String::from("Star"), UnitId(7)),
    (String::from("Plus"), UnitId(4)),
    (String::from("Eps"), UnitId(0)),
    (String::from("Minus"), UnitId(6)),
    (String::from("Num"), UnitId(13)),
    (String::from("Float"), UnitId(12)),
    (String::from("Dot"), UnitId(14)),
    (String::from("SubExpr"), UnitId(2)),
    (String::from("Fact"), UnitId(8)),
    (String::from(":PseudoToken:"), UnitId(15)),
    (String::from("Expr"), UnitId(3)),
    (String::from("Term"), UnitId(5)),
    (String::from("Div"), UnitId(9)),
    (String::from("LPar"), UnitId(10)),
    (String::from("RPar"), UnitId(11)),
    (String::from("EOI"), UnitId(1)),
];
let tokens = vec![
    (UnitId(0), true), 
    (UnitId(1), false), 
    (UnitId(4), false), 
    (UnitId(6), false), 
    (UnitId(7), false), 
    (UnitId(9), false), 
    (UnitId(10), false), 
    (UnitId(11), false), 
    (UnitId(13), false), 
    (UnitId(14), false), 
    (UnitId(15), false), 
];
let nterms = vec![
    (UnitId(2), vec![RuleId(1),RuleId(2),RuleId(3),RuleId(4),], String::from("f64"), false),
    (UnitId(3), vec![RuleId(0),], String::from("f64"), true),
    (UnitId(5), vec![RuleId(5),RuleId(6),RuleId(7),], String::from("f64"), false),
    (UnitId(8), vec![RuleId(8),RuleId(9),], String::from("f64"), false),
    (UnitId(12), vec![RuleId(10),RuleId(11),], String::from("f64"), false),
];
let rules = vec![
    (UnitId(3), vec![UnitId(2),], vec![Some(String::from("e")),], String::from(r##"{ e }"##)),
    (UnitId(2), vec![UnitId(2),UnitId(4),UnitId(5),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{ lhs + rhs }"##)),
    (UnitId(2), vec![UnitId(2),UnitId(6),UnitId(5),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{ lhs - rhs }"##)),
    (UnitId(2), vec![UnitId(5),], vec![Some(String::from("t")),], String::from(r##"{ t }"##)),
    (UnitId(2), vec![UnitId(6),UnitId(5),], vec![None,Some(String::from("t")),], String::from(r##"{ -t }"##)),
    (UnitId(5), vec![UnitId(5),UnitId(7),UnitId(8),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{ lhs * rhs }"##)),
    (UnitId(5), vec![UnitId(5),UnitId(9),UnitId(8),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{ lhs / rhs }"##)),
    (UnitId(5), vec![UnitId(8),], vec![Some(String::from("f")),], String::from(r##"{ f }"##)),
    (UnitId(8), vec![UnitId(10),UnitId(2),UnitId(11),], vec![None,Some(String::from("e")),None,], String::from(r##"{ e }"##)),
    (UnitId(8), vec![UnitId(12),], vec![Some(String::from("n")),], String::from(r##"{ n }"##)),
    (UnitId(12), vec![UnitId(13),UnitId(14),UnitId(13),], vec![Some(String::from("f")),None,Some(String::from("s")),], String::from(r##"{ format!("{f}.{s}").parse().unwrap() }"##)),
    (UnitId(12), vec![UnitId(13),], vec![Some(String::from("n")),], String::from(r##"{ n.parse().unwrap() }"##)),
];
let reg = RegistryBuilder::from_vecs(named_units, tokens, nterms, rules);
ParserBase::from_parts(dfa, reg)
}

pub struct ExprParser
{
    base: ParserBase
}

impl ExprParser
{
    pub fn new() -> Self
    { ExprParser{ base: create_parser_base() } }

    pub fn parse<'this, 'input>(&'this self, to_parse: &'input str) -> Result<f64, ParseError<'this>>
    { self.base.create::<_Data, f64>(to_parse).parse() }
}

fn _SubExpr_0(
    lhs: f64, // SubExpr
    _: String, // Plus
    rhs: f64, // Term
) -> f64
{ lhs + rhs }

fn _SubExpr_1(
    lhs: f64, // SubExpr
    _: String, // Minus
    rhs: f64, // Term
) -> f64
{ lhs - rhs }

fn _SubExpr_2(
    t: f64, // Term
) -> f64
{ t }

fn _SubExpr_3(
    _: String, // Minus
    t: f64, // Term
) -> f64
{ -t }


fn _Expr_0(
    e: f64, // SubExpr
) -> f64
{ e }


fn _Term_0(
    lhs: f64, // Term
    _: String, // Star
    rhs: f64, // Fact
) -> f64
{ lhs * rhs }

fn _Term_1(
    lhs: f64, // Term
    _: String, // Div
    rhs: f64, // Fact
) -> f64
{ lhs / rhs }

fn _Term_2(
    f: f64, // Fact
) -> f64
{ f }


fn _Fact_0(
    _: String, // LPar
    e: f64, // SubExpr
    _: String, // RPar
) -> f64
{ e }

fn _Fact_1(
    n: f64, // Float
) -> f64
{ n }


fn _Float_0(
    f: String, // Num
    _: String, // Dot
    s: String, // Num
) -> f64
{ format!("{f}.{s}").parse().unwrap() }

fn _Float_1(
    n: String, // Num
) -> f64
{ n.parse().unwrap() }

// outer world

pub enum _Data
{
    _SubExpr(f64),
    _Expr(f64),
    _Term(f64),
    _Fact(f64),
    _Float(f64),
    _Token(String),
}

impl ActionCallback for _Data
{
    fn run_action(args: Vec<Self>, rule_id: RuleId) -> Self
    {
        let mut args = args.into_iter();
        match rule_id {
            RuleId(0) => match args.next().unwrap() {
                Self::_SubExpr(_0) => Self::_Expr(_Expr_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(1) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_SubExpr(_0),Self::_Token(_1),Self::_Term(_2)) => Self::_SubExpr(_SubExpr_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(2) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_SubExpr(_0),Self::_Token(_1),Self::_Term(_2)) => Self::_SubExpr(_SubExpr_1(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(3) => match args.next().unwrap() {
                Self::_Term(_0) => Self::_SubExpr(_SubExpr_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(4) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_Term(_1)) => Self::_SubExpr(_SubExpr_3(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(5) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Term(_0),Self::_Token(_1),Self::_Fact(_2)) => Self::_Term(_Term_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(6) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Term(_0),Self::_Token(_1),Self::_Fact(_2)) => Self::_Term(_Term_1(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(7) => match args.next().unwrap() {
                Self::_Fact(_0) => Self::_Term(_Term_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(8) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_SubExpr(_1),Self::_Token(_2)) => Self::_Fact(_Fact_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(9) => match args.next().unwrap() {
                Self::_Float(_0) => Self::_Fact(_Fact_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(10) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_Token(_1),Self::_Token(_2)) => Self::_Float(_Float_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(11) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_Float(_Float_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            _ => unreachable!(),
        }
    }

    fn wrap_token(token_str: String) -> Self
    { Self::_Token(token_str) }
}


impl ParsedData<f64> for _Data
{
    fn extract_data(self) -> f64
    {
        match self {
            Self::_Expr(res) => res,
            _ => panic!("Couldn't extract f64"),
        }
    }

    fn sym_id() -> UnitId
    { UnitId(3) }
}

