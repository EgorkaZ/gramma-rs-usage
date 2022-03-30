pub mod lexer;
pub mod tree;
pub mod parser;
pub mod graph_printer;
pub mod grammar;

extern crate graphviz_rust;

#[cfg(test)]
mod tests {
    use std::iter;

    use crate::{parser::parse, grammar::RegexParser};

    use super::lexer::*;

    #[test]
    fn tokenize()
    {
        use Token::*;

        let string = "  ( a  b\t\n) | d *";
        let expected = [ LParen, Char('a'), Char('b'), RParen, Pipe, Char('d'), Star ];

        string.chars()
            .tokenize()
            .zip(expected.map(Ok))
            .for_each(|(lhs, rhs)| assert_eq!(lhs, rhs));
    }

    #[test]
    fn tokenize_bad()
    {
        use Token::*;
        use TokenizeError::*;

        let string = "  (  d\t e) $ | *";
        //                             ^ bad symbol here
        let expected = [ LParen, Char('d'), Char('e'), RParen].into_iter()
            .map(Ok)
            .chain(iter::once(Err(Unexpected('$'))));

        string.chars()
            .tokenize()
            .zip(expected)
            .for_each(|(lhs, rhs)| assert_eq!(lhs, rhs));
    }

    fn check_ok(s: &str)
    {
        let parser = RegexParser::new();
        match parser.parse(s) {
            Ok(_) => (),
            Err(err) => panic!("Couldn't parse '{s}': {err}"),
        }
    }

    fn check_fail(s: &str)
    {
        let parser = RegexParser::new();
        match parser.parse(s) {
            Ok(res) => panic!("Unexpectedly parsed '{s}': {res:?}"),
            Err(_) => (),
        }
    }

    #[test]
    fn parse_good()
    {
        // check_ok("");
        check_ok("a*");
        check_ok("(a*)*");
        check_ok("((a*)c)*");
        check_ok("hell0");
        check_ok("(hell*o)|(w(or(l)))d");
        check_ok("ab|c*(de)");
        check_ok("ab|(((c))*)*(de)");
        check_ok("((a)b)|fl|(ad)");
    }

    #[test]
    fn parse_bad()
    {
        check_fail("?");
        check_fail("(a");
        check_fail("()");
        check_fail("|f");
        check_fail("(*)a");
        check_fail("*a");
        check_fail(")");

        check_fail("a**");
        check_fail("ab|c**(de)");
        check_fail("(hell**o)|(w(or(l)))d");
    }
}