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


/// Dレジスタの値に対して条件がtrueなら-1、falseなら0がスタックに入る。
/// 内部でformatマクロを使っているため、concatマクロの中では使えない。
/// * 第一引数はスタックポインタ名
/// * 第二引数は条件
/// * 第三引数のrowsは行数
macro_rules! ifd {
    ($var:expr, $jump:expr, $rows:expr) => {
        format!(concat!(
/* 01 */    "@{t_address} \n",  // 条件がtrueのときのジャンプ先を指定
/* 02 */    "D;", $jump, " \n", // 条件がtrueならt_adressにジャンプする
/* 03 */    "@0 \n",            // 条件がfalseの場合
/* 04 */    "D=A \n",           // Dレジスタに0を入れる
/* 05 */    "@{e_address} \n",  // e_addressをジャンプ先として指定
/* 06 */    "0;JMP \n",         // 無条件でe_addressへジャンプする
/* 07 */    "D=-1 \n",          // t_adressのジャンプ先、Dレジスタに-1を入れる
/* 08 */    "@", $var, " \n",   // e_addressのジャンプ先
/* 09 */    "A=M \n",
/* 10 */    "M=D \n",           // $var変数にDレジスタの値を入れる
            inc!($var)          // インクリメントする
        ), t_address=($rows + 6), e_address=($rows + 7))
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
    asm += &ifd!("SP", "JEQ", rows + binfunc!() + pop2d!());
    
    (asm, binfunc!() + pop2d!() + ifd!())
}

/// gtコマンドを変換する関数。引数は現在のアセンブリコードの行数。
/// trueなら0、falseなら-1がスタックに入る
pub fn gt(rows: usize) -> (String, usize) {
    let mut asm = String::new();
    /*
    引き算をした結果が0より大きければtrue
    */
    asm += binfunc!("SP", "-"); // 引き算をする
    asm += pop2d!("SP"); // 引き算の結果をDレジスタに入れる
    asm += &ifd!("SP", "JGT", rows + binfunc!() + pop2d!());

    (asm, binfunc!() + pop2d!() + ifd!())
}

/// ltコマンドを変換する関数。引数は現在のアセンブリコードの行数。
/// trueなら0、falseなら-1がスタックに入る
pub fn lt(rows: usize) -> (String, usize) {
    let mut asm = String::new();
    /*
    引き算をした結果が0より小さければtrue
    */
    asm += binfunc!("SP", "-"); // 引き算をする
    asm += pop2d!("SP"); // 引き算の結果をDレジスタに入れる
    asm += &ifd!("SP", "JLT", rows + binfunc!() + pop2d!());

    (asm, binfunc!() + pop2d!() + ifd!())
}

/// andコマンド
pub fn and() -> (String, usize) {
    (binfunc!("SP", "&").to_string(), binfunc!())
}

/// orコマンド
pub fn or() -> (String, usize) {
    (binfunc!("SP", "|").to_string(), binfunc!())
}

/// notコマンド
pub fn not() -> (String, usize) {
    (concat!(
        pop2m!("SP"),
        "M=!M \n",
        inc!("SP"),
    ).to_string(), pop2m!() + 1 + inc!())
}


/// スタックの一番上のデータをpopし、それをsegment[index]に格納する。
/// * 第一引数はセグメントのレジスタ名
/// * 第二引数はindex
macro_rules! pop2s {
    ($segment:expr, $index:ident) => {
        format!(concat!(
            /* 
            セグメントの値の番地とindexを足した結果をR13レジスタに保存する
            SPをポップしてDレジスタに入れる
            R13レジスタの番地をAに入れ、MレジスタでR13レジスタの中身を受け取る
            Mレジスタの値をAレジスタに入れる
            MレジスタにDレジスタの値を入れる
            */
            "@", $segment," \n", 
            "D=M \n", // segmentレジスタの値をDレジスタへ
            "@{} \n", // indexの値をAレジスタへ
            "D=D+A \n", // D+Aを計算して出てきた番地をDレジスタに入れる
            "@R13 \n",
            "M=D \n", // R13にDレジスタに入っている計算結果を保存する
            
            pop2d!("SP"), // SPをポップしてDレジスタに入れる
            "@R13 \n",
            "A=M \n", // R13レジスタの値(segment+indexの計算結果)をAレジスタへ
            "M=D \n", // スタックからpopしたレジスタをsegment+indexの計算結果の
                      // 番地に保存する
        ), $index)
    };
    () => { 6 + pop2d!() + 3 }
}


/// pointer, temp向けのpop2s。
/// pointer, tempはベースアドレス保持しているのではないので、それぞれの番地に
/// 直接indexを足した結果の番地を操作する
macro_rules! pop2s_2 {
    ($segment:expr, $index:ident) => {
        format!(concat!(
            "@", $segment," \n", 
            "D=A \n", // segmentレジスタの番地をDレジスタへ
            "@{} \n", // indexの値をAレジスタへ
            "D=D+A \n", // D+Aを計算して出てきた番地をDレジスタに入れる
            "@R13 \n",
            "M=D \n", // R13にDレジスタに入っている計算結果を保存する
            
            pop2d!("SP"), // SPをポップしてDレジスタに入れる
            "@R13 \n",
            "A=M \n", // R13レジスタの値(segment+indexの計算結果)をAレジスタへ
            "M=D \n", // スタックからpopしたレジスタをsegment+indexの計算結果の
                      // 番地に保存する
        ), $index)
    };
    () => { 6 + pop2d!() + 3 }
}

/// segment[index]をスタックの上にプッシュする
/// * 第一引数はセグメントのレジスタ名
/// * 第二引数はindex
macro_rules! push2stack {
    ($segment:expr, $index:ident) => {
        format!(concat!(
            "@", $segment, " \n",
            "D=M \n", // segmentレジスタの値をDレジスタへ
            "@{} \n", // indexの値をAレジスタへ
            "A=D+A \n", // segmentレジスタの値とindexの値を足してAレジスタへ
            "D=M \n", // segment[index]の値をDレジスタへ
            "@SP \n",
            "A=M \n", // M[SP]の値をAアドレスへ
            "M=D \n", // スタックの先頭にsegment[index]の値を入れる
            inc!("SP"), // SPレジスタの値をインクリメントする
        ), $index)
    };
    () => { 7 + inc!() }
}

/// pointer, temp向けのpop2s。
/// pointer, tempはベースアドレス保持しているのではないので、それぞれの番地に
/// 直接indexを足した結果の番地を操作する
macro_rules! push2stack_2 {
    ($segment:expr, $index:ident) => {
        format!(concat!(
            "@", $segment, " \n",
            "D=A \n", // segmentレジスタの番地をDレジスタへ
            "@{} \n", // indexの値をAレジスタへ
            "A=D+A \n", // segmentレジスタの値とindexの値を足してAレジスタへ
            "D=M \n", // segment[index]の値をDレジスタへ
            "@SP \n",
            "A=M \n", // M[SP]の値をAアドレスへ
            "M=D \n", // スタックの先頭にsegment[index]の値を入れる
            inc!("SP"), // SPレジスタの値をインクリメントする
        ), $index)
    };
    () => { 7 + inc!() }
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

/// segment[index]の値をスタック上にpushする
pub fn push_local(index: isize) -> (String, usize) {
    (push2stack!("LCL", index), push2stack!())
}

/// SPの番地の値をlocalが指す番地+indexの番地に書き込む
pub fn pop_local(index: isize) -> (String, usize) {
    (pop2s!("LCL", index), pop2s!())
}

/// segment[index]の値をスタック上にpushする
pub fn push_argument(index: isize) -> (String, usize) {
    (push2stack!("ARG", index), push2stack!())
}

/// SPの番地の値をargumentが指す番地+indexの番地に書き込む
pub fn pop_argument(index: isize) -> (String, usize) {
    (pop2s!("ARG", index), pop2s!())
}

/// segment[index]の値をスタック上にpushする
pub fn push_this(index: isize) -> (String, usize) {
    (push2stack!("THIS", index), push2stack!())
}

/// SPの番地の値をthisが指す番地+indexの番地に書き込む
pub fn pop_this(index: isize) -> (String, usize) {
    (pop2s!("THIS", index), pop2s!())
}

/// segment[index]の値をスタック上にpushする
pub fn push_that(index: isize) -> (String, usize) {
    (push2stack!("THAT", index), push2stack!())
}

/// SPの番地の値をthatが指す番地+indexの番地に書き込む
pub fn pop_that(index: isize) -> (String, usize) {
    (pop2s!("THAT", index), pop2s!())
}

/// segment[index]の値をスタック上にpushする
pub fn push_temp(index: isize) -> (String, usize) {
    (push2stack_2!("R5", index), push2stack_2!())
}

/// SPの番地の値をtempが指す番地+indexの番地に書き込む
pub fn pop_temp(index: isize) -> (String, usize) {
    (pop2s_2!("R5", index), pop2s_2!())
}

/// segment[index]の値をスタック上にpushする。
/// pointerはthisとthatの間にマッピングされる。
pub fn push_pointer(index: isize) -> (String, usize) {
    (push2stack_2!("THIS", index), push2stack_2!())
}

/// SPの番地の値をtempが指す番地+indexの番地に書き込む
/// pointerはthisとthatの間にマッピングされる。
pub fn pop_pointer(index: isize) -> (String, usize) {
    (pop2s_2!("THIS", index), pop2s_2!())
}

pub fn push_static(index: isize, filename: &str) -> (String, usize) {
    (format!(concat!(
        "@{}.{n} \n",
        "D=M \n",
        "@SP \n",
        "A=M \n", // SPレジスタの値をAレジスタに入れる
        "M=D \n", // SPレジスタの値の番地にnを入れる
        inc!("SP"), // SPレジスタの値をインクリメントする
    ), filename, n=index), 5 + inc!())
}

pub fn pop_static(index: isize, filename: &str) -> (String, usize) {
    (format!(concat!(
        pop2d!("SP"),
        "@{}.{n} \n",
        "M=D \n",
    ), filename, n=index), pop2d!() + 2)
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
