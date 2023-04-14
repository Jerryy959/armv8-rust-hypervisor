.section .text
.global exception_vector_table
.global common_exception_entry

exception_vector_table:
    // Synchronous exception
    b sync_handler
    // IRQ
    b irq_handler
    // FIQ
    b fiq_handler
    // SError
    b serror_handler

sync_handler:
    // Save general-purpose registers and exception-related registers
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x8, x9, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    stp x18, x19, [sp, #-16]!
    stp x20, x21, [sp, #-16]!
    stp x22, x23, [sp, #-16]!
    stp x24, x25, [sp, #-16]!
    stp x26, x27, [sp, #-16]!
    stp x28, x29, [sp, #-16]!
    stp x30, xzr, [sp, #-16]!
    mrs x0, elr_el1
    mrs x1, spsr_el1
    mrs x2, esr_el1
    stp x0, x1, [sp, #-16]!
    stp x2, xzr, [sp, #-16]!

    // Call common_exception_entry
    bl common_exception_entry
    
.macro VECTOR_DISABLED
1:  wfe
    b      1b
.endm

irq_handler:
    // Handle IRQ
    b .

fiq_handler:
    // Handle FIQ
    b .

serror_handler:
    // Handle SError
    b .
