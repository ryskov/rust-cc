extern crate cc;

use cc::lexer;
use cc::parser;
use cc::generator;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];
    let input_file = Path::new(file_name);

    let mut file = File::open(&input_file)
        .expect(&format!("Could not open file {}", file_name));

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect(&format!("Could not read file {}", file_name));
    
    println!("Compiling file:\r\n\r\n{}\r\n", contents);

    let tokens = lexer::lex(contents);

    println!("Lexing complete: {:#?}", tokens);

    let ast = parser::parse(tokens);

    println!("Parsing complete: {:#?}", ast);

    let assembly = generator::generate(ast);

    let base_path = &file_name.to_string()[0..file_name.len() - 2];
    let out_file = format!("{}.s", base_path);

    let out_path = Path::new(&out_file);
    println!("Writing output to {}", out_path.display());

    let mut file = match File::create(&out_path) {
        Err(why) => panic!("Could not create {}: {}", out_path.display(), why),
        Ok(file) => file
    };

    match file.write_all(assembly.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", out_path.display(), why),
        Ok(_) => println!("Success!")
    };

    Command::new("gcc")
        .arg(&out_file)
        .arg("-o")
        .arg(base_path)
        .output()
        .expect("Failed to execute gcc");
}