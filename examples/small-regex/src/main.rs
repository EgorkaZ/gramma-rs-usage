use small_regex::grammar::RegexParser;
use small_regex::graph_printer::print_tree;

fn main()
{
    let parser = RegexParser::new();
    let string = "(hel(l*)*o)|(w(or(l)))d";
    let parsed = parser.parse(string);
    match parsed {
        Ok(tree) => print_tree(&tree),
        Err(parse_err) => println!("parse error: {parse_err}"),
    }
}