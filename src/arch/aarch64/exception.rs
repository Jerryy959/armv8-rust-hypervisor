use core::arch::{asm, global_asm};

use super::context::GeneralRegisters;

global_asm!(include_str!(concat!(env!("OUT_DIR"), "/exception_aarch64.S")));

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ExceptionType {
    pub const Sync: u8 = 0;
    pub const Irq: u8 = 1;
    pub const Fiq: u8 = 2;
    pub const SError: u8 = 3;
}

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    // Pushed by `common_exception_entry`
    pub regs: GeneralRegisters,

    // Pushed by 'exception_aarch64.S'
    pub esr_el1: u64,
    pub elr_el1: u64,
    pub spsr_el1: u64,
}

fn exception_handler(frame: &TrapFrame) {
    trace!("Exception or interrupt #{:#x}", frame.esr_el1);
    let ec = (frame.esr_el1 >> 26) as u8;
    match ec {
        ExceptionType::Sync => handle_sync_exception(frame),
        ExceptionType::Irq => handle_irq(frame),
        ExceptionType::Fiq => handle_fiq(frame),
        ExceptionType::SError => handle_serror(frame),
        _ => {
            error!("{:#x?}", frame);
            panic!("Unhandled exception #{:#x}", frame.esr_el1);
        }
    }
}

fn handle_sync_exception(frame: &TrapFrame) {
    // Handle synchronous exceptions (e.g., undefined instructions, SVC)
}

fn handle_irq(frame: &TrapFrame) {
    // Handle IRQ
}

fn handle_fiq(frame: &TrapFrame) {
    // Handle FIQ
}

fn handle_serror(frame: &TrapFrame) {
    // Handle System Errors
}

#[naked]
#[no_mangle]
unsafe extern "C" fn common_exception_entry() -> ! {
    asm!(
        save_regs_to_stack!(),
        "mov x0, sp",
        "bl {0}",
        restore_regs_from_stack!(),
        "eret",
        sym exception_handler,
        options(noreturn),
    );
}
