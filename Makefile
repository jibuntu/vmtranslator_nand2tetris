# 他のディレクトリにある問題用の*.vmファイルをコンパイルするためのMakefile

fibonacciseres:
	cargo run ../ProgramFlow/FibonacciSeries/FibonacciSeries.vm ../ProgramFlow/FibonacciSeries/FibonacciSeries.asm

basicloop:
	cargo run ../ProgramFlow/BasicLoop/BasicLoop.vm ../ProgramFlow/BasicLoop/BasicLoop.asm

goto:
	cargo run ./test/goto/test.vm ./test/goto/test.asm

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
