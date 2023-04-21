use crate::percpu::PerCpu;

#[naked]
#[no_mangle]
pub unsafe extern "C" fn arch_entry() -> i32 {
    println!("Welcome to AArch64 Hypervisor\n");
    
    // core::arch::asm!(
        
    // );
    0

}
