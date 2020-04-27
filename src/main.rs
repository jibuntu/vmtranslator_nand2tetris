// VMコマンドをHackアセンブリコードへ変換する

use std::env;
use std::fs::File;
use std::io::{Read, Write};

mod parser;
use parser::Parser;
use parser::CommandType;
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

/// vmコードを変換してCodeWriterへ保存する。エラーはすべて関数の外に投げ捨てる
fn vm_to_asm<R, W>(p: &mut Parser<R>, cw: &mut CodeWriter<W>) 
    -> Result<(), String> where R: Read, 
                                W: Write {
    while p.has_more_commands() {
        p.advance();
        match p.command_type() {
            CommandType::PUSH => { 
                cw.write_push_pop("push", &p.arg1().unwrap(), 
                                  p.arg2().unwrap())?;
            },
            CommandType::POP => {
                cw.write_push_pop("pop", &p.arg1().unwrap(), 
                                  p.arg2().unwrap())?;
            },
            CommandType::ARITHMETIC => {
                cw.write_arithmetic(p.arg1().unwrap().as_str())?;
            },
            CommandType::LABEL => cw.write_label(&p.arg1().unwrap())?,
            CommandType::GOTO => cw.write_goto(&p.arg1().unwrap())?,
            CommandType::IF => cw.write_if_goto(&p.arg1().unwrap())?,
            CommandType::FUNCTION => {
                cw.write_function(&p.arg1().unwrap(),
                                  p.arg2().unwrap() as usize)?
            },
            CommandType::RETURN => cw.write_return()?,
            CommandType::None => return Err(format!("{} は無効なコマンドです",
                                                    p.arg1().unwrap())),
            _ => return Err(format!("{:?} は未実装のコマンドです", 
                                    p.command_type()))
        }
    }
    Ok(())
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
    let file = match File::open(&filename) {
        Ok(f) => f,
        Err(_) => return print_error(&format!("'{}' is not exist.", filename))
    };
    let outputfile = match File::create(&output_filename) {
        Ok(f) => f,
        Err(_) => return print_error(&format!("can't create '{}'.", 
                                              output_filename))
    };

    let mut parser = Parser::new(file);
    let mut code_writer = CodeWriter::new(outputfile);
    code_writer.write_init();
    code_writer.set_file_name(filename.split('/').rev().nth(1).unwrap());
    if let Err(e) = vm_to_asm(&mut parser, &mut code_writer) {
        print_error(&e);
    }
}
