# 他のディレクトリにある問題用の*.vmファイルをコンパイルするためのMakefile

simpleadd:
	cargo run ../StackArithmetic/SimpleAdd/SimpleAdd.vm ../StackArithmetic/SimpleAdd/SimpleAdd.asm

stacktest:
	cargo run ../StackArithmetic/StackTest/StackTest.vm ../StackArithmetic/StackTest/StackTest.asm

eq:
	cargo run ./test/eq/eq.vm ./test/eq/eq.asm
