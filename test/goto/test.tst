// gotoのテスト

load test.asm,
output-file test.out,
compare-to test.cmp,
output-list RAM[0]%D2.6.2 RAM[256]%D2.6.2 RAM[257]%D2.6.2 RAM[258]%D2.6.2 RAM[259]%D2.6.2;

repeat 100 {     // enough cycles to complete the execution
  ticktock;
}

output;          // the stack pointer and the stack base
