#![feature(hash_set_entry)]
#![allow(dead_code)]

mod ast;
use ast::{BinOper, Primal, Action};
pub use ast::{Tree};
mod grammar;
mod printer;
pub use printer::format;

pub use grammar::ExprParser;

type Parser = ExprParser;

// arith
fn add(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Add, Box::new(l), Box::new(r)) }

fn sub(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Sub, Box::new(l), Box::new(r)) }

fn mul(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Mul, Box::new(l), Box::new(r)) }

fn div(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Div, Box::new(l), Box::new(r)) }

// cmp
fn eq(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Eq, Box::new(l), Box::new(r)) }

fn ne(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Ne, Box::new(l), Box::new(r)) }

fn le(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Le, Box::new(l), Box::new(r)) }

fn ge(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Ge, Box::new(l), Box::new(r)) }

fn lt(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Lt, Box::new(l), Box::new(r)) }

fn gt(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Gt, Box::new(l), Box::new(r)) }

// logic
fn and(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::And, Box::new(l), Box::new(r)) }

fn or(l: Tree, r: Tree) -> Tree
{ Tree::BinOp(BinOper::Or, Box::new(l), Box::new(r)) }

// misc
fn if_then_else(c: Tree, t: Tree, e: Tree) -> Tree
{ Tree::IfThenElse(Box::new(c), Box::new(t), Some(Box::new(e))) }

fn if_then(c: Tree, t: Tree) -> Tree
{ Tree::IfThenElse(Box::new(c), Box::new(t), None) }

fn let_expr(name: &str, init: Tree, then: Tree) -> Tree
{ Tree::Let(name.into(), Box::new(init), Box::new(then)) }

fn look(act: Action, then: Tree) -> Tree
{ Tree::Look(act, Box::new(then)) }

fn action(act: Action) -> Tree
{ Tree::Action(act) }

fn print(expr: Tree) -> Action
{ Action::Print(Box::new(expr)) }

// primal
fn num(n: i32) -> Tree
{ Tree::Primal(Primal::Numeric(n)) }

fn boolean(b: bool) -> Tree
{ Tree::Primal(Primal::Bool(b)) }

fn var(name: &str) -> Tree
{ Tree::Primal(Primal::Variable(name.into())) }

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::{grammar::ExprParser, ast::{Tree}};
    use super::*;

    fn parse<'a>(input: &str) -> Tree
    {
        let parser = ExprParser::new();
        match parser.parse(input) {
            Ok(res) => {
                println!("parsed: {:?}", res);
                res
            },
            Err(err) => panic!("error: {}", err),
        }
    }

    #[test]
    fn just_num()
    {
        parse("32");
        parse(" 42");
        parse("14 ");
    }

    #[fixture]
    fn parser() -> ExprParser
    { ExprParser::new() }

    mod add_sub_tests
    {
        use super::*;

        #[fixture]
        fn input(#[default(0)] num: usize) -> &'static str
        {
            [ "+ 2 2",
              "+ 2 * 3 4",
              "- * 2 7 + 20 68",
              "* + + 4 10 - 1000 7 0",
              "/ 2 / 3 4",
              "/ 2 * 3 4",
              "* 42 14",
              "* 1 1",
              "* * * 2 7 4 22",

              "== 3 9",
              "<= * 4 + 3 9 - 11 / 9 3",
              "& != + 2 12 14 >= 18 1",

              "if False 1",
              "if True + 1 2 - 4 3",
              "if >= + - 1 2 3 4 5",
              "if if | == 1 + 2 3 < 4 5 >= 6 7 > 8 9  if == 10 11 12 13  14",
              // (  (                  )            )(                 )(  )
              "if True False",
              "if True if False False True",

              "let a 3 let b 4 + a b",
              "let a if < 3 4 + 1 2 - 2 1 let b False if b a + 1 a",

              "let a 4 print + a 1",
              "let a 4 look print a + a 1",
            ][num]
        }

        #[fixture]
        fn expectation<'a>(
            #[default(0)] test_num: usize,
        ) -> Tree
        {
            match test_num {
                0 => add(num(2), num(2)),
                1 => add(num(2), mul(num(3), num(4))),
                2 => sub(mul(num(2), num(7)), add(num(20), num(68))),
                3 => mul(add(add(num(4), num(10)), sub(num(1000), num(7))), num(0)),
                4 => div(num(2), div(num(3), num(4))),
                5 => div(num(2), mul(num(3), num(4))),
                6 => mul(num(42), num(14)),
                7 => mul(num(1), num(1)),
                8 => mul(mul(mul(num(2), num(7)), num(4)), num(22)),

                9  => eq(num(3), num(9)),
                10 => le(
                    mul(num(4), add(num(3), num(9))),
                    sub(num(11), div(num(9), num(3)))),
                11 => and(
                    ne(add(num(2), num(12)), num(14)),
                    ge(num(18), num(1))
                ),

                12 => if_then(
                    boolean(false),
                    num(1)
                ),
                13 => if_then_else(
                    boolean(true),
                    add(num(1), num(2)),
                    sub(num(4), num(3))
                ),
                14 => if_then(
                    ge(add(sub(num(1), num(2)), num(3)), num(4)),
                    num(5)
                ),
                15 => if_then_else(
                    if_then_else(
                        or(eq(num(1),add(num(2), num(3))), lt(num(4), num(5))),
                        ge(num(6), num(7)),
                        gt(num(8), num(9))
                    ),
                    if_then_else(
                        eq(num(10), num(11)),
                        num(12),
                        num(13)
                    ),
                    num(14)
                ),

                16 => if_then(
                    boolean(true),
                    boolean(false)
                    ),
                17 => if_then(
                    boolean(true),
                    if_then_else(
                        boolean(false),
                        boolean(false),
                        boolean(true)
                    )
                ),
                18 => let_expr(
                    "a",
                    num(3),
                    let_expr("b", num(4), add(var("a"),var("b")))
                ),
                19 => let_expr(
                    "a",
                    if_then_else(
                        lt(num(3), num(4)),
                        add(num(1), num(2)),
                        sub(num(2), num(1))
                    ),
                    let_expr(
                        "b",
                        boolean(false),
                        if_then_else(
                            var("b"),
                            var("a"),
                            add(num(1), var("a"))
                        )
                    )
                ),
                20 => let_expr(
                    "a",
                    num(4),
                    action(
                        print(add(var("a"), num(1)))
                    )
                ),
                21 => let_expr(
                    "a",
                    num(4),
                    look(
                        print(var("a")),
                        add(var("a"), num(1))
                    )
                ),
                _ => panic!("unknown test case: {}", test_num),
            }
        }

        #[rstest]
        fn test(
            #[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21)]
            num: usize,
            #[with(num)] input: &str,
            parser: ExprParser)
        {
            let parsed = parser.parse(input).expect(&format!("Couldn't parse: '{}'", input));
            let expected = expectation(num);
            assert_eq!(parsed, expected);
        }
    }

}
