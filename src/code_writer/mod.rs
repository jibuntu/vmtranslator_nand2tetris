// APIの仕様については nand2tetris - page 160

#![allow(dead_code)]
use std::io::Write;

mod converter;

/// VMコマンドをHackのアセンブリコードに変換する
/// CommandType::{LAVEL, GOTO, IF, FUNCTION, RETURN, CALL}に対応する部分は
/// まだ実装しなくてよい
pub struct CodeWriter<W> {
    filename: String,
    asm: W
}

impl <W: Write> CodeWriter<W> {
    /// 引数はstd::io::Writeトレイトを実装している構造体
    pub fn new(stream: W) -> CodeWriter<W> {
        CodeWriter {
            filename: String::new(),
            asm: stream,
        }
    }

    /// CodeWriterモジュールに新しいVMファイルの変換が開始したことを知らせる
    pub fn set_file_name(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    /// 与えられた算術コマンドをアセンブリコードに変換し、それを書き込む
    pub fn write_arithmetic(&mut self, command: &str) -> Result<(), String> {
        let asm = match command {
            "add" => converter::add(),
            "sub" => converter::sub(),
            _ => return Err(format!("{} は無効なコマンドです", command))
        };

        let _ = self.asm.write(asm.as_bytes());

        Ok(())
    }

    /// `CommandType::PUSH`または`CommandType::POP`コマンドをアセンブリコードに
    /// 変換し、それを書き込む
    pub fn write_push_pop(&mut self, command: &str, segment: &str, 
                          index: isize) -> Result<(), String> {
        let asm = match command {
            "push" => {
                match segment {
                    "constant" => {
                        converter::push_constant(index)
                    },
                    _ => return Err(format!("{} は無効なセグメントです", 
                                            segment))
                }
            },
            "pop" => return Err("POPコマンドは未対応".to_string()),
            _ => return Err(format!("{} は無効なコマンドです", command)),
        };

        let _ = self.asm.write(asm.as_bytes());

        Ok(())
    }
}