// popのテスト

load test.asm,
output-file test.out,
compare-to test.cmp,
output-list RAM[0]%D2.6.2 RAM[256]%D2.6.2 RAM[257]%D2.6.2;

set RAM[0] 256,  // initializes the stack pointer
set RAM[1] 100,  // localセグメントの値を設定

repeat 60 {     // enough cycles to complete the execution
  ticktock;
}

output;          // the stack pointer and the stack base
