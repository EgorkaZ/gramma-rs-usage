#![allow(non_snake_case)]
use crate::ast::{Tree, Action, BinOper, UnOper, Primal};


use gramma_rs::{*, lexer::{Symbol::*, *}};
use gramma_rs::parser::{ParserBase, ParseError, ActionCallback, ParsedData};
fn create_parser_base() -> ParserBase
{
let dfa = {
    let states = vec![
        (Some(UnitId(17)), vec![]),
        (Some(UnitId(15)), vec![]),
        (Some(UnitId(8)), vec![]),
        (Some(UnitId(7)), vec![(6, 20),(11, 11),(14, 6),(24, 3),]),
        (None, vec![(19, 15),(1, 21),(3, 12),(2, 2),(7, 25),(16, 23),(0, 10),(4, 22),(18, 18),(23, 24),(5, 13),(10, 0),(9, 1),(15, 17),(13, 19),(21, 7),(17, 5),(12, 9),(22, 8),(6, 27),(8, 14),(11, 16),(24, 26),]),
        (Some(UnitId(31)), vec![]),
        (Some(UnitId(7)), vec![(6, 20),(11, 11),(14, 6),(24, 3),]),
        (Some(UnitId(29)), vec![]),
        (Some(UnitId(23)), vec![]),
        (Some(UnitId(22)), vec![]),
        (Some(UnitId(21)), vec![]),
        (Some(UnitId(7)), vec![(6, 20),(11, 11),(14, 6),(24, 3),]),
        (Some(UnitId(35)), vec![]),
        (Some(UnitId(4)), vec![]),
        (Some(UnitId(16)), vec![]),
        (Some(UnitId(36)), vec![]),
        (Some(UnitId(34)), vec![(11, 16),]),
        (Some(UnitId(30)), vec![]),
        (Some(UnitId(20)), vec![]),
        (Some(UnitId(28)), vec![]),
        (Some(UnitId(7)), vec![(6, 20),(11, 11),(14, 6),(24, 3),]),
        (Some(UnitId(37)), vec![]),
        (Some(UnitId(24)), vec![]),
        (Some(UnitId(6)), vec![]),
        (Some(UnitId(25)), vec![]),
        (Some(UnitId(9)), vec![]),
        (Some(UnitId(7)), vec![(6, 20),(11, 11),(14, 6),(24, 3),]),
        (Some(UnitId(7)), vec![(6, 20),(11, 11),(14, 6),(24, 3),]),
    ];
    let start_state = 4;
    let symbols = vec![
        Str(StrEdge(String::from("!="))),
        Str(StrEdge(String::from("print"))),
        Str(StrEdge(String::from("call"))),
        Str(StrEdge(String::from("True"))),
        Str(StrEdge(String::from("<="))),
        Str(StrEdge(String::from("if"))),
        Str(StrEdge(String::from("_"))),
        Str(StrEdge(String::from("look"))),
        Str(StrEdge(String::from("|"))),
        Str(StrEdge(String::from("&"))),
        Str(StrEdge(String::from("!"))),
        Range(RangeEdge('0', '9')),
        Str(StrEdge(String::from("<"))),
        Str(StrEdge(String::from("+"))),
        Range(RangeEdge('A', 'Z')),
        Str(StrEdge(String::from("*"))),
        Str(StrEdge(String::from("let"))),
        Str(StrEdge(String::from("/"))),
        Str(StrEdge(String::from("=="))),
        Str(StrEdge(String::from("False"))),
        Eps(EpsEdge),
        Str(StrEdge(String::from("-"))),
        Str(StrEdge(String::from(">"))),
        Str(StrEdge(String::from(">="))),
        Range(RangeEdge('a', 'z')),
    ];
DFA::from_vecs(states, start_state, symbols)
};
let named_units = vec![
    (String::from("Action"), UnitId(10)),
    (String::from("Gt"), UnitId(23)),
    (String::from("Add"), UnitId(28)),
    (String::from("Div"), UnitId(31)),
    (String::from("Let"), UnitId(6)),
    (String::from("Arith"), UnitId(19)),
    (String::from("If"), UnitId(4)),
    (String::from("BiLogicOperator"), UnitId(12)),
    (String::from("Or"), UnitId(16)),
    (String::from("LogicOp"), UnitId(11)),
    (String::from("Mul"), UnitId(30)),
    (String::from("Num"), UnitId(34)),
    (String::from("Eps"), UnitId(0)),
    (String::from("Look"), UnitId(9)),
    (String::from(":PseudoToken:"), UnitId(38)),
    (String::from("Compare"), UnitId(14)),
    (String::from("Expr"), UnitId(3)),
    (String::from("Sub"), UnitId(29)),
    (String::from("LetExpr"), UnitId(5)),
    (String::from("EOI"), UnitId(1)),
    (String::from("VarName"), UnitId(7)),
    (String::from("Prim"), UnitId(27)),
    (String::from("UnLogicOperator"), UnitId(13)),
    (String::from("Eq"), UnitId(20)),
    (String::from("True"), UnitId(35)),
    (String::from("Call"), UnitId(8)),
    (String::from("PureBool"), UnitId(33)),
    (String::from("Not"), UnitId(17)),
    (String::from("Ge"), UnitId(25)),
    (String::from("False"), UnitId(36)),
    (String::from("IfThen"), UnitId(2)),
    (String::from("Le"), UnitId(24)),
    (String::from("CmpOperator"), UnitId(18)),
    (String::from("ArithOperator"), UnitId(26)),
    (String::from("Lt"), UnitId(22)),
    (String::from("PureNum"), UnitId(32)),
    (String::from("And"), UnitId(15)),
    (String::from("Ne"), UnitId(21)),
    (String::from("Print"), UnitId(37)),
];
let tokens = vec![
    (UnitId(0), true), 
    (UnitId(1), false), 
    (UnitId(4), false), 
    (UnitId(6), false), 
    (UnitId(7), false), 
    (UnitId(8), false), 
    (UnitId(9), false), 
    (UnitId(15), false), 
    (UnitId(16), false), 
    (UnitId(17), false), 
    (UnitId(20), false), 
    (UnitId(21), false), 
    (UnitId(22), false), 
    (UnitId(23), false), 
    (UnitId(24), false), 
    (UnitId(25), false), 
    (UnitId(28), false), 
    (UnitId(29), false), 
    (UnitId(30), false), 
    (UnitId(31), false), 
    (UnitId(34), false), 
    (UnitId(35), false), 
    (UnitId(36), false), 
    (UnitId(37), false), 
    (UnitId(38), false), 
];
let nterms = vec![
    (UnitId(2), vec![RuleId(1),RuleId(2),], String::from("Tree"), false),
    (UnitId(3), vec![RuleId(0),], String::from("Tree"), true),
    (UnitId(5), vec![RuleId(3),RuleId(4),RuleId(5),RuleId(6),RuleId(7),], String::from("Tree"), false),
    (UnitId(10), vec![RuleId(35),], String::from("Action"), false),
    (UnitId(11), vec![RuleId(8),RuleId(9),RuleId(10),], String::from("Tree"), false),
    (UnitId(12), vec![RuleId(11),RuleId(12),], String::from("BinOper"), false),
    (UnitId(13), vec![RuleId(13),], String::from("UnOper"), false),
    (UnitId(14), vec![RuleId(14),RuleId(15),], String::from("Tree"), false),
    (UnitId(18), vec![RuleId(16),RuleId(17),RuleId(18),RuleId(19),RuleId(20),RuleId(21),], String::from("BinOper"), false),
    (UnitId(19), vec![RuleId(22),RuleId(23),], String::from("Tree"), false),
    (UnitId(26), vec![RuleId(24),RuleId(25),RuleId(26),RuleId(27),], String::from("BinOper"), false),
    (UnitId(27), vec![RuleId(28),RuleId(29),RuleId(30),RuleId(31),], String::from("Tree"), false),
    (UnitId(32), vec![RuleId(32),], String::from("Primal"), false),
    (UnitId(33), vec![RuleId(33),RuleId(34),], String::from("Primal"), false),
];
let rules = vec![
    (UnitId(3), vec![UnitId(2),], vec![Some(String::from("t")),], String::from(r##"{ t }"##)),
    (UnitId(2), vec![UnitId(4),UnitId(5),UnitId(5),], vec![None,Some(String::from("cond")),Some(String::from("then")),], String::from(r##"{ Tree::IfThenElse(Box::new(cond), Box::new(then), None) }"##)),
    (UnitId(2), vec![UnitId(5),], vec![Some(String::from("l")),], String::from(r##"{ l }"##)),
    (UnitId(5), vec![UnitId(6),UnitId(7),UnitId(5),UnitId(5),], vec![None,Some(String::from("var")),Some(String::from("val")),Some(String::from("e")),], String::from(r##"{ Tree::Let(var, Box::new(val), Box::new(e)) }"##)),
    (UnitId(5), vec![UnitId(8),UnitId(5),UnitId(5),], vec![None,Some(String::from("fun")),Some(String::from("arg")),], String::from(r##"{ Tree::FunCall(Box::new(fun), Box::new(arg)) }"##)),
    (UnitId(5), vec![UnitId(9),UnitId(10),UnitId(5),], vec![None,Some(String::from("act")),Some(String::from("after")),], String::from(r##"{ Tree::Look(act, Box::new(after)) }"##)),
    (UnitId(5), vec![UnitId(4),UnitId(5),UnitId(5),UnitId(5),], vec![None,Some(String::from("cond")),Some(String::from("t")),Some(String::from("e")),], String::from(r##"{ Tree::IfThenElse(Box::new(cond), Box::new(t), Some(Box::new(e))) }"##)),
    (UnitId(5), vec![UnitId(11),], vec![Some(String::from("l")),], String::from(r##"{ l }"##)),
    (UnitId(11), vec![UnitId(12),UnitId(5),UnitId(5),], vec![Some(String::from("op")),Some(String::from("l")),Some(String::from("r")),], String::from(r##"{ Tree::BinOp(op, Box::new(l), Box::new(r)) }"##)),
    (UnitId(11), vec![UnitId(13),UnitId(5),], vec![Some(String::from("op")),Some(String::from("sub")),], String::from(r##"{ Tree::UnOp(op, Box::new(sub)) }"##)),
    (UnitId(11), vec![UnitId(14),], vec![Some(String::from("c")),], String::from(r##"{ c }"##)),
    (UnitId(12), vec![UnitId(15),], vec![None,], String::from(r##"{ BinOper::And }"##)),
    (UnitId(12), vec![UnitId(16),], vec![None,], String::from(r##"{ BinOper::Or  }"##)),
    (UnitId(13), vec![UnitId(17),], vec![None,], String::from(r##"{ UnOper::Not }"##)),
    (UnitId(14), vec![UnitId(18),UnitId(5),UnitId(5),], vec![Some(String::from("op")),Some(String::from("l")),Some(String::from("r")),], String::from(r##"{ Tree::BinOp(op, Box::new(l), Box::new(r)) }"##)),
    (UnitId(14), vec![UnitId(19),], vec![Some(String::from("a")),], String::from(r##"{ a }"##)),
    (UnitId(18), vec![UnitId(20),], vec![None,], String::from(r##"{ BinOper::Eq }"##)),
    (UnitId(18), vec![UnitId(21),], vec![None,], String::from(r##"{ BinOper::Ne }"##)),
    (UnitId(18), vec![UnitId(22),], vec![None,], String::from(r##"{ BinOper::Lt }"##)),
    (UnitId(18), vec![UnitId(23),], vec![None,], String::from(r##"{ BinOper::Gt }"##)),
    (UnitId(18), vec![UnitId(24),], vec![None,], String::from(r##"{ BinOper::Le }"##)),
    (UnitId(18), vec![UnitId(25),], vec![None,], String::from(r##"{ BinOper::Ge }"##)),
    (UnitId(19), vec![UnitId(26),UnitId(5),UnitId(5),], vec![Some(String::from("op")),Some(String::from("l")),Some(String::from("r")),], String::from(r##"{ Tree::BinOp(op, Box::new(l), Box::new(r)) }"##)),
    (UnitId(19), vec![UnitId(27),], vec![Some(String::from("p")),], String::from(r##"{ p }"##)),
    (UnitId(26), vec![UnitId(28),], vec![None,], String::from(r##"{ BinOper::Add }"##)),
    (UnitId(26), vec![UnitId(29),], vec![None,], String::from(r##"{ BinOper::Sub }"##)),
    (UnitId(26), vec![UnitId(30),], vec![None,], String::from(r##"{ BinOper::Mul }"##)),
    (UnitId(26), vec![UnitId(31),], vec![None,], String::from(r##"{ BinOper::Div }"##)),
    (UnitId(27), vec![UnitId(32),], vec![Some(String::from("n")),], String::from(r##"{ Tree::Primal(n) }"##)),
    (UnitId(27), vec![UnitId(33),], vec![Some(String::from("b")),], String::from(r##"{ Tree::Primal(b) }"##)),
    (UnitId(27), vec![UnitId(7),], vec![Some(String::from("v")),], String::from(r##"{ Tree::Primal(Primal::Variable(v)) }"##)),
    (UnitId(27), vec![UnitId(10),], vec![Some(String::from("a")),], String::from(r##"{ Tree::Action(a) }"##)),
    (UnitId(32), vec![UnitId(34),], vec![Some(String::from("n")),], String::from(r##"{ Primal::Numeric(n.parse().unwrap()) }"##)),
    (UnitId(33), vec![UnitId(35),], vec![None,], String::from(r##"{ Primal::Bool(true) }"##)),
    (UnitId(33), vec![UnitId(36),], vec![None,], String::from(r##"{ Primal::Bool(false) }"##)),
    (UnitId(10), vec![UnitId(37),UnitId(5),], vec![None,Some(String::from("sub")),], String::from(r##"{ Action::Print(Box::new(sub)) }"##)),
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

    pub fn parse<'this, 'input>(&'this self, to_parse: &'input str) -> Result<Tree, ParseError<'this>>
    { self.base.create::<_Data, Tree>(to_parse).parse() }

    pub fn dump_info(&self)
    { self.base.registry().print_lalr_items() }
}

fn _IfThen_0(
    _: String, // If
    cond: Tree, // LetExpr
    then: Tree, // LetExpr
) -> Tree
{ Tree::IfThenElse(Box::new(cond), Box::new(then), None) }

fn _IfThen_1(
    l: Tree, // LetExpr
) -> Tree
{ l }


fn _Expr_0(
    t: Tree, // IfThen
) -> Tree
{ t }


fn _LetExpr_0(
    _: String, // Let
    var: String, // VarName
    val: Tree, // LetExpr
    e: Tree, // LetExpr
) -> Tree
{ Tree::Let(var, Box::new(val), Box::new(e)) }

fn _LetExpr_1(
    _: String, // Call
    fun: Tree, // LetExpr
    arg: Tree, // LetExpr
) -> Tree
{ Tree::FunCall(Box::new(fun), Box::new(arg)) }

fn _LetExpr_2(
    _: String, // Look
    act: Action, // Action
    after: Tree, // LetExpr
) -> Tree
{ Tree::Look(act, Box::new(after)) }

fn _LetExpr_3(
    _: String, // If
    cond: Tree, // LetExpr
    t: Tree, // LetExpr
    e: Tree, // LetExpr
) -> Tree
{ Tree::IfThenElse(Box::new(cond), Box::new(t), Some(Box::new(e))) }

fn _LetExpr_4(
    l: Tree, // LogicOp
) -> Tree
{ l }


fn _Action_0(
    _: String, // Print
    sub: Tree, // LetExpr
) -> Action
{ Action::Print(Box::new(sub)) }


fn _LogicOp_0(
    op: BinOper, // BiLogicOperator
    l: Tree, // LetExpr
    r: Tree, // LetExpr
) -> Tree
{ Tree::BinOp(op, Box::new(l), Box::new(r)) }

fn _LogicOp_1(
    op: UnOper, // UnLogicOperator
    sub: Tree, // LetExpr
) -> Tree
{ Tree::UnOp(op, Box::new(sub)) }

fn _LogicOp_2(
    c: Tree, // Compare
) -> Tree
{ c }


fn _BiLogicOperator_0(
    _: String, // And
) -> BinOper
{ BinOper::And }

fn _BiLogicOperator_1(
    _: String, // Or
) -> BinOper
{ BinOper::Or  }


fn _UnLogicOperator_0(
    _: String, // Not
) -> UnOper
{ UnOper::Not }


fn _Compare_0(
    op: BinOper, // CmpOperator
    l: Tree, // LetExpr
    r: Tree, // LetExpr
) -> Tree
{ Tree::BinOp(op, Box::new(l), Box::new(r)) }

fn _Compare_1(
    a: Tree, // Arith
) -> Tree
{ a }


fn _CmpOperator_0(
    _: String, // Eq
) -> BinOper
{ BinOper::Eq }

fn _CmpOperator_1(
    _: String, // Ne
) -> BinOper
{ BinOper::Ne }

fn _CmpOperator_2(
    _: String, // Lt
) -> BinOper
{ BinOper::Lt }

fn _CmpOperator_3(
    _: String, // Gt
) -> BinOper
{ BinOper::Gt }

fn _CmpOperator_4(
    _: String, // Le
) -> BinOper
{ BinOper::Le }

fn _CmpOperator_5(
    _: String, // Ge
) -> BinOper
{ BinOper::Ge }


fn _Arith_0(
    op: BinOper, // ArithOperator
    l: Tree, // LetExpr
    r: Tree, // LetExpr
) -> Tree
{ Tree::BinOp(op, Box::new(l), Box::new(r)) }

fn _Arith_1(
    p: Tree, // Prim
) -> Tree
{ p }


fn _ArithOperator_0(
    _: String, // Add
) -> BinOper
{ BinOper::Add }

fn _ArithOperator_1(
    _: String, // Sub
) -> BinOper
{ BinOper::Sub }

fn _ArithOperator_2(
    _: String, // Mul
) -> BinOper
{ BinOper::Mul }

fn _ArithOperator_3(
    _: String, // Div
) -> BinOper
{ BinOper::Div }


fn _Prim_0(
    n: Primal, // PureNum
) -> Tree
{ Tree::Primal(n) }

fn _Prim_1(
    b: Primal, // PureBool
) -> Tree
{ Tree::Primal(b) }

fn _Prim_2(
    v: String, // VarName
) -> Tree
{ Tree::Primal(Primal::Variable(v)) }

fn _Prim_3(
    a: Action, // Action
) -> Tree
{ Tree::Action(a) }


fn _PureNum_0(
    n: String, // Num
) -> Primal
{ Primal::Numeric(n.parse().unwrap()) }


fn _PureBool_0(
    _: String, // True
) -> Primal
{ Primal::Bool(true) }

fn _PureBool_1(
    _: String, // False
) -> Primal
{ Primal::Bool(false) }

// outer world

pub enum _Data
{
    _IfThen(Tree),
    _Expr(Tree),
    _LetExpr(Tree),
    _Action(Action),
    _LogicOp(Tree),
    _BiLogicOperator(BinOper),
    _UnLogicOperator(UnOper),
    _Compare(Tree),
    _CmpOperator(BinOper),
    _Arith(Tree),
    _ArithOperator(BinOper),
    _Prim(Tree),
    _PureNum(Primal),
    _PureBool(Primal),
    _Token(String),
}

impl ActionCallback for _Data
{
    fn run_action(args: Vec<Self>, rule_id: RuleId) -> Self
    {
        let mut args = args.into_iter();
        match rule_id {
            RuleId(0) => match args.next().unwrap() {
                Self::_IfThen(_0) => Self::_Expr(_Expr_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(1) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_LetExpr(_1),Self::_LetExpr(_2)) => Self::_IfThen(_IfThen_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(2) => match args.next().unwrap() {
                Self::_LetExpr(_0) => Self::_IfThen(_IfThen_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(3) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_Token(_1),Self::_LetExpr(_2),Self::_LetExpr(_3)) => Self::_LetExpr(_LetExpr_0(_0,_1,_2,_3)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(4) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_LetExpr(_1),Self::_LetExpr(_2)) => Self::_LetExpr(_LetExpr_1(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(5) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_Action(_1),Self::_LetExpr(_2)) => Self::_LetExpr(_LetExpr_2(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(6) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_LetExpr(_1),Self::_LetExpr(_2),Self::_LetExpr(_3)) => Self::_LetExpr(_LetExpr_3(_0,_1,_2,_3)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(7) => match args.next().unwrap() {
                Self::_LogicOp(_0) => Self::_LetExpr(_LetExpr_4(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(8) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_BiLogicOperator(_0),Self::_LetExpr(_1),Self::_LetExpr(_2)) => Self::_LogicOp(_LogicOp_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(9) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_UnLogicOperator(_0),Self::_LetExpr(_1)) => Self::_LogicOp(_LogicOp_1(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(10) => match args.next().unwrap() {
                Self::_Compare(_0) => Self::_LogicOp(_LogicOp_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(11) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_BiLogicOperator(_BiLogicOperator_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(12) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_BiLogicOperator(_BiLogicOperator_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(13) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_UnLogicOperator(_UnLogicOperator_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(14) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_CmpOperator(_0),Self::_LetExpr(_1),Self::_LetExpr(_2)) => Self::_Compare(_Compare_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(15) => match args.next().unwrap() {
                Self::_Arith(_0) => Self::_Compare(_Compare_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(16) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_CmpOperator(_CmpOperator_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(17) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_CmpOperator(_CmpOperator_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(18) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_CmpOperator(_CmpOperator_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(19) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_CmpOperator(_CmpOperator_3(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(20) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_CmpOperator(_CmpOperator_4(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(21) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_CmpOperator(_CmpOperator_5(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(22) => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_ArithOperator(_0),Self::_LetExpr(_1),Self::_LetExpr(_2)) => Self::_Arith(_Arith_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(23) => match args.next().unwrap() {
                Self::_Prim(_0) => Self::_Arith(_Arith_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(24) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_ArithOperator(_ArithOperator_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(25) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_ArithOperator(_ArithOperator_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(26) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_ArithOperator(_ArithOperator_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(27) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_ArithOperator(_ArithOperator_3(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(28) => match args.next().unwrap() {
                Self::_PureNum(_0) => Self::_Prim(_Prim_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(29) => match args.next().unwrap() {
                Self::_PureBool(_0) => Self::_Prim(_Prim_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(30) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_Prim(_Prim_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(31) => match args.next().unwrap() {
                Self::_Action(_0) => Self::_Prim(_Prim_3(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(32) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_PureNum(_PureNum_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(33) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_PureBool(_PureBool_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(34) => match args.next().unwrap() {
                Self::_Token(_0) => Self::_PureBool(_PureBool_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            RuleId(35) => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_LetExpr(_1)) => Self::_Action(_Action_0(_0,_1)),
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
            Self::_Expr(res) => res,
            _ => panic!("Couldn't extract Tree"),
        }
    }

    fn sym_id() -> UnitId
    { UnitId(3) }
}

