#![allow(non_snake_case)]

use gramma_rs::{*, lexer::{Symbol::*, *}};
use gramma_rs::parser::{ParserBase, ParseError, ActionCallback, ParsedData};
fn create_parser_base() -> ParserBase
{
let dfa = {
    let states = vec![
        (None, vec![(6, 1),(8, 7),(7, 4),(1, 3),(2, 8),(4, 5),(3, 2),(5, 6),]),
        (Some(UnitId(10)), vec![]),
        (Some(UnitId(8)), vec![]),
        (Some(UnitId(3)), vec![]),
        (Some(UnitId(6)), vec![]),
        (Some(UnitId(14)), vec![]),
        (Some(UnitId(13)), vec![(5, 6),]),
        (Some(UnitId(11)), vec![]),
        (Some(UnitId(5)), vec![]),
    ];
    let start_state = 0;
    let symbols = vec![
        Eps(EpsEdge),
        Str(StrEdge(String::from("+"))),
        Str(StrEdge(String::from("-"))),
        Str(StrEdge(String::from("/"))),
        Str(StrEdge(String::from("."))),
        Range(RangeEdge('0', '9')),
        Str(StrEdge(String::from("("))),
        Str(StrEdge(String::from("*"))),
        Str(StrEdge(String::from(")"))),
    ];
DFA::from_vecs(states, start_state, symbols)
};
let named_units = vec![
    (String::from("Div"), UnitId(8)),
    (String::from("RPar"), UnitId(11)),
    (String::from("EOI"), UnitId(1)),
    (String::from("LPar"), UnitId(10)),
    (String::from("Num"), UnitId(13)),
    (String::from("Dot"), UnitId(14)),
    (String::from("NegFloat"), UnitId(9)),
    (String::from("Eps"), UnitId(0)),
    (String::from("Expr"), UnitId(2)),
    (String::from("Float"), UnitId(12)),
    (String::from("Plus"), UnitId(3)),
    (String::from(":PseudoToken:"), UnitId(15)),
    (String::from("Term"), UnitId(4)),
    (String::from("Factor"), UnitId(7)),
    (String::from("Minus"), UnitId(5)),
    (String::from("Star"), UnitId(6)),
];
let tokens = vec![
    (UnitId(0), true), 
    (UnitId(1), false), 
    (UnitId(3), false), 
    (UnitId(5), false), 
    (UnitId(6), false), 
    (UnitId(8), false), 
    (UnitId(10), false), 
    (UnitId(11), false), 
    (UnitId(13), false), 
    (UnitId(14), false), 
    (UnitId(15), false), 
];
let nterms = vec![
    (UnitId(2), vec![RuleId(0),RuleId(1),RuleId(2),], String::from("f64"), true),
    (UnitId(4), vec![RuleId(3),RuleId(4),RuleId(5),], String::from("f64"), false),
    (UnitId(7), vec![RuleId(6),RuleId(7),], String::from("f64"), false),
    (UnitId(9), vec![RuleId(8),RuleId(9),], String::from("f64"), false),
    (UnitId(12), vec![RuleId(10),RuleId(11),], String::from("f64"), false),
];
let rules = vec![
    (UnitId(2), vec![UnitId(2),UnitId(3),UnitId(4),], vec![Some(String::from("l")),None,Some(String::from("r")),], String::from(r##"{ l + r }"##)),
    (UnitId(2), vec![UnitId(2),UnitId(5),UnitId(4),], vec![Some(String::from("l")),None,Some(String::from("r")),], String::from(r##"{ l - r }"##)),
    (UnitId(2), vec![UnitId(4),], vec![Some(String::from("sub")),], String::from(r##"{ sub }"##)),
    (UnitId(4), vec![UnitId(4),UnitId(6),UnitId(7),], vec![Some(String::from("l")),None,Some(String::from("r")),], String::from(r##"{ l * r }"##)),
    (UnitId(4), vec![UnitId(4),UnitId(8),UnitId(7),], vec![Some(String::from("l")),None,Some(String::from("r")),], String::from(r##"{ l / r }"##)),
    (UnitId(4), vec![UnitId(7),], vec![Some(String::from("sub")),], String::from(r##"{ sub }"##)),
    (UnitId(7), vec![UnitId(9),], vec![Some(String::from("f")),], String::from(r##"{ f }"##)),
    (UnitId(7), vec![UnitId(10),UnitId(2),UnitId(11),], vec![None,Some(String::from("sub")),None,], String::from(r##"{ sub }"##)),
    (UnitId(9), vec![UnitId(5),UnitId(12),], vec![None,Some(String::from("f")),], String::from(r##"{ -f }"##)),
    (UnitId(9), vec![UnitId(12),], vec![Some(String::from("f")),], String::from(r##"{ f }"##)),
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
    { ExprParser{ base: create_parser_base().set_sym::<_Data, f64>() } }

    pub fn parse<'this, 'input>(&'this self, to_parse: &'input str) -> Result<f64, ParseError<'this>>
    { self.base.create::<_Data, f64>(to_parse).parse() }

    pub fn dump_info(&self)
    { self.base.registry().print_lalr_items() }
}

fn _Expr_0(
    l: f64, // Expr
    _: String, // Plus
    r: f64, // Term
) -> f64
{ l + r }

fn _Expr_1(
    l: f64, // Expr
    _: String, // Minus
    r: f64, // Term
) -> f64
{ l - r }

fn _Expr_2(
    sub: f64, // Term
) -> f64
{ sub }


fn _Term_0(
    l: f64, // Term
    _: String, // Star
    r: f64, // Factor
) -> f64
{ l * r }

fn _Term_1(
    l: f64, // Term
    _: String, // Div
    r: f64, // Factor
) -> f64
{ l / r }

fn _Term_2(
    sub: f64, // Factor
) -> f64
{ sub }


fn _Factor_0(
    f: f64, // NegFloat
) -> f64
{ f }

fn _Factor_1(
    _: String, // LPar
    sub: f64, // Expr
    _: String, // RPar
) -> f64
{ sub }


fn _NegFloat_0(
    _: String, // Minus
    f: f64, // Float
) -> f64
{ -f }

fn _NegFloat_1(
    f: f64, // Float
) -> f64
{ f }


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
    _Expr(f64),
    _Term(f64),
    _Factor(f64),
    _NegFloat(f64),
    _Float(f64),
    _Token(String),
}

impl ActionCallback for _Data
{
    fn run_action(args: Vec<Self>, rule_id: RuleId) -> Self
    {
        let mut args = args.into_iter();
        match rule_id {
            RuleId(0) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Expr(_0),Self::_Token(_1),Self::_Term(_2)) => Self::_Expr(_Expr_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(1) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Expr(_0),Self::_Token(_1),Self::_Term(_2)) => Self::_Expr(_Expr_1(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(2) => match args.next().unwrap() {
                Self::_Term(_0) => Self::_Expr(_Expr_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(3) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Term(_0),Self::_Token(_1),Self::_Factor(_2)) => Self::_Term(_Term_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(4) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Term(_0),Self::_Token(_1),Self::_Factor(_2)) => Self::_Term(_Term_1(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(5) => match args.next().unwrap() {
                Self::_Factor(_0) => Self::_Term(_Term_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(6) => match args.next().unwrap() {
                Self::_NegFloat(_0) => Self::_Factor(_Factor_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(7) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_Expr(_1),Self::_Token(_2)) => Self::_Factor(_Factor_1(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(8) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_Float(_1)) => Self::_NegFloat(_NegFloat_0(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(9) => match args.next().unwrap() {
                Self::_Float(_0) => Self::_NegFloat(_NegFloat_1(_0)),
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
    { UnitId(2) }
}

