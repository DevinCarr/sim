#[derive(Debug)]
struct Mem {
    inst: [i32; 12],
    stack: [i32; 4],
    sp: i32,
    ip: usize,
    running: bool,
}

// Match the correct instruction with a function
fn exec(mem: &mut Mem) {
    match mem.inst[mem.ip] {
        0 => halt(mem),
        1 => push(mem),
        2 => pop(mem),
        3 => add(mem),
        _ => er(mem, "Invalid instruction!"),
    }
}

// Instruction halt ()
fn halt(mem: &mut Mem) {
    mem.running = false;
}

// Instruction push (val)
fn push(mem: &mut Mem) {
    mem.sp += 1; // increment stack pointer to new top
    // check for a stack overflow
    if mem.sp >= mem.stack.len() as i32 {
        er(mem, "Stack overflow!");
    }
    mem.ip += 1; // increment instruction pointer to the value
    mem.stack[mem.sp as usize] = mem.inst[mem.ip];
}

// Instruction pop ()
fn pop(mem: &mut Mem) {
    if mem.sp < 0 {
        er(mem, "Already an Empty stack, cannot pop!");
    }
    // zero the top of the stack
    mem.stack[mem.sp as usize] = 0;
    // decrement the stack pointer
    mem.sp -= 1;
}

// Instruction add ()
fn add(mem: &mut Mem) {
    // check for two items on the stack
    if mem.sp < 1 {
        er(mem, "Stack has less than two items!");
    }
    // fetch the top value on the stack
    let x = mem.stack[mem.sp as usize];
    // pop the value
    pop(mem);
    // get the next value on the stack
    let y = mem.stack[mem.sp as usize];
    // add them together and put on top of stack
    mem.stack[mem.sp as usize] = x + y;
}

// Report an invalid instruction
fn er(mem: &mut Mem, message: &str) {
    println!("MEMORY DUMP: {:?}",mem);
    panic!("Error: {:?}", message);
}

fn main() {
    // initialize the memory with a zero'd stack, sp at -1, and ip starting at 0
    let mut mem: Mem = Mem { inst: [1,1,1,2,1,3,1,4,2,2,3,0], stack:[0,0,0,0], sp:-1, ip:0, running:true };
    println!("{:?}",mem);
    // run through the instructions
    while mem.running {
        exec(&mut mem);
        mem.ip += 1; // Increment instruction pointer to next instrcution
    }
    println!("{:?}",mem);
}
