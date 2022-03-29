use std::io::{self, BufRead};

use grammar::SParser;

mod grammar;

fn main() {
    let parser = SParser::new();
    io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| parser.parse(&line))
        .for_each(|parse_res| match parse_res {
            Ok((l, r)) => println!("{l}, {r}"),
            Err(err) => println!("Error: {err}"),
        })
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn bad_strings()
    {
        let parser = SParser::new();
        [ "a", "ab", "aba", "b", "", "aaa", "aaba", "aababb", "ccbcb" ].into_iter()
        .map(|line| (line, parser.parse(line)))
        .for_each(|(line, res)| match res {
            Ok(res) => panic!("Unexpected parse for '{line}': {res:?}"),
            Err(_) => ()
        });
    }

    #[test]
    fn good_strings()
    {
        let parser = SParser::new();
        [ "bb", "aabb", "baaab", "aaabaab", "abab" ].into_iter()
        .zip([(0, 0), (2, 0), (0, 3), (3, 2), (1, 1)])
        .map(|(line, expected)| (line, parser.parse(line), expected))
        .for_each(|(line, res, expected)| match res {
            Ok(res) => assert_eq!(res, expected, "Unexpected value for '{line}': {res:?}"),
            Err(err) => panic!("Parse error for '{line}': {err}"),
        })
    }
}
