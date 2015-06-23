#[derive(Debug)]
struct Mem {
    inst: [i32; 13],
    stack: [i32; 4],
    sp: i32,
    ip: usize,
}

// Match the correct instruction with a function
fn exec(mem: &mut Mem) {
    match mem.inst[mem.ip] {
        0 => push(mem),
        1 => pop(mem),
        _ => er(mem),
    }
}

// Instruction push (val)
fn push(mem: &mut Mem) {
    mem.sp += 1; // increment stack pointer to new top
    // check for a stack overflow
    if mem.sp >= mem.stack.len() as i32 {
        println!("MEMORY DUMP: {:?}",mem);
        panic!("Stack overflow!");
    }
    mem.ip += 1; // increment instruction pointer to the value
    mem.stack[mem.sp as usize] = mem.inst[mem.ip];
}

// Instruction pop ()
fn pop(mem: &mut Mem) {
    if mem.sp < 0 {
        println!("MEMORY DUMP: {:?}",mem);
        panic!("Already an Empty stack, cannot pop!");
    }
    // zero the top of the stack
    mem.stack[mem.sp as usize] = 0;
    // decrement the stack pointer
    mem.sp -= 1;
}

// Report an invalid instruction
fn er(mem: &mut Mem) {
    panic!("Error instruction: {:?}", mem.inst[mem.ip]);
}

fn main() {
    // initialize the memory with a zero'd stack, sp at -1, and ip starting at 0
    let mut mem: Mem = Mem { inst: [0,1,0,2,0,3,0,4,1,1,1,1,1], stack:[0,0,0,0], sp:-1, ip:0 };
    println!("{:?}",mem);
    // run through the instructions
    while mem.ip < mem.inst.len() {
        exec(&mut mem);
        mem.ip += 1; // Increment instruction pointer to next instrcution
    }
    println!("{:?}",mem);
}
