// APIの仕様については nand2tetris - page 160

//! R13~R15までのアドレスの使用方法
//! * R13 pop2d!マクロ内で使われる
//! * R14 returnコマンドのLCLの値を一時保存するために使われる
//! * R15 returnコマンドのreturn addressを一時保存するために使われる

#![allow(dead_code)]
use std::io::Write;

mod converter;
mod symbol_manager;
use symbol_manager::SymbolManager;

/// VMコマンドをHackのアセンブリコードに変換する。
/// `CommandType::{LAVEL, GOTO, IF, FUNCTION, RETURN, CALL}`に対応する部分は
/// まだ実装しなくてよい
pub struct CodeWriter<W> {
    filename: String,
    sm: SymbolManager,
    asm: W
}

impl <W: Write> CodeWriter<W> {
    /// 引数はstd::io::Writeトレイトを実装している構造体
    pub fn new(stream: W) -> CodeWriter<W> {
        CodeWriter {
            filename: String::new(),
            sm: SymbolManager::new(),
            asm: stream,
        }
    }

    /// CodeWriterモジュールに新しいVMファイルの変換が開始したことを知らせる
    pub fn set_file_name(&mut self, filename: &str) {
        self.filename = filename.to_string();
        let asm = format!("// [file] {} \n", filename);
        let _ = self.asm.write(asm.as_bytes());
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
    }

    /// labelコマンドを行うアセンブリコードを書く
    pub fn write_label(&mut self, label: &str) -> Result<(), String> {
        // labelが被らないようにSymbolManagerを使う
        let label = self.sm.get_goto_symbol(label);
        let _ = self.asm.write(format!("({}) \n", label).as_bytes());
        Ok(())
    }

    /// gotoコマンドを行うアセンブリコードを書く
    pub fn write_goto(&mut self, label: &str) -> Result<(), String> {
        // 元のラベルをSymbolManagerを使って変換する
        let label = self.sm.get_goto_symbol(label);
        let asm = format!(concat!(
            "@{} \n",
            "0;JMP \n"
        ), label);

        let asm_code = format!(concat!(
            "// [start] goto {l}\n",
            "{}",
            "// [end] goto {l}\n"
        ), asm, l=label);

        let _ = self.asm.write(asm_code.as_bytes());

        Ok(())
    }

    /// if-gotoコマンドを行うアセンブリコードを書く
    pub fn write_if_goto(&mut self, label: &str) -> Result<(), String> {
        // 元のラベルをSymbolManagerを使って変換する
        let label = self.sm.get_goto_symbol(label);
        let asm = converter::if_goto(&label);

        let asm_code = format!(concat!(
            "// [start] if-goto {l}\n",
            "{}",
            "// [end] if-goto {l}\n"
        ), asm, l=label);

        let _ = self.asm.write(asm_code.as_bytes());

        Ok(())
    }

   /// callコマンドを行うアセンブリコードを書く
   pub fn write_call(&mut self, function: &str, argc: usize) -> Result<(), String> {
       // 関数名を取得
       let funcname = self.sm.get_function_symbol(function);
       // return addressのラベルを取得
       let return_address = self.sm.get_return_address_symbol(function);
       let asm = converter::call(&funcname, argc, &return_address);

       let asm_code = format!(concat!(
           "// [start] call {f} {n}\n",
           "{}",
           "// [end] call {f} {n}\n"
       ), asm, f=function, n=argc);

       let _ = self.asm.write(asm_code.as_bytes());

       Ok(())
   }

    /// functinoコマンドを行うアセンブリコードを書く
    pub fn write_function(&mut self, function: &str, number: usize) 
        -> Result<(), String> 
    {
        let funcname = self.sm.get_function_symbol(function);
        let asm = converter::function(&funcname, number);

        let asm_code = format!(concat!(
            "// [start] function {f} {n}\n",
            "{}",
            "// [end] function {f} {n}\n"
        ), asm, f=function, n=number);
        let _ = self.asm.write(asm_code.as_bytes());

        self.sm.set_function_name(function);

        Ok(())
    }

    /// returnコマンドを行うアセンブリコードを書く
    pub fn write_return(&mut self) -> Result<(), String> {
        let asm = converter::ret();

        let asm_code = format!(concat!(
            "// [start] return\n",
            "{}",
            "// [end] return\n"
        ), asm);

        let _ = self.asm.write(asm_code.as_bytes());

        Ok(())
    }

    /// 与えられた算術コマンドをアセンブリコードに変換し、それを書き込む
    pub fn write_arithmetic(&mut self, command: &str) -> Result<(), String> {
        let asm = match command {
            "add" => converter::add(),
            "sub" => converter::sub(),
            "neg" => converter::neg(),
            "eq" => converter::eq(&self.sm.get_ifd_symbol()),
            "gt" => converter::gt(&self.sm.get_ifd_symbol()),
            "lt" => converter::lt(&self.sm.get_ifd_symbol()),
            "and" => converter::and(),
            "or" => converter::or(),
            "not" => converter::not(),
            _ => return Err(format!("{} は無効なコマンドです", command))
        };

        let asm_code = format!(concat!(
            "// [start] {c} \n",
            "{}",
            "// [end] {c} \n"
        ), asm, c=command);
        let _ = self.asm.write(asm_code.as_bytes());

        Ok(())
    }

    /// `CommandType::PUSH`または`CommandType::POP`コマンドをアセンブリコードに
    /// 変換し、それを書き込む
    pub fn write_push_pop(&mut self, command: &str, segment: &str, 
                          index: isize) -> Result<(), String> {
        let asm = match command {
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
            "// [start] {c} {s} {i} \n",
            "{}",
            "// [end] {c} {s} {i} \n"
        ), asm, c=command, s=segment, i=index);
        let _ = self.asm.write(asm_code.as_bytes());
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
        let _cw = CodeWriter::new(asm);
    }

    #[test]
    fn test_code_writer_write_arithmetic() {
        let cursor = Cursor::new(Vec::new());
        let mut cw = CodeWriter::new(cursor);
        cw.write_push_pop("push", "constant", 1).unwrap();
        cw.write_push_pop("push", "constant", 2).unwrap();

        let mut asm = format!(concat!(
            "// [start] push constant {n} \n",
            "@{n} \n",
            "D=A \n",
            "@SP \n",
            "A=M \n",
            "M=D \n",
            "@SP \n",
            "M=M+1 \n", 
            "// [end] push constant {n} \n"
        ), n=1);

        asm += &format!(concat!(
            "// [start] push constant {n} \n",
            "@{n} \n",
            "D=A \n",
            "@SP \n",
            "A=M \n",
            "M=D \n",
            "@SP \n",
            "M=M+1 \n", 
            "// [end] push constant {n} \n"
        ), n=2);

        assert_eq!(cw.asm.get_ref(), &asm.into_bytes());
        println!("{}", String::from_utf8(cw.asm.get_ref().to_vec()).unwrap());

        cw.write_arithmetic("eq").unwrap();
        println!("{}", String::from_utf8(cw.asm.get_ref().to_vec()).unwrap());
    }
}