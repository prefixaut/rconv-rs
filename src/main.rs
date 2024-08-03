use std::fs;

mod common;
mod stepmania;

use crate::stepmania::StepmaniaParser;

fn main() {
    let file_name = "test-files/sample1.sm";
    let unparsed_file = fs::read_to_string(file_name).expect("cannot read file");
    let mut parser = StepmaniaParser::new();

    let step = parser.parse_from_string(&unparsed_file);
    
    println!("{:?}, err: {:?}", step, parser.errors);
}
