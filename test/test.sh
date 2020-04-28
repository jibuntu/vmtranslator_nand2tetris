#!/bin/bash
# コマンドラインでcpuemulatorを使ったテストをする

vmtranslator="./../target/debug/vmtranslator"
emulator="./../../../../tools/CPUEmulator.sh"

test() {
    sh $emulator $1
}

# 第一引数はテスト名、第二引数は.tstファイルへのパス
# arg変数の値がテスト名または"all"と同じだったらpath変数にパスが入る
ifarg() {
    name=$1
    path=""
    if [[ $arg == $name || $arg == "all" ]]; then
        path=$2
    fi
}
# テストで使う.tstファイルへパスの配列を返す
# 引数はテスト名
get_test_list() {
    arg=$1
    path=""
    result=()

    # pash変数の値が""なら配列には何も追加されない
    # Sys.init関数がない場合は[-w]をつける
    # ".tst"は書かない
    ifarg "staticstest" "../../FunctionCalls/StaticsTest/StaticsTest"; result+=($path)
    ifarg "nestedcall" "../../FunctionCalls/NestedCall/NestedCall"; result+=($path)
    ifarg "fibonaccielement" "../../FunctionCalls/FibonacciElement/FibonacciElement"; result+=($path)
    ifarg "simplefunction" "[-w]../../FunctionCalls/SimpleFunction/SimpleFunction"; result+=($path)
    ifarg "fibonacciseres" "[-w]../../ProgramFlow/FibonacciSeries/FibonacciSeries"; result+=($path)
    ifarg "basicloop" "[-w]../../ProgramFlow/BasicLoop/BasicLoop"; result+=($path)
    ifarg "if" "[-w]./if/test"; result+=($path)
    ifarg "goto" "[-w]./goto/test"; result+=($path)
    ifarg "push" "[-w]./push/test"; result+=($path)
    ifarg "pop" "[-w]./pop/test"; result+=($path)
    ifarg "lt" "[-w]./lt/lt"; result+=($path)
    ifarg "gt" "[-w]./gt/gt"; result+=($path)
    ifarg "eq" "[-w]./eq/eq/eq"; result+=($path)
    ifarg "eq_true" "[-w]./eq/true/true"; result+=($path)
    ifarg "eq_false" "[-w]./eq/false/false"; result+=($path)
    ifarg "eq_long" "[-w]./eq/long/long"; result+=($path)
    ifarg "add" "[-w]./add/add"; result+=($path)
    ifarg "sub" "[-w]./sub/sub"; result+=($path)
    # 7章のテスト
    ifarg "simpleadd" "[-w]../../../07/StackArithmetic/SimpleAdd/SimpleAdd"; result+=($path)
    ifarg "stacktest" "[-w]../../../07/StackArithmetic/StackTest/StackTest"; result+=($path)
    ifarg "basictest" "[-w]../../../07/MemoryAccess/BasicTest/BasicTest"; result+=($path)
    ifarg "pointertest" "[-w]../../../07/MemoryAccess/PointerTest/PointerTest"; result+=($path)
    ifarg "statictest" "[-w]../../../07/MemoryAccess/StaticTest/StaticTest"; result+=($path)


}

args=() # 引数のリスト
compile="" # コンパイルするかどうか

# 引数をパースする
# -cオプションはコンパイルをする
while [[ $1 != "" ]]; do
    if [[ $1 == "-c" ]]; then
        compile="true"
    else
        args+=($1)
    fi
    
    shift 1
done

for arg in ${args[@]}; do
    shift 1

    result=()
    get_test_list $arg

    if [[ ${result[@]} == "" ]]; then
        echo $arg"というテストはありません"
        exit
    fi

    if [[ $compile == "true" ]]; then
        cargo build
    fi
    
    for path in ${result[@]}; do
        asm_path=$(echo $path | sed -e "s/^\[-w\]*//") # 引数を取り除く
        if [[ $compile == "true" ]]; then
            # コンパイルするときは.vmファイルではなく.vmファイルが入っているディレクトリをコンパイル対象にする
            # したがって、$pathが"./test/vmtest"だったら"./test"がコンパイルする対象になる
            # コンパイル後のファイル名は$path".asm"にする
            vm_path=$(echo $path | sed -e "s/[^/]*$//") # ファイル名の部分を取り除く
            vm_path=$(echo $vm_path | sed -e "s/^\[-w\]*/-w /") # 引数があれば引数を設定する
            printf "[cmpiled] "
            $vmtranslator $vm_path $asm_path".asm"
        fi

        printf $asm_path" : "
        printf "\x1b[31m" # 文字を赤色にする
        test_result=$(test $asm_path".tst")
        if [[ $test_result == "End of script - Comparison ended successfully" ]]; then
            printf "\x1b[32m" # 文字を黄緑色にする
            echo $test_result
        fi
        printf "\x1b[0m" # 文字色を戻す
    done
done

