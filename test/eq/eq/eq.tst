// eqのテスト

load eq.asm,
output-file eq.out,
compare-to eq.cmp,
output-list RAM[0]%D2.6.2 RAM[256]%D2.6.2 RAM[257]%D2.6.2;

set RAM[0] 256,  // initializes the stack pointer 

repeat 100 {     // enough cycles to complete the execution
  ticktock;
}

output;          // the stack pointer and the stack base
