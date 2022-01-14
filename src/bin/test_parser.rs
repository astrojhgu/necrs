extern crate necrs;
use std::fs::read_to_string;


use necrs::nec_parser::{parse_nec_file, parse_numbers, NecParser, Rule};

use pest::Parser;
fn main() {
    let unparsed_file = read_to_string("./b.nec").unwrap();
    //let a=NecParser::parse(Rule::Gw, &unparsed_file);
    //println!("{:?}", a);
    parse_nec_file(
        NecParser::parse(Rule::NecFile, &unparsed_file)
            .expect("failed")
            .next()
            .unwrap(),
    );
}
