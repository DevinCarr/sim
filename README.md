Sim
=======
A very simple virtual machine based on [this](http://www.felixangell.com/virtual-machine-in-c/), but written in rust instead. This also acts as a beginner project in rust.

## Current ISA
Instruction | Machine code | Description
------------|--------------|------------
halt        | 0            | Halt the program
push `i32`  | 1,`i32`      | Push the value `i32` onto the stack
pop         | 2            | Pop the top of the stack
add         | 3            | Add the top two values on the stack and push back on the stack

#### Memory:
Stack size: `4`

*Note: In the `inst` static array, the length needs to be noted in the struct `Mem`.*
