use std::io::{self, BufRead};

use crate::calc::ExprParser;

mod calc;

fn main() {
    let parser = ExprParser::new();

    io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| parser.parse(&line))
        .for_each(|parse_res| match parse_res {
            Ok(res) => println!("{res}"),
            Err(err) => println!("Error: {err}"),
        })
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn bad_exprs()
    {
        let parser = ExprParser::new();
        [
            "1 +", "* 2", "(1", ") 5 - 1",
            "42 )", "( 2 + 4))", "7.",
        ].into_iter()
        .map(|line| (line, parser.parse(line)))
        .for_each(|(line, res)| match res {
            Ok(res) => panic!("Unexpected parse for '{line}': {res}"),
            Err(_) => ()
        });
    }

    #[test]
    fn good_exprs()
    {
        let parser = ExprParser::new();
        [
            "1.0 + 2.0", "3.8 + 38.2", "12 / 4 / 3",
            "-1 - (2 - 3)", "((1)) + (((2)))"
        ].into_iter()
        .zip([
            1.0 + 2.0, 3.8 + 38.2, 12. / 4. / 3.,
            -1. - (2. - 3.), 1. + 2.
        ])
        .map(|(line, expected)| (line, parser.parse(line), expected))
        .for_each(|(line, res, expected)| match res {
            Ok(res) => assert_eq!(res, expected),
            Err(err) => panic!("Parse error for '{line}': {err}"),
        });
    }
}
