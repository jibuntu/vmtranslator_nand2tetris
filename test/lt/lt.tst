// ltのテスト

load lt.asm,
output-file lt.out,
compare-to lt.cmp,
output-list RAM[0]%D2.6.2 RAM[256]%D2.6.2 RAM[257]%D2.6.2 RAM[258]%D2.6.2;

set RAM[0] 256,  // initializes the stack pointer 

repeat 120 {     // enough cycles to complete the execution
  ticktock;
}

output;          // the stack pointer and the stack base
