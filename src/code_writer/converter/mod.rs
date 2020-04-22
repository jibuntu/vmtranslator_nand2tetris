//! 特定のVMコマンドに対応するアセンブリコードを返す関数群

#![allow(dead_code)]

/// addコマンド
/// スタックから2つpopして足し算をする。その結果をスタックに入れる 
pub fn add() -> String {
    concat!(
        "// [start] add\n",
        "@SP \n",
        "M=M-1 \n", // SPレジスタの値をデクリメント
        "A=M \n", 
        "D=M \n", // Dレジスタに数値を入れる
        "@SP \n",
        "M=M-1 \n", // 再びSPレジスタの値をデクリメント
        "A=M \n",
        "M=D+M \n", // M[SP] = M[SP+1] + M[SP]
        "// [end] add \n"
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
            "@SP \n",
            "M=M+1 \n", // SPレジスタの値をインクリメントする
            "// [end] push constant {n} \n",
        ), n=n)
}