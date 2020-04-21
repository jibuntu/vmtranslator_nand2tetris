// APIの仕様については nand2tetris - page 160

#![allow(dead_code)]

mod converter;

/// VMコマンドをHackのアセンブリコードに変換する
/// CommandType::{LAVEL, GOTO, IF, FUNCTION, RETURN, CALL}に対応する部分は
/// まだ実装しなくてよい
pub struct CodeWriter {
    filename: String,
    asm: String,
//    file: File,
}

impl CodeWriter {
    /// 引数はアセンブリコードの保存先のファイル名
    pub fn new() -> CodeWriter {
        CodeWriter {
            filename: String::new(),
            asm: String::new()
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
            _ => return Err("無効なコマンドです".to_string())
        };

        self.asm += &asm;

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
                    _ => return Err("無効なセグメントです".to_string())
                }
            },
            "pop" => return Err("未対応".to_string()),
            _ => return Err("無効なコマンドです".to_string()),
        };

        self.asm += &asm;

        Ok(())
    }
}