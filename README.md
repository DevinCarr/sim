Sim
=======
A very simple virtual machine based on [this](https://blog.felixangell.com/virtual-machine-in-c/), but written in rust instead. This also acts as a beginner project in rust.

## Current ISA
Instruction      | Machine code  | Description
-----------------|---------------|------------
halt             | 0             | Halt the program
pushi `$im`      | 1,`i32`       | Push the immediate (`$im`) onto the stack
pushr `$im`      | 2,`i32`       | Push a register (location `$im`) onto the stack
pop              | 3             | Pop the top of the stack
add `src`,`dest` | 4,`i32`,`i32` | Add the `src` and `dest` registers and put back into `dest`
movi `$im`,`dest`| 5,`i32`,`i32` | Move the `$im` into the register at `dest`
movr `src`,`dest`| 6,`i32`,`i32` | Move the value from `src` to `dest`
log              | 7             | Print out the current stack

#### Memory:
Stack size: `16`

Registers:

Register | Value
---------|-------
eax      | 0
ebx      | 1
ecx      | 2
edx      | 3

# Running

*See prog.si for example format*

`$ sim < prog.si`
