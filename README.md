Sim
=======
A very simple virtual machine based on [this](http://www.felixangell.com/virtual-machine-in-c/), but written in rust instead. This also acts as a beginner project in rust.

## Current ISA
Instruction | Machine code | Description
------------|--------------|------------
push `i32`  | 0,`i32`      | Push the value `i32` onto the stack
pop         | 1            | Pop the top of the stack

#### Memory:
Stack size: `4`

*Note: In the `inst` static array, the length needs to be noted in the struct `Mem`.*
