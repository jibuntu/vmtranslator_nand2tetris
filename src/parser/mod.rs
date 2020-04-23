// APIの仕様については nand2tetris - page 158

#![allow(dead_code)]
use std::str::FromStr;
use std::io::Read;

mod vmlines;
use vmlines::Vmlines;


/// ひとつの.vmファイルに対してパースを行うとともに、入力コードへのアクセスを
/// カプセル化する。つまり、このモジュールはVMコマンドを読み、それをパースし、
/// その要素に対してアクセスする便利なメソッドを提供する。さらに、空白文字と
/// コメントを取り除く
pub struct Parser<R> {
    vm_lines: Vmlines<R>,
    command: Option<String>, // 現在のコマンド
    next: Option<String>, // 次のコマンド
}

/// 現VMコマンドの種類を表す。
/// 算術コマンドはすべて`CommandType::ARITHMETIC`として表される。
/// どれにも該当しない場合は`CommandType::None`として表される。
#[derive(Debug, PartialEq)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
    None,
}

impl<R: Read> Parser<R> {
    /// 引数は`std::io::Read`トレイトを実装している構造体
    pub fn new(stream: R) -> Parser<R> {
        let mut vm_lines = Vmlines::new(stream);
        let next = vm_lines.next();

        Parser {
            vm_lines,
            command: None,
            next,
        }
    }

    // 入力において、さらにコマンドが存在するか？
    pub fn has_more_commands(&self) -> bool {
        match self.next {
            Some(_) => true,
            None => false
        }
    }

    /// 入力から次のコマンドを読み、それを現コマンドとする。
    /// `has_more_commands()`が`true`の場合のみ呼ぶようにする。
    /// 最初は現コマンドは空である。
    pub fn advance(&mut self) {
        self.command = self.next.take();
        self.next = self.vm_lines.next();
    }

    /// 現VMコマンドの種類を返す。算術コマンドはすべて
    /// `CommandType::ARITHMETIC`が返される。
    pub fn command_type(&self) -> CommandType {
        let command = match &self.command {
            Some(c) => c,
            None => return CommandType::None
        };
        let word = command.split(' ').next().unwrap_or("").trim();
        
        match word {
            "add" | "sub" | "neg" | "eq" | "gt" 
            | "lt" | "and" | "or" | "not" => CommandType::ARITHMETIC,
            "push" => CommandType::PUSH,
            "pop" => CommandType::POP,
            "label" => CommandType::LABEL,
            "goto" => CommandType::GOTO,
            "if-goto" => CommandType::IF,
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
        let command = match &self.command {
            Some(c) => c,
            None => return None
        };

        let words: Vec<&str> = command.split(' ')
                                   .filter(|w| *w != "").collect();

        if words.len() == 3 {
            return Some(words[1].to_string());
        } else if words.len() == 2 {
            return Some(words[1].to_string());
        } else if words.len() == 1 {
            return Some(words[0].to_string());
        }

        None
    }

    /// 現コマンドの２番目の引数が返される。現コマンドが`CommandType::PUSH`、
    /// `CommandType::POP`、`CommandType::FUNCTION`、`CommandType::CALL`の
    /// 場合のみ本ルーチンを呼ぶようにする
    pub fn arg2(&self) -> Option<isize> {
        let command = match &self.command {
            Some(c) => c,
            None => return None
        };

        let words: Vec<&str> = command.split(' ')
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
        Parser::new("".as_bytes());
    }

    #[test]
    fn test_parser_has_more_commands() {
        let parser = Parser::new("".as_bytes());
        assert_eq!(parser.has_more_commands(), false);
        
        let parser = Parser::new("VM".as_bytes());
        assert_eq!(parser.has_more_commands(), true);
    }

    #[test]
    fn test_parser_advance() {
        let mut parser = Parser::new("VM".as_bytes());
        assert_eq!(parser.has_more_commands(), true);
        parser.advance();
        assert_eq!(parser.has_more_commands(), false);    
 
        let mut parser = Parser::new(r#"
        // test
        push local 2
        // test
        push local 3
        "#.as_bytes());
        parser.advance();
        assert_eq!(parser.command, Some("push local 2".to_string()));
        parser.advance();
        assert_eq!(parser.command, Some("push local 3".to_string()));
        parser.advance();
        assert_eq!(parser.command, None);
    }
    
    #[test]
    fn test_parser_command_type() {
        let mut parser = Parser::new(r#"
        push local 2
        add 1
        sub 1
        "#.as_bytes());

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
        add
        sub
        "#.as_bytes());
        parser.advance();
        assert_eq!(&parser.arg1().unwrap(), "local");
        parser.advance();
        assert_eq!(parser.arg1(), Some("add".to_string()));
        parser.advance();
        assert_eq!(parser.arg1(), Some("sub".to_string()));
        parser.advance();
        assert_eq!(parser.arg1(), None);
    }

    #[test]
    fn test_parser_arg2() {
        let mut parser = Parser::new(r#"
        push local 2
        pop local 1
        add
        return
        "#.as_bytes());
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