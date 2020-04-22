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
    () => { 2 };
}

/// 特定の変数の値をdecrementするためのアセンブリコードを出力するマクロ
macro_rules! dec {
    ($ver:expr) => {
        concat!(
            "@", $ver, " \n",
            "M=M-1 \n"
        )
    };
    () => { 2 };
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
    () => { dec!() + 2 };
}

/// 特定の変数の値をMレジスタにpopするマクロ
macro_rules! pop2m {
    ($var:expr) => {
        concat!(
            dec!($var), // $varレジスタの値をデクリメント
            "A=M \n", 
        )
    };
    () => { dec!() + 1 };
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
            "M=M", $sign, "D", " \n", // M[SP] = M[SP] $sign M[SP+1]
            inc!($var) // SPレジスタの値をインクリメントする
        )
    };
    () => { pop2d!() + pop2m!() + 1 + inc!() };
}

/// Dレジスタの値が0かどうか判定するマクロ（is d zero?）。
/// Dレジスタの値が0なら-1がスタックに入る。
/// Dレジスタの値が0以外なら0がスタックに入る。
/// 内部でformatマクロを使っているため、concatマクロの中では使えない。
/// * 第一引数はスタックポインタ名
/// * 第二引数のrowsは行数
macro_rules! isdz {
    ($var:expr, $rows:expr) => {
        format!(concat!(
/* 01 */    "@{adress_true} \n", // Dが0のときのジャンプ先を指定
/* 02 */    "D;JEQ \n", // Dが0ならadress_trueにジャンプする
/* 03 */    "@0 \n", // Dが0でない場合
/* 04 */    "D=A \n", // Dに0を入れる
/* 05 */    "@{adress_end} \n", // trueのときに飛ぶコードが終わった所を指定
/* 06 */    "0;JMP \n", // adress_endへジャンプする
/* 07 */    "D=-1 \n", // Dに-1を入れる // D==0ならここに飛ぶ
/* 08 */    "@", $var, " \n", // address_endで飛んでくる場所
/* 09 */    "A=M \n",
/* 10 */    "M=D \n", // $var変数にDの値を入れる
            inc!($var) // インクリメントする
        ), adress_true=($rows + 6), adress_end=($rows + 7))
    };
    () => { 10 + inc!() };
}

/// addコマンド。
/// スタックから2つpopして足し算をする。その結果をスタックに入れる 
pub fn add() -> (String, usize) {
    (binfunc!("SP", "+").to_string(), binfunc!())
}

/// subコマンド
pub fn sub() -> (String, usize) {
    (binfunc!("SP", "-").to_string(), binfunc!())
}

/// negコマンド
pub fn neg() -> (String, usize) {
    (concat!(
        pop2m!("SP"),
        "M=-M \n",
        inc!("SP"),
    ).to_string(), pop2m!() + 1 + inc!())
}

/// eqコマンドを変換する関数。引数は現在のアセンブリコードの行数。
/// trueなら0、falseなら-1がスタックに入る
pub fn eq(rows: usize) -> (String, usize) {
    /*
    引き算をした結果のMが0かどうか
    */
    let mut asm = String::new();
    asm += binfunc!("SP", "-"); // 引き算をする
    asm += pop2d!("SP"); // 引き算の結果をDレジスタに入れる
    asm += &isdz!("SP", rows + binfunc!() + pop2d!()); // 現在の行数を渡す
    
    (asm, binfunc!() + pop2d!() + isdz!())
}


/// SPが指す番地に定数(n)を代入してSPをインクリメントする
pub fn push_constant(n: isize) -> (String, usize) {
    /*
    spレジスタの番地ではなく、spレジスタの値の番地にnを代入する
    */
    (format!(
        concat!(
            "@{n} \n", // Aレジスタにnを入れる
            "D=A \n", // Dレジスタに移す
            "@SP \n",
            "A=M \n", // SPレジスタの値をAレジスタに入れる
            "M=D \n", // SPレジスタの値の番地にnを入れる
            inc!("SP"), // SPレジスタの値をインクリメントする
        ), n=n), 5 + inc!())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let asm = concat!(
            "@SP \n",
            "M=M-1 \n", // SPレジスタの値をデクリメント
            "A=M \n", 
            "D=M \n", // Dレジスタに数値を入れる
            "@SP \n",
            "M=M-1 \n", // 再びSPレジスタの値をデクリメント
            "A=M \n",
            "M=M+D \n", // M[SP] = M[SP] + M[SP+1]
            "@SP \n",
            "M=M+1 \n", // SPレジスタの値をインクリメントする
        ).to_string();
        assert_eq!(add(), (asm, 10));
    }
    
    #[test]
    fn test_push_constant() {
        let n = 5;
        let asm = format!(concat!(
            "@{n} \n", // Aレジスタにnを入れる
            "D=A \n", // Dレジスタに移す
            "@SP \n",
            "A=M \n", // SPレジスタの値をAレジスタに入れる
            "M=D \n", // SPレジスタの値の番地にnを入れる
            "@SP \n",
            "M=M+1 \n", // SPレジスタの値をインクリメントする
        ), n=n).to_string();
        assert_eq!(push_constant(n), (asm, 7))
    }
}
