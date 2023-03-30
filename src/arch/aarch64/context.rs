


#[derive(Debug)]
pub struct LinuxContext { // 寄存器
    x     :      [u64; 31],
    spsr  :       u64,
    elr   :       u64,
    sp    :       u64,
    tpidr :       u64,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct GeneralRegisters { // 栈帧
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    
    pub  fp: u64,
    pub  sp: u64,
    pub  pc: u64,

}

macro_rules! save_regs_to_stack {
    () => {
        "
        
        "
    };
}

macro_rules! restore_regs_from_stack {
    () => {
        "
        
        "
    };
}

impl LinuxContext {
    /// Load linux callee-saved registers from the stack, and other system registers.
    pub fn load_from (linux_sp: usize) -> Self {

    }

    /// Restore system registers.
    pub fn restore (&self) {

    }

    /// Restore linux general-purpose registers and stack, then return back to linux.
    pub fn return_to_linux (&self, guest_regs: &GeneralRegisters) -> ! {

    }
}
