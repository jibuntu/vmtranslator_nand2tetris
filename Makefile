# 他のディレクトリにある問題用の*.vmファイルをコンパイルするためのMakefile

simpleadd:
	cargo run ../StackArithmetic/SimpleAdd/SimpleAdd.vm ../StackArithmetic/SimpleAdd/SimpleAdd.asm

stacktest:
	cargo run ../StackArithmetic/StackTest/StackTest.vm ../StackArithmetic/StackTest/StackTest.asm

basictest:
	cargo run ../MemoryAccess/BasicTest/BasicTest.vm ../MemoryAccess/BasicTest/BasicTest.asm

pointertest:
	cargo run ../MemoryAccess/PointerTest/PointerTest.vm ../MemoryAccess/PointerTest/PointerTest.asm

statictest:
	cargo run ../MemoryAccess/StaticTest/StaticTest.vm ../MemoryAccess/StaticTest/StaticTest.asm

push:
	cargo run ./test/push/test.vm ./test/push/test.asm

pop:
	cargo run ./test/pop/test.vm ./test/pop/test.asm

lt:
	cargo run ./test/lt/lt.vm ./test/lt/lt.asm

gt:
	cargo run ./test/gt/gt.vm ./test/gt/gt.asm

eq:
	cargo run ./test/eq/eq/eq.vm ./test/eq/eq/eq.asm

eq_true: 
	cargo run ./test/eq/true/true.vm ./test/eq/true/true.asm

eq_false: 
	cargo run ./test/eq/false/false.vm ./test/eq/false/false.asm

eq_long: 
	cargo run ./test/eq/long/long.vm ./test/eq/long/long.asm

add:
	cargo run ./test/add/add.vm ./test/add/add.asm

sub:
	cargo run ./test/sub/sub.vm ./test/sub/sub.asm
