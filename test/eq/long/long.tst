// eqのテスト

load long.asm,
output-file long.out,
compare-to long.cmp,
output-list RAM[0]%D2.6.2 RAM[256]%D2.6.2 RAM[257]%D2.6.2 RAM[258]%D2.6.2 RAM[259]%D2.6.2 RAM[260]%D2.6.2 RAM[261]%D2.6.2;

set RAM[0] 256,  // initializes the stack pointer 

repeat 250 {     // enough cycles to complete the execution
  ticktock;
}

output;          // the stack pointer and the stack base
