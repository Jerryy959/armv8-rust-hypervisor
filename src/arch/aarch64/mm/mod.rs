pub mod page_table;

use core::marker::PhantomData;

pub use page_table::{GenericPTE, Level4PageTable};

pub const PAGE_SIZE: usize = 0x1000;

/// Guest virtual address.
pub type GuestVirtAddr = usize;
/// Guest physical address.
pub type GuestPhysAddr = usize;
/// Host virtual address.
pub type HostVirtAddr = usize;
/// Host physical address.
pub type HostPhysAddr = usize;

bitflags::bitflags! {
    /// Permission and type of a guest physical memory region.
    pub struct MemFlags: u64 {
        const READ          = 1 << 0;
        const WRITE         = 1 << 1;
        const EXECUTE       = 1 << 2;
        const DEVICE        = 1 << 3;
        const USER          = 1 << 4;
    }
}