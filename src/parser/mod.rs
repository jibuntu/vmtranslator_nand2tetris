// APIの仕様については nand2tetris - page 158

#![allow(dead_code)]
use std::str::FromStr;

/// ひとつの.vmファイルに対してパースを行うとともに、入力コードへのアクセスを
/// カプセル化する。つまり、このモジュールはVMコマンドを読み、それをパースし、
/// その要素に対してアクセスする便利なメソッドを提供する。さらに、空白文字と
/// コメントを取り除く
pub struct Parser {
    vm_lines: Vec<String>, // 不要なデータを削除した行のvector
    command: String, // 現在のコマンド
    count: usize, // 現在の行数
}

/// 現VMコマンドの種類を表す。
/// 算術コマンドはすべて`CommandType::ARITHMETIC`として表される。
/// どれにも該当しない場合は`CommandType::None`として表される。
#[derive(Debug, PartialEq)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LAVEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
    None,
}

impl Parser {
    /// 引数はVMの文字列
    pub fn new (vm: &str) -> Parser {
        let lines = vm.lines();
        let mut vm_lines = Vec::new();

        // 不要な行や空白を除外する
        for line in lines {
            let mut line = line;

            // コメントの削除
            let comment: Vec<_> = line.match_indices("//").collect();
            if comment.len() != 0 {
                line = &line[..comment[0].0];
            }

            // 両端の空白を削除
            line = line.trim_matches(' ');

            if line.len() == 0 {
                continue;
            }

            vm_lines.push(line.to_string());
        }

        Parser {
            vm_lines,
            command: String::new(),
            count: 0,
        }
    }

    // 入力において、さらにコマンドが存在するか？
    pub fn has_more_commands(&self) -> bool {
        if self.count < self.vm_lines.len() {
            return true
        }
        return false
    }

    /// 入力から次のコマンドを読み、それを現コマンドとする。
    /// `hasMoreCommands()`が`true`の場合のみ呼ぶようにする。
    /// 最初は現コマンドは空である。
    pub fn advance(&mut self) {
        self.command = self.vm_lines[self.count].clone();
        self.count+=1;
    }

    /// 現VMコマンドの種類を返す。算術コマンドはすべて
    /// `CommandType::ARITHMETIC`が返される。
    pub fn command_type(&self) -> CommandType {
        let word = self.command.split(' ').next().unwrap_or("").trim();
        
        match word {
            "add" | "sub" | "neg" | "eq" | "gt" 
            | "lt" | "and" | "or" | "not" => CommandType::ARITHMETIC,
            "push" => CommandType::PUSH,
            "pop" => CommandType::POP,
            "lavel" => CommandType::LAVEL,
            "goto" => CommandType::GOTO,
            "if" => CommandType::IF,
            "function" => CommandType::FUNCTION,
            "return" => CommandType::RETURN,
            "call" => CommandType::CALL,
            _ => CommandType::None
        }
    }

    /// 現コマンドの最初の引数が返される。`CommandType::ARITHMETIC`の場合、
    /// コマンド自体（add、subなど）が返される。現コマンドが
    /// `CommandType::RETURN`の場合、本ルーチンを呼ばないようにする
    pub fn arg1(&self) -> Option<String> {
        let words: Vec<&str> = self.command.split(' ')
                                   .filter(|w| *w != "").collect();

        if words.len() == 3 {
            return Some(words[1].to_string());
        } else if words.len() == 2 {
            return Some(words[0].to_string());
        }

        None
    }

    /// 現コマンドの２番目の引数が返される。現コマンドが`CommandType::PUSH`、
    /// `CommandType::POP`、`CommandType::FUNCTION`、`CommandType::CALL`の
    /// 場合のみ本ルーチンを呼ぶようにする
    fn arg2(&self) -> Option<isize> {
        let words: Vec<&str> = self.command.split(' ')
                                   .filter(|w| *w != "").collect();

        if words.len() == 3 {
            match isize::from_str(words[2]) {
                Ok(n) => return Some(n),
                Err(_) => return None
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use super::CommandType;

    #[test]
    fn test_parser_new() {
        Parser::new("");
    }

    #[test]
    fn test_parser_has_more_commands() {
        let parser = Parser::new("");
        assert_eq!(parser.has_more_commands(), false);
        
        let parser = Parser::new("VM");
        assert_eq!(parser.has_more_commands(), true);
    }

    #[test]
    fn test_parser_advance() {
        let mut parser = Parser::new("VM");
        assert_eq!(parser.has_more_commands(), true);
        parser.advance();
        assert_eq!(parser.has_more_commands(), false);    
 
        let mut parser = Parser::new(r#"
        // test
        push local 2
        // test
        push local 3
        "#);
        parser.advance();
        assert_eq!(&parser.command, "push local 2");
        parser.advance();
        assert_eq!(&parser.command, "push local 3");
    }

    #[test]
    fn test_parser_command_type() {
        let mut parser = Parser::new(r#"
        push local 2
        add 1
        sub 1
        "#);

        parser.advance();
        assert_eq!(parser.command_type(), CommandType::PUSH);
        parser.advance();
        assert_eq!(parser.command_type(), CommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), CommandType::ARITHMETIC);
    }

    #[test]
    fn test_parser_arg1() {
        let mut parser = Parser::new(r#"
        push local 2
        add 1
        return
        "#);
        parser.advance();
        assert_eq!(&parser.arg1().unwrap(), "local");
        parser.advance();
        assert_eq!(&parser.arg1().unwrap(), "add");
        parser.advance();
        assert_eq!(parser.arg1(), None);
    }

    #[test]
    fn test_parser_arg2() {
        let mut parser = Parser::new(r#"
        push local 2
        pop local 1
        add 1
        return
        "#);
        parser.advance();
        assert_eq!(parser.arg2().unwrap(), 2);
        parser.advance();
        assert_eq!(parser.arg2().unwrap(), 1);
        parser.advance();
        assert_eq!(parser.arg2(), None);
        parser.advance();
        assert_eq!(parser.arg2(), None);
    }
}