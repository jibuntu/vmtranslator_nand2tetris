# 他のディレクトリにある問題用の*.vmファイルをコンパイルするためのMakefile

simpleadd:
	cargo run ../StackArithmetic/SimpleAdd/SimpleAdd.vm ../StackArithmetic/SimpleAdd/SimpleAdd.asm

stacktest:
	cargo run ../StackArithmetic/StackTest/StackTest.vm ../StackArithmetic/StackTest/StackTest.asm

eq:
	cargo run ./test/eq/eq/eq.vm ./test/eq/eq/eq.asm

add:
	cargo run ./test/add/add.vm ./test/add/add.asm

sub:
	cargo run ./test/sub/sub.vm ./test/sub/sub.asm
