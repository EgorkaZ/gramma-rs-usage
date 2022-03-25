use grammar::ExprParser;

mod grammar;

fn main() {
    let parser = ExprParser::new();

    let input = "2 * (-1 + 7)";
    let parsed = parser.parse(input)
        .unwrap_or_else(|err| panic!(r#"Couldn't parse "{input}": {err}"#));
    println!("parsed: {parsed}");
}
