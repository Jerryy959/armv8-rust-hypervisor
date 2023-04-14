use aarch64::*;
use register::cpu::CpReg;
use register::register_bitfields;


const SAVED_LINUX_REGS: usize = 17 * 2; // 17 pairs of registers, each containing 2 registers



#[derive(Debug)]
pub struct LinuxContext {
    pub sp: u64,
    pub elr_el1: u64, // Exception Link Register (ELR) for Exception Level 1。它存储了在异常发生时的程序计数器（PC）值，即发生异常的指令的地址。当处理器从异常处理程序返回时，它会从ELR_EL1中读取返回地址。

    pub x30: u64,
    pub x29: u64,
    pub x28: u64,
    pub x27: u64,
    pub x26: u64,
    pub x25: u64,
    pub x24: u64,
    pub x23: u64,
    pub x22: u64,
    pub x21: u64,
    pub x20: u64,
    pub x19: u64,
    pub x18: u64,

    pub ttbr0_el1: u64,
    pub ttbr1_el1: u64, // Translation Table Base Register 0 和 1 for Exception Level 1。这两个寄存器分别存储了内存翻译表的基地址，它们用于虚拟地址到物理地址的翻译过程。在ARMv8架构中，可以使用两个不同的翻译表来处理不同范围的虚拟地址空间。0是低地址，1是高地址。
    pub mair_el1: u64, // Memory Attribute Indirection Register for Exception Level 1。此寄存器定义了内存属性，例如设备内存、正常内存和非缓存内存。这些属性用于在内存翻译过程中确定访问特定内存区域时使用的缓存策略和访问权限。
    pub sctlr_el1: u64, // System Control Register for Exception Level 1。此寄存器控制处理器的一些系统级特性，例如MMU（内存管理单元）的启用/禁用、指令和数据缓存的启用/禁用等。
    pub tcr_el1: u64, // Translation Control Register for Exception Level 1。它包含控制地址翻译过程的设置，如虚拟地址空间大小、页大小、翻译表的级别等。
    pub spsr_el1: u64, // Saved Program Status Register for Exception Level 1。当处理器从较高异常级别（如EL2或EL3）降到较低异常级别（如EL1）时，它保存了降级前的程序状态寄存器（CPSR）的值。当处理器从异常处理程序返回时，它会从SPSR_EL1中恢复程序状态寄存器的值。
}


#[repr(C)]
#[derive(Debug, Default)]
pub struct GeneralRegisters {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
}

macro_rules! save_regs_to_stack {
    () => {
        "
        sub     sp, sp, 34 * 8
        stp     x0, x1, [sp, 0 * #8]
        stp     x2, x3, [sp, 2 * #8]
        stp     x4, x5, [sp, 4 * #8]
        stp     x6, x7, [sp, 6 * #8]
        stp     x8, x9, [sp, 8 * #8]
        stp     x10, x11, [sp, 10 * #8]
        stp     x12, x13, [sp, 12 * #8]
        stp     x14, x15, [sp, 14 * #8]
        stp     x16, x17, [sp, 16 * #8]
        stp     x18, x19, [sp, 18 * #8]
        stp     x20, x21, [sp, 20 * #8]
        stp     x22, x23, [sp, 22 * #8]
        stp     x24, x25, [sp, 24 * #8]
        stp     x26, x27, [sp, 26 * #8]
        stp     x28, x29, [sp, 28 * #8]
        mrs     x9, sp_el1
        mrs     x10, elr_el2
        mrs     x11, spsr_el2
        stp     x30, x9, [sp, 30 * 8]
        stp     x10, x11, [sp, 32 * 8]
        "
    };
}

macro_rules! restore_regs_from_stack {
    () => {
        "
        ldp     x10, x11, [sp, 32 * 8]
        ldp     x30, x9, [sp, 30 * 8]
        msr     sp_el1, x9
        msr     elr_el2, x10
        msr     spsr_el2, x11
    
        ldp     x28, x29, [sp, 28 * 8]
        ldp     x26, x27, [sp, 26 * 8]
        ldp     x24, x25, [sp, 24 * 8]
        ldp     x22, x23, [sp, 22 * 8]
        ldp     x20, x21, [sp, 20 * 8]
        ldp     x18, x19, [sp, 18 * 8]
        ldp     x16, x17, [sp, 16 * 8]
        ldp     x14, x15, [sp, 14 * 8]
        ldp     x12, x13, [sp, 12 * 8]
        ldp     x10, x11, [sp, 10 * 8]
        ldp     x8, x9, [sp, 8 * 8]
        ldp     x6, x7, [sp, 6 * 8]
        ldp     x4, x5, [sp, 4 * 8]
        ldp     x2, x3, [sp, 2 * 8]
        ldp     x0, x1, [sp]
        add     sp, sp, 34 * 8
        "
    };
}

impl LinuxContext {
    /// Load linux callee-saved registers from the stack, and other system registers.
    pub fn load_from(linux_sp: usize) -> Self {
        let regs = unsafe { core::slice::from_raw_parts(linux_sp as *const u64, SAVED_LINUX_REGS) };

        Self {
            sp: regs.as_ptr_range().end as _,

            // Load general-purpose registers
            x0: regs[0],
            x1: regs[1],
            x2: regs[2],
            x3: regs[3],
            x4: regs[4],
            x5: regs[5],
            x6: regs[6],
            x7: regs[7],
            x8: regs[8],
            x9: regs[9],
            x10: regs[10],
            x11: regs[11],
            x12: regs[12],
            x13: regs[13],
            x14: regs[14],
            x15: regs[15],
            x16: regs[16],
            x17: regs[17],
            x18: regs[18],
            x19: regs[19],
            x20: regs[20],
            x21: regs[21],
            x22: regs[22],
            x23: regs[23],
            x24: regs[24],
            x25: regs[25],
            x26: regs[26],
            x27: regs[27],
            x28: regs[28],
            x29: regs[29],

            // Load system registers
            sp_el1: regs[30],
            elr_el2: regs[31],
            spsr_el2: regs[32],

            // Load other system registers
            sctlr_el1: unsafe { aarch64::sctlr_el1() },
            mair_el1: unsafe { Msr::MAIR_EL1.read() },
            tcr_el1: unsafe { Msr::TCR_EL1.read() },
            ttbr0_el1: unsafe { Msr::TTBR0_EL1.read() },
            ttbr1_el1: unsafe { Msr::TTBR1_EL1.read() },
        }
    }

    /// Restore system registers.
    pub fn restore(&self) {
        // Restore other system registers
        unsafe {
            aarch64::set_ttbr0_el1(self.ttbr0_el1);
            aarch64::set_ttbr1_el1(self.ttbr1_el1);
            aarch64::set_mair_el1(self.mair_el1);
            aarch64::set_tcr_el1(self.tcr_el1);
            aarch64::set_sctlr_el1(self.sctlr_el1);
        }
    }

    /// Restore linux general-purpose registers and stack, then return back to linux.
    pub fn return_to_linux(&self, guest_regs: &GeneralRegisters) -> ! {
        unsafe {
            core::arch::asm!(
                "mov x0, {guest_regs}",
                "mov x1, {linux_rsp}",
                "mov x2, {linux_elr}",
                "mov x3, {linux_spsr}",
                restore_regs_from_stack!(),
                "msr sp_el1, x1",
                "msr elr_el2, x2",
                "msr spsr_el2, x3",
                "eret",
                guest_regs = in(reg) guest_regs,
                linux_rsp = in(reg) self.sp_el1,
                linux_elr = in(reg) self.elr_el2,
                linux_spsr = in(reg) self.spsr_el2,
            );
            core::intrinsics::unreachable();
        }
    }
}