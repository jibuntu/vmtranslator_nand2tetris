//! 特定のVMコマンドに対応するアセンブリコードを返す関数群

#![allow(dead_code)]
#![allow(unused_macros)]

/// 特定の変数の値をincrementするためのアセンブリコードを出力するマクロ
macro_rules! inc {
    ($ver:expr) => {
        concat!(
            "@", $ver, " \n",
            "M=M+1 \n"
        )
    };
}

/// 特定の変数の値をdecrementするためのアセンブリコードを出力するマクロ
macro_rules! dec {
    ($ver:expr) => {
        concat!(
            "@", $ver, " \n",
            "M=M-1 \n"
        )
    };
}

/// 特定の変数の値をDレジスタにpopするマクロ
macro_rules! pop2d {
    ($var:expr) => {
        concat!(
            dec!($var), // $varレジスタの値をデクリメント
            "A=M \n",
            "D=M \n" // Dレジスタに数値を入れる
        )
    };
}

/// 特定の変数の値をMレジスタにpopするマクロ
macro_rules! pop2m {
    ($var:expr) => {
        concat!(
            dec!($var), // $varレジスタの値をデクリメント
            "A=M \n", 
        )
    };
}

/// ２変数関数（binary functoin）の計算を行うマクロ。
/// 計算結果はMレジスタに保存される
/// * 第一引数はスタックポインタ名
/// * 第二引数は次のうちのどれか。`+, -, &, |`
macro_rules! binfunc {
    ($var:expr, $sign:expr) => {
        concat!(
            pop2d!($var), // SPレジスタの値をDレジスタに入れる
            pop2m!($var), // SPレジスタの値をMレジスタに入れる
            "M=D", $sign, "M", " \n", // M[SP] = M[SP+1] $sign M[SP]
            inc!($var) // SPレジスタの値をインクリメントする
        )
    };
}

/// addコマンド
/// スタックから2つpopして足し算をする。その結果をスタックに入れる 
pub fn add() -> String {
    concat!(
        "// [start] add\n",
        binfunc!("SP", "+"),
        "// [end] add \n"
    ).to_string()
}

/// subコマンド
pub fn sub() -> String {
    concat!(
        "// [start] sub \n",
        binfunc!("SP", "-"),
        "// [end] sub \n"
    ).to_string()
}


/// SPが指す番地に定数(n)を代入してSPをインクリメントする
pub fn push_constant(n: isize) -> String {
    /*
    spレジスタの番地ではなく、spレジスタの値の番地にnを代入する
    */
    format!(
        concat!(
            "// [start] push constant {n} \n",
            "@{n} \n", // Aレジスタにnを入れる
            "D=A \n", // Dレジスタに移す
            "@SP \n",
            "A=M \n", // SPレジスタの値をAレジスタに入れる
            "M=D \n", // SPレジスタの値の番地にnを入れる
            inc!("SP"), // SPレジスタの値をインクリメントする
            "// [end] push constant {n} \n",
        ), n=n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let asm = concat!(
            "// [start] add\n",
            "@SP \n",
            "M=M-1 \n", // SPレジスタの値をデクリメント
            "A=M \n", 
            "D=M \n", // Dレジスタに数値を入れる
            "@SP \n",
            "M=M-1 \n", // 再びSPレジスタの値をデクリメント
            "A=M \n",
            "M=D+M \n", // M[SP] = M[SP+1] + M[SP]
            "@SP \n",
            "M=M+1 \n", // SPレジスタの値をインクリメントする
            "// [end] add \n"
        ).to_string();
        assert_eq!(add(), asm);
    }
    
    #[test]
    fn test_push_constant() {
        let n = 5;
        let asm = format!(concat!(
            "// [start] push constant {n} \n",
            "@{n} \n", // Aレジスタにnを入れる
            "D=A \n", // Dレジスタに移す
            "@SP \n",
            "A=M \n", // SPレジスタの値をAレジスタに入れる
            "M=D \n", // SPレジスタの値の番地にnを入れる
            "@SP \n",
            "M=M+1 \n", // SPレジスタの値をインクリメントする
            "// [end] push constant {n} \n",
        ), n=n).to_string();
        assert_eq!(push_constant(n), asm)
    }
}
