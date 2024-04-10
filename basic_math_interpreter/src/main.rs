// use "Parser.rs";

mod Parser;
use Parser::parse_perfect_input;
// use Parser::parse_perfect_input;

fn main() {

    let input_string = String::from("10+20*(10+3)");
    let ret_val = parse_perfect_input(&input_string);
    println!("{}", ret_val[0]);
    println!("Hello world motherfuckers\n");

}


