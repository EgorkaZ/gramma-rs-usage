use std::env;

use prefix_expr::{ExprParser};

fn main() {
    let input = env::args().skip(1).next();
    if let Some(input) = input {
        let parser = ExprParser::new();
        match parser.parse(&input) {
            Ok(res) => println!("{}", prefix_expr::format(&res)),
            Err(err) => println!("Error: {:?}", err),
        }
    } else {
        println!("Expected expression")
    }
}