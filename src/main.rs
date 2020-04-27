// VMコマンドをHackアセンブリコードへ変換する

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

mod parser;
use parser::Parser;
use parser::CommandType;
mod code_writer;
use code_writer::CodeWriter;

fn print_usage() {
    println!("VMコマンドをHackアセンブリコードへ変換する");
    println!();
    println!("Usage:");
    println!("   command vm_path asm_path");
    println!();
    println!("Arguments:");
    println!("    vm_path     vmファイル、もしくはvmファイルのあるディレクトリのパス。");
    println!("                vm_pathがディレクトリのパスの場合はディレクトリ内にあるすべての");
    println!("                vmファイルを１つのasmファイルに変換する");
    println!("    asm_path    コンパイルされたasmファイルを書き込むパス");

}

fn print_error(e: &str) {
    println!("Error: {}", e);
    println!();
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
            CommandType::CALL => {
                cw.write_call(&p.arg1().unwrap(),
                              p.arg2().unwrap() as usize)?
            },
            CommandType::None => return Err(format!("{} は無効なコマンドです",
                                                    p.arg1().unwrap())),
        }
    }
    Ok(())
}

/// pathからvmファイルのリストを取得する
fn get_f_list(vm_path: &str) -> Result<Vec<String>, String> {
    // ファイル名とFile構造体のリスト
    let mut f_list: Vec<String> = Vec::new();
    let metadata = match fs::metadata(&vm_path) {
        Ok(m) => m,
        Err(_) => return Err(format!("'{}' is not exist.", vm_path))
    };

    if metadata.is_file() {
        f_list.push(vm_path.to_string());
        return Ok(f_list)
    }

    for entry in fs::read_dir(vm_path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            let filename = path.to_str().unwrap();
            if &filename[filename.len()-3..filename.len()] == ".vm" {
                f_list.push(filename.to_string());
            }
        }
    }

    if f_list.len() == 0 {
        return Err(format!("There isn't vm files in '{}'.", vm_path));
    }

    Ok(f_list)
}

fn main() {
    let mut args = env::args().skip(1);
    let vm_path = match args.next() {
        Some(f) => f,
        None => return print_error("vm_pathがありません")
    };
    let asm_path = match args.next() {
        Some(f) => f,
        None => return print_error("asm_pathがありません")
    };

    let f_list = match get_f_list(&vm_path) {
        Ok(f_list) => f_list,
        Err(e) => return print_error(&e)
    };
    let outputfile = match File::create(&asm_path) {
        Ok(f) => f,
        Err(_) => return print_error(&format!("can't create '{}'.", 
                                              asm_path))
    };

    let mut code_writer = CodeWriter::new(outputfile);
    code_writer.write_init();
    let _ = code_writer.write_call("Sys.init", 0);
    
    for filename in f_list {
        let file = match File::open(&filename) {
            Ok(f) => f,
            Err(_) => return print_error(&format!("{}を開けません", &filename))
        };
        
        let mut parser = Parser::new(file);
        code_writer.set_file_name(filename.split('/').rev().nth(1).unwrap());
        if let Err(e) = vm_to_asm(&mut parser, &mut code_writer) {
            return print_error(&e);
        }
    }
}
