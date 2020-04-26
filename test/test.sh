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
    # ".tst"は書かない
    ifarg "fibonacciseres" "../../ProgramFlow/FibonacciSeries/FibonacciSeries"; result+=($path)
    ifarg "basicloop" "../../ProgramFlow/BasicLoop/BasicLoop"; result+=($path)
    ifarg "if" "./if/test"; result+=($path)
    ifarg "goto" "./goto/test"; result+=($path)
    ifarg "push" "./push/test"; result+=($path)
    ifarg "pop" "./pop/test"; result+=($path)
    ifarg "lt" "./lt/lt"; result+=($path)
    ifarg "gt" "./gt/gt"; result+=($path)
    ifarg "eq" "./eq/eq/eq"; result+=($path)
    ifarg "eq_true" "./eq/true/true"; result+=($path)
    ifarg "eq_false" "./eq/false/false"; result+=($path)
    ifarg "eq_long" "./eq/long/long"; result+=($path)
    ifarg "add" "./add/add"; result+=($path)
    ifarg "sub" "./sub/sub"; result+=($path)
    # 7章のテスト
    ifarg "simpleadd" "../../../07/StackArithmetic/SimpleAdd/SimpleAdd"; result+=($path)
    ifarg "stacktest" "../../../07/StackArithmetic/StackTest/StackTest"; result+=($path)
    ifarg "basictest" "../../../07/MemoryAccess/BasicTest/BasicTest"; result+=($path)
    ifarg "pointertest" "../../../07/MemoryAccess/PointerTest/PointerTest"; result+=($path)
    ifarg "statictest" "../../../07/MemoryAccess/StaticTest/StaticTest"; result+=($path)


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
        if [[ $compile == "true" ]]; then
            printf "[cmpiled] "
            $vmtranslator $path".vm" $path".asm"
        fi
        printf $path" : "
        printf "\x1b[31m" # 文字を赤色にする
        test_result=$(test $path".tst")
        if [[ $test_result == "End of script - Comparison ended successfully" ]]; then
            printf "\x1b[32m" # 文字を黄緑色にする
            echo $test_result
        fi
        printf "\x1b[0m" # 文字色を戻す
    done
done

