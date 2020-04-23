// APIの仕様については nand2tetris - page 160

#![allow(dead_code)]
use std::io::Write;

mod converter;

/// VMコマンドをHackのアセンブリコードに変換する。
/// `CommandType::{LAVEL, GOTO, IF, FUNCTION, RETURN, CALL}`に対応する部分は
/// まだ実装しなくてよい
pub struct CodeWriter<W> {
    filename: String,
    rows: usize,
    asm: W
}

impl <W: Write> CodeWriter<W> {
    /// 引数はstd::io::Writeトレイトを実装している構造体
    pub fn new(stream: W) -> CodeWriter<W> {
        CodeWriter {
            filename: String::new(),
            rows: 0,
            asm: stream,
        }
    }

    /// CodeWriterモジュールに新しいVMファイルの変換が開始したことを知らせる
    pub fn set_file_name(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    /// VMの初期化（これは「ブートストラップ」と呼ばれる）
    /// を行うアセンブリコードを書く。このコードは出力ファイルの先頭に
    /// 配置しなければならない
    pub fn write_init(&mut self) {
        let asm = format!(concat!(
            "@256 \n", // SP(スタックポインタ)を256に設定する
            "D=A \n",
            "@SP \n",
            "M=D \n",

        ));

        let _ = self.asm.write(asm.as_bytes());
        self.rows += 4;
    }

    /// labelコマンドを行うアセンブリコードを書く
    pub fn write_label(&mut self, label: &str) -> Result<(), String> {
        /*
        labelが定義された関数内のみで有効ということは関数名を先頭に
        つける必要があるかもしれない
        */
        let _ = self.asm.write(format!("({}) \n", label).as_bytes());
        Ok(())
    }

    /// gotoコマンドを行うアセンブリコードを書く
    pub fn write_goto(&mut self, label: &str) -> Result<(), String> {
        let asm = format!(concat!(
            "@{} \n",
            "0;JMP \n"
        ), label);

        let _ = self.asm.write(asm.as_bytes());
        self.rows += 2;

        Ok(())
    }

    /// 与えられた算術コマンドをアセンブリコードに変換し、それを書き込む
    pub fn write_arithmetic(&mut self, command: &str) -> Result<(), String> {
        let (asm, rows) = match command {
            "add" => converter::add(),
            "sub" => converter::sub(),
            "neg" => converter::neg(),
            "eq" => converter::eq(self.rows),
            "gt" => converter::gt(self.rows),
            "lt" => converter::lt(self.rows),
            "and" => converter::and(),
            "or" => converter::or(),
            "not" => converter::not(),
            _ => return Err(format!("{} は無効なコマンドです", command))
        };

        let asm_code = format!(concat!(
            "// [start: {}] {c} \n",
            "{}",
            "// [end: {}] {c} \n"
        ), self.rows, asm, self.rows+rows-1, c=command);
        let _ = self.asm.write(asm_code.as_bytes());
        self.rows += rows;

        Ok(())
    }

    /// `CommandType::PUSH`または`CommandType::POP`コマンドをアセンブリコードに
    /// 変換し、それを書き込む
    pub fn write_push_pop(&mut self, command: &str, segment: &str, 
                          index: isize) -> Result<(), String> {
        let (asm, rows) = match command {
            "push" => match segment {
                "constant" => converter::push_constant(index),
                "local" => converter::push_local(index),
                "argument" => converter::push_argument(index),
                "this" => converter::push_this(index),
                "that" => converter::push_that(index),
                "temp" => converter::push_temp(index),
                "pointer" => converter::push_pointer(index),
                "static" => converter::push_static(index, &self.filename),
                _ => return Err(format!("push {} は無効なセグメントです", 
                                        segment))
            },
            "pop" => match segment {
                "local" => converter::pop_local(index),
                "argument" => converter::pop_argument(index),
                "this" => converter::pop_this(index),
                "that" => converter::pop_that(index),
                "temp" => converter::pop_temp(index),
                "pointer" => converter::pop_pointer(index),
                "static" => converter::pop_static(index, &self.filename),
                _ => return Err(format!("pop {} は無効なセグメントです", 
                                        segment))
            },
            _ => return Err(format!("{} は無効なコマンドです", command)),
        };

        let asm_code = format!(concat!(
            "// [start: {}] {c} {s} {i} \n",
            "{}",
            "// [end: {}] {c} {s} {i} \n"
        ), self.rows, asm, self.rows+rows-1, c=command, s=segment, i=index);
        let _ = self.asm.write(asm_code.as_bytes());
        self.rows += rows;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use std::io::Read;

    use super::CodeWriter;
    use std::io::Cursor;

    #[test]
    fn test_code_writer() {
        let asm = Cursor::new(Vec::new());
        let mut cw = CodeWriter::new(asm);
        cw.write_push_pop("push", "constant", 1).unwrap();
        cw.write_push_pop("push", "constant", 2).unwrap();
        cw.write_arithmetic("add").unwrap();
        assert_eq!(cw.rows, 24);
    }

    #[test]
    fn test_code_writer_write_arithmetic() {
        let cursor = Cursor::new(Vec::new());
        let mut cw = CodeWriter::new(cursor);
        cw.write_push_pop("push", "constant", 1).unwrap();
        cw.write_push_pop("push", "constant", 2).unwrap();

        let mut asm = format!(concat!(
            "// [start: 0] push constant {n} \n",
            "@{n} \n",
            "D=A \n",
            "@SP \n",
            "A=M \n",
            "M=D \n",
            "@SP \n",
            "M=M+1 \n", 
            "// [end: 6] push constant {n} \n"
        ), n=1);

        asm += &format!(concat!(
            "// [start: 7] push constant {n} \n",
            "@{n} \n",
            "D=A \n",
            "@SP \n",
            "A=M \n",
            "M=D \n",
            "@SP \n",
            "M=M+1 \n", 
            "// [end: 13] push constant {n} \n"
        ), n=2);

        assert_eq!(cw.rows, 14);
        assert_eq!(cw.asm.get_ref(), &asm.into_bytes());
        println!("{}", String::from_utf8(cw.asm.get_ref().to_vec()).unwrap());

        cw.write_arithmetic("eq").unwrap();
        println!("{}", String::from_utf8(cw.asm.get_ref().to_vec()).unwrap());
    }
}