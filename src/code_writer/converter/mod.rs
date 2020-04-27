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
            "M=M", $sign, "D", " \n", // M[SP] = M[SP] $sign M[SP+1]
            inc!($var) // SPレジスタの値をインクリメントする
        )
    };
}


/// Dレジスタの値に対して条件がtrueなら-1、falseなら0がスタックに入る。
/// 内部でformatマクロを使っているため、concatマクロの中では使えない。
/// * 第一引数はスタックポインタ名
/// * 第二引数は条件
/// * 第三引数のrowsは行数
macro_rules! ifd {
    ($var:expr, $jump:expr, $label:expr) => {
        format!(concat!(
            "@{t_label}-true \n",   // 条件がtrueのときのジャンプ先を指定
            "D;", $jump, " \n",     // 条件がtrueならt_labelにジャンプする
            "@0 \n",                // 条件がfalseの場合
            "D=A \n",               // Dレジスタに0を入れる
            "@{f_label}-false \n",  // f_labelをジャンプ先として指定
            "0;JMP \n",             // 無条件でf_labelへジャンプする
            "({t_label}-true) \n",   // t_labelのジャンプ先
            "D=-1 \n",              // Dレジスタに-1を入れる
            "({f_label}-false) \n", // f_labelのジャンプ先
            "@", $var, " \n",
            "A=M \n",
            "M=D \n",               // $var変数にDレジスタの値を入れる
            inc!($var)              // インクリメントする
        ), t_label=$label, f_label=$label)
    };
}


/// addコマンド。
/// スタックから2つpopして足し算をする。その結果をスタックに入れる 
pub fn add() -> String {
    binfunc!("SP", "+").to_string()
}

/// subコマンド
pub fn sub() -> String {
    binfunc!("SP", "-").to_string()
}

/// negコマンド
pub fn neg() -> String {
    concat!(
        pop2m!("SP"),
        "M=-M \n",
        inc!("SP"),
    ).to_string()
}

/// eqコマンドを変換する関数。引数は現在のアセンブリコードの行数。
/// trueなら0、falseなら-1がスタックに入る
pub fn eq(label: &str) -> String {
    /*
    引き算をした結果のMが0かどうか
    */
    let mut asm = String::new();
    asm += binfunc!("SP", "-"); // 引き算をする
    asm += pop2d!("SP"); // 引き算の結果をDレジスタに入れる
    asm += &ifd!("SP", "JEQ", label);
    
    asm
}

/// gtコマンドを変換する関数。引数は現在のアセンブリコードの行数。
/// trueなら0、falseなら-1がスタックに入る
pub fn gt(label: &str) -> String {
    let mut asm = String::new();
    /*
    引き算をした結果が0より大きければtrue
    */
    asm += binfunc!("SP", "-"); // 引き算をする
    asm += pop2d!("SP"); // 引き算の結果をDレジスタに入れる
    asm += &ifd!("SP", "JGT", label);

    asm
}

/// ltコマンドを変換する関数。引数は現在のアセンブリコードの行数。
/// trueなら0、falseなら-1がスタックに入る
pub fn lt(label: &str) -> String {
    let mut asm = String::new();
    /*
    引き算をした結果が0より小さければtrue
    */
    asm += binfunc!("SP", "-"); // 引き算をする
    asm += pop2d!("SP"); // 引き算の結果をDレジスタに入れる
    asm += &ifd!("SP", "JLT", label);

    asm
}

/// andコマンド
pub fn and() -> String {
    binfunc!("SP", "&").to_string()
}

/// orコマンド
pub fn or() -> String {
    binfunc!("SP", "|").to_string()
}

/// notコマンド
pub fn not() -> String {
    concat!(
        pop2m!("SP"),
        "M=!M \n",
        inc!("SP"),
    ).to_string()
}


// Dレジスタの値をスタックへ入れる
macro_rules! d2stack {
    () => {
        concat!(
            "@SP \n",
            "A=M \n",   // M[SP]の値をAレジスタへ
            "M=D \n",   // Dレジスタの値をM[M[SP]]へ
            inc!("SP"), // スタックをインクリメントする
        )
    };
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
}



/// if-gotoコマンド。スタック最上位をpopし、その値が0以外なら
/// ラベルで指定された場所へジャンプする
pub fn if_goto(label: &str) -> String {
    format!(concat!(
        pop2d!("SP"), // スタックをpopしてDレジスタへ
        "@{} \n", // ジャンプ先をAレジスタへ
        "D;JNE \n", // Dレジスタが0以外ならジャンプ     
    ), label)
}

/// callコマンド
pub fn call(funcname: &str, argc: usize, return_address: &str) -> String {
    /*
    callコマンドが呼ばれる前に引数はスタックにpushされている
    return addressをスタックにpushする
    呼び出し側のLCLからTHATまでの値をスタックにpushしておく
    ARGとLCLは呼び出される関数が使うので値を設定しておく
    関数に移るジャンプ命令を書く
    ラベルを宣言する
    */
    let mut asm = String::new();

    // return addressをpushする
    asm += &format!(concat!(
        "@{} \n",   // return addressをAレジスタへ
        "D=A \n",   // return addressをDレジスタへ
        d2stack!(), // Dレジスタの値(return address)をスタックへ
    ), return_address);

    // 呼び出し側のセグメントのアドレスをスタックへpush
    for segment in &["LCL", "ARG", "THIS", "THAT"] {
        asm += &format!(concat!(
            "@{} \n",
            "D=M \n",   // M[segment]の値をDレジスタへ
            d2stack!(), // Dレジスタの値をstackへpush
        ), segment);
    }

    // ARGセグメントの値を設定する
    // 現在のSPの値 - argc - 5
    // M[ARG] = M[SP]-argc-5
    asm += &format!(concat!(
        "@SP \n",
        "D=M \n",   // M[SP]の値をDレジスタへ
        "@{} \n",   // argcをAレジスタへ
        "D=D-A \n", // M[SP]-argc
        "@5 \n",    // 5をAレジスタへ
        "D=D-A \n", // M[SP]-argc-5
        "@ARG \n",
        "M=D \n",   // M[ARG] = M[SP]-argc-5
    ), argc);

    // LCLセグメントの値を設定する
    // LCLセグメントは先頭なのでSPレジスタの値を入れればいい
    asm += concat!(
        "@SP \n",
        "D=M \n", // SPレジスタの値をDレジスタへ
        "@LCL \n",
        "M=D \n", // M[LCL]=M[SP]
    );

    // 関数を呼び出す前の準備が済んだのでジャンプする
    // 関数のアドレスをAレジスタに入れてジャンプする
    asm += &format!(concat!(
        "@{} \n",
        "0;JMP \n"
    ), funcname);

    // return addressのラベルを宣言する
    asm += &format!("({}) \n", return_address);

    asm
}

/// functionコマンド。
pub fn function(funcname: &str, argc: usize) -> String {
    let mut asm = String::new();
    
    // 関数のラベルを設定
    asm += &format!("({}) \n", funcname);

    /*
    argc個のローカル変数を0で初期化する
    ローカル変数はスタックに積まれているので、0をスタックにpushすればいい
    また、関数の開始時にはローカル変数の次のアドレスをSPが指している必要がある
    */
    for _i in 0..argc {
        asm += concat!(
                "@0 \n", // Aレジスタに0を入れる
                "D=A \n", // Dレジスタに移す
                "@SP \n",
                "A=M \n", // SPレジスタの値をAレジスタに入れる
                "M=D \n", // SPレジスタの値の番地に0を入れる
                inc!("SP"), // SPレジスタの値をインクリメントする
            );
    }

    asm
}

/// returnコマンド
pub fn ret() -> String {
    /*
    return addressにジャンプする前に返り値(スタックの先頭の値)を
    ARG[0]の場所に置いておく
    LCLの前に呼び出し側のセグメント情報があるので、
    LCLの値の番地-iでアクセスする
    各セグメントを呼び出し側の値に戻す
    */
    let mut asm = String::new();

    // LCLが持つ番地の値をR14レジスタへ
    asm += "@LCL \n";
    asm += "D=M \n"; // M[LCL]の値をDレジスタへ
    asm += "@R14 \n";
    asm += "M=D \n"; // M[LCL]の値をR14レジスタへ

    // return addressの値をR15レジスタへ
    // return addressはM[M[LCL]-5]の値
    asm += "@R14 \n";
    asm += "D=M \n";   // D=M[LCL]
    asm += "@5 \n";
    asm += "A=D-A \n"; // A=M[LCL]-5
    asm += "D=M \n";   // D=M[M[LCL]-5]
    asm += "@R15 \n";
    asm += "M=D \n";   // M[R15]=M[M[LCL]-5]

    
    // ARG[0]の番地にスタックの先頭を値を入れる
    asm += pop2d!("SP"); // スタックの先頭の値をDレジスタへ
    asm += "@ARG \n";
    asm += "A=M \n"; // M[ARG]
    asm += "M=D \n"; // M[M[ARG]]へDを入れる
    
    // SPの値を呼び出し側の値に戻す
    // ARG[0]の番地の戻り値が入っているので、ARG[0]の番地+1をSPの値にする
    // SPの値をM[ARG[0]]+1の値にする
    asm += "@ARG \n"; // ARGのレジスタには最初の引数のアドレスが入っている
    asm += "D=M \n"; // M[ARG[0]]の値をDレジスタへ
    asm += "D=D+1 \n"; // Dレジスタの値をインクリメントする D=M[ARG[0]]+1
    asm += "@SP \n";
    asm += "M=D \n"; // SPレジスタの値にM[ARG[0]]+1を入れる

    // THATからLCLのセグメントの値を呼び出し側の値に戻す
    // R14の値をデクリメントしながら各セグメントへ値を入れる
    for segment in &["THAT", "THIS", "ARG", "LCL"] {
        // segmentの値をM[M[R14]-1]にする
        asm += "@R14 \n";
        asm += "M=M-1 \n"; // R14をデクリメント
        asm += "A=M \n";   // M[R14]-1をAレジスタへ
        asm += "D=M \n";   // M[M[R14]-1]をDレジスタへ
        asm += "@";
        asm += segment;
        asm += " \n";
        asm += "M=D \n";   // M[M[R14]-1]をsegmentレジスタへ
    }

    // return addressへジャンプする
    // return addressはR15の番地に入っている
    asm += "@R15 \n";
    asm += "A=M \n"; // A=M[R15]
    asm += "0;JMP \n"; // return addressへジャンプ

    asm
}


/// SPが指す番地に定数(n)を代入してSPをインクリメントする
pub fn push_constant(n: isize) -> String {
    /*
    spレジスタの番地ではなく、spレジスタの値の番地にnを代入する
    */
    format!(
        concat!(
            "@{n} \n", // Aレジスタにnを入れる
            "D=A \n", // Dレジスタに移す
            "@SP \n",
            "A=M \n", // SPレジスタの値をAレジスタに入れる
            "M=D \n", // SPレジスタの値の番地にnを入れる
            inc!("SP"), // SPレジスタの値をインクリメントする
        ), n=n)
}

/// segment[index]の値をスタック上にpushする
pub fn push_local(index: isize) -> String {
    push2stack!("LCL", index)
}

/// SPの番地の値をlocalが指す番地+indexの番地に書き込む
pub fn pop_local(index: isize) -> String {
    pop2s!("LCL", index)
}

/// segment[index]の値をスタック上にpushする
pub fn push_argument(index: isize) -> String {
    push2stack!("ARG", index)
}

/// SPの番地の値をargumentが指す番地+indexの番地に書き込む
pub fn pop_argument(index: isize) -> String {
    pop2s!("ARG", index)
}

/// segment[index]の値をスタック上にpushする
pub fn push_this(index: isize) -> String {
    push2stack!("THIS", index)
}

/// SPの番地の値をthisが指す番地+indexの番地に書き込む
pub fn pop_this(index: isize) -> String {
    pop2s!("THIS", index)
}

/// segment[index]の値をスタック上にpushする
pub fn push_that(index: isize) -> String {
    push2stack!("THAT", index)
}

/// SPの番地の値をthatが指す番地+indexの番地に書き込む
pub fn pop_that(index: isize) -> String {
    pop2s!("THAT", index)
}

/// segment[index]の値をスタック上にpushする
pub fn push_temp(index: isize) -> String {
    push2stack_2!("R5", index)
}

/// SPの番地の値をtempが指す番地+indexの番地に書き込む
pub fn pop_temp(index: isize) -> String {
    pop2s_2!("R5", index)
}

/// segment[index]の値をスタック上にpushする。
/// pointerはthisとthatの間にマッピングされる。
pub fn push_pointer(index: isize) -> String {
    push2stack_2!("THIS", index)
}

/// SPの番地の値をtempが指す番地+indexの番地に書き込む
/// pointerはthisとthatの間にマッピングされる。
pub fn pop_pointer(index: isize) -> String {
    pop2s_2!("THIS", index)
}

pub fn push_static(index: isize, filename: &str) -> String {
    format!(concat!(
        "@{}.{n} \n",
        "D=M \n",
        "@SP \n",
        "A=M \n", // SPレジスタの値をAレジスタに入れる
        "M=D \n", // SPレジスタの値の番地にnを入れる
        inc!("SP"), // SPレジスタの値をインクリメントする
    ), filename, n=index)
}

pub fn pop_static(index: isize, filename: &str) -> String {
    format!(concat!(
        pop2d!("SP"),
        "@{}.{n} \n",
        "M=D \n",
    ), filename, n=index)
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
        assert_eq!(add(), asm);
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
        assert_eq!(push_constant(n), asm)
    }
}
