// VMコマンドをHackアセンブリコードへ変換する

use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{Read, Write};

mod parser;
use parser::Parser;
mod code_writer;
use code_writer::CodeWriter;

fn print_usage() {
    println!("VMコマンドをHackアセンブリコードへ変換する");
    println!("Usage: command <filename> <output filename>");
}

fn print_error(e: &str) {
    println!("Error: {}", e);
    print_usage();
}

fn main() {
    let mut args = env::args().skip(1);
    let filename = match args.next() {
        Some(f) => f,
        None => return print_error("ファイル名がありません")
    };
    let output_filename = match args.next() {
        Some(f) => f,
        None => return print_error("出力先のファイル名がありません")
    };
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(_) => return print_error(&format!("'{}' is not exist.", filename))
    };

//    let mut vm = String::new();
//    let _ = file.read_to_string(&mut vm);
//
//    let mut parser = Parser::new(&vm);
//    let mut 
//
//    while parser.has_more_commands() {
//        parser.advance();
//
//    }


}
