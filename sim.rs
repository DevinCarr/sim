use std::io;

#[derive(Debug)]
#[allow(dead_code)]
struct Mem {
    program: [i32; 32],
    registers: [i32; 7],
    stack: [i32; 16],
    running: bool,
}

#[allow(dead_code)]
impl Mem {
    // Instruction pointer stored at 0 in register memory
    fn ip(&self) -> usize {
        return self.registers[0] as usize;
    }
    fn ipi(&mut self) {
        return self.registers[0] += 1;
    }
    // Stack pointer stored at 1 in register memory
    fn sp(&self) -> usize {
        return self.registers[1] as usize;
    }
    // Alter the stack pointer by an integer value
    fn spi(&mut self,i:i32) {
        self.registers[1] -= i;
    }
    // Base pointer stored at 2 in register memory
    fn bp(&self) -> usize {
        return self.registers[2] as usize;
    }
    // Return the register values
    fn reg(&mut self) -> &mut[i32] {
        return &mut self.registers[3..];
    }
}


// Match the correct instruction with a function
fn exec(mem: &mut Mem) {
    match mem.program[mem.ip()] {
        0 => halt(mem),
        1 => pushi(mem),
        2 => pushr(mem),
        3 => pop(mem),
        4 => add(mem),
        5 => movi(mem),
        6 => movr(mem),
        7 => log(mem),
        _ => er(mem, "Invalid instruction!"),
    }
}

// Instruction halt ()
fn halt(mem: &mut Mem) {
    mem.running = false;
}

// Instruction push (val)
// Push a value onto the stack
fn pushi(mem: &mut Mem) {
    // check for a stack overflow
    if mem.sp() == 0 {
        er(mem, "Stack overflow!");
    }
    mem.spi(1); // increment stack pointer to new top
    mem.ipi(); // increment instruction pointer to the value
    mem.stack[mem.sp()] = mem.program[mem.ip()];
}

// Instruction push (reg*) a register ptr
// Push a register onto the stack
fn pushr(mem: &mut Mem) {
    // check for a stack overflow
    if mem.sp() == 0 {
        er(mem, "Stack overflow!");
    }
    mem.spi(1); // increment stack pointer to new top
    mem.ipi(); // increment instruction pointer to the value
    mem.stack[mem.sp()] = mem.reg()[mem.program[mem.ip()] as usize];
}

// Instruction pop ()
fn pop(mem: &mut Mem) {
    // zero the top of the stack
    mem.stack[mem.sp()] = 0;
    // decrement the stack pointer
    if mem.sp() < mem.stack.len() {
        mem.spi(-1);
    }
}

// Instruction add (src*,dest*) *must be registers
// Add two register values (src and dest) and store back in dest
fn add(mem: &mut Mem) {
    // fetch value
    mem.ipi();
    let src = mem.reg()[mem.program[mem.ip()] as usize];
    // get the dest
    mem.ipi();
    let val = mem.reg()[mem.program[mem.ip()] as usize];
    // add values and put back
    mem.reg()[mem.program[mem.ip()] as usize] = val + src;
}

// Instruction mov ($im,dest*) *must be registers
// Move an immediate value ($im) to dest register
fn movi(mem: &mut Mem) {
    mem.ipi();
    let val = mem.program[mem.ip()];
    mem.ipi();
    let dest = mem.program[mem.ip()];
    if dest >= mem.reg().len() as i32 || dest < 0 {
        er(mem,"Invalid register access!");
    }
    mem.reg()[dest as usize] = val;
}

// Instruction mov (src*,dest*) *must be registers
// Move values between two registers from src to dest
fn movr(mem: &mut Mem) {
    mem.ipi();
    // check valid register location
    let src = mem.program[mem.ip()];
    if src >= mem.reg().len() as i32 || src < 0 {
        er(mem,"Invalid register access!");
    }
    mem.ipi();
    // check valid register location
    let dest = mem.program[mem.ip()];
    if dest >= mem.reg().len() as i32 || dest < 0 {
        er(mem,"Invalid register access!");
    }
    // store src in dest
    mem.reg()[dest as usize] = mem.reg()[src as usize];
}

// Instruction log ()
// Prints out the current stack
fn log(mem: &mut Mem) {
    println!("log: {:?}",mem.stack);
}

// Report an invalid instruction
fn er(mem: &mut Mem, message: &str) {
    println!("MEMORY DUMP: {:?}",mem);
    panic!("Error: {:?}", message);
}

fn main() {
    // initialize the memory with a zero'd stack
    let mut mem: Mem = Mem {
        program:[0;32],
        registers:[0,16,0,0,0,0,0],
        stack:[0;16],
        running:false,
    };
    let mut index = 0;

    let mut input_s = String::new();
    // main loop
    loop {
        match io::stdin().read_line(&mut input_s) {
            Ok(n) => {
                if n == 0 {
                    mem.running = true;
                    // run through the instructions
                    while mem.running {
                        exec(&mut mem);
                        mem.ipi(); // Increment instruction pointer to next instruction
                    }
                    break;
                }
                let input: i32 = input_s.trim().parse::<i32>().ok().expect("str-to-i32 conv failed");
                mem.program[index] = input;
                index += 1;
                input_s = String::new();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
