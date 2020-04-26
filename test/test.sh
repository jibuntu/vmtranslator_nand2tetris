#!/bin/bash
# コマンドラインでcpuemulatorを使ったテストをする

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
    ifarg "fibonacciseres" "../../ProgramFlow/FibonacciSeries/FibonacciSeries.tst"; result+=($path)
    ifarg "basicloop" "../../ProgramFlow/BasicLoop/BasicLoop.tst"; result+=($path)
    ifarg "if" "./if/test.tst"; result+=($path)
    ifarg "goto" "./goto/test.tst"; result+=($path)
    ifarg "push" "./push/test.tst"; result+=($path)
    ifarg "pop" "./pop/test.tst"; result+=($path)
    ifarg "lt" "./lt/lt.tst"; result+=($path)
    ifarg "gt" "./gt/gt.tst"; result+=($path)
    ifarg "eq" "./eq/eq/eq.tst"; result+=($path)
    ifarg "eq_true" "./eq/true/true.tst"; result+=($path)
    ifarg "eq_false" "./eq/false/false.tst"; result+=($path)
    ifarg "eq_long" "./eq/long/long.tst"; result+=($path)
    ifarg "add" "./add/add.tst"; result+=($path)
    ifarg "sub" "./sub/sub.tst"; result+=($path)
}


while [[ $1 != "" ]]; do
    arg=$1
    shift 1

    result=()
    get_test_list $arg

    if [[ ${result[@]} == "" ]]; then
        echo $arg"というテストはありません"
        exit
    fi
    
    for path in ${result[@]}; do
        printf $path" : "
        test $path
    done
done

