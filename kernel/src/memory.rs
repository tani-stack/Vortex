//! Memory Management
//! 
//! Provides virtual memory, page tables, and protection domains

use aero_types::AeroResult;

/// Page size (4KB for ARM)
pub const PAGE_SIZE: usize = 4096;

/// Virtual address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualAddress(pub u32);

/// Physical address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalAddress(pub u32);

/// Page table entry flags
pub struct PageTableFlags {
    /// Present bit
    pub present: bool,
    /// Writable bit
    pub writable: bool,
    /// User accessible
    pub user: bool,
    /// Execute disable
    pub no_execute: bool,
}

impl PageTableFlags {
    /// Kernel read-only page
    pub const KERNEL_RO: Self = PageTableFlags {
        present: true,
        writable: false,
        user: false,
        no_execute: false,
    };

    /// Kernel read-write page
    pub const KERNEL_RW: Self = PageTableFlags {
        present: true,
        writable: true,
        user: false,
        no_execute: false,
    };

    /// User read-only page
    pub const USER_RO: Self = PageTableFlags {
        present: true,
        writable: false,
        user: true,
        no_execute: false,
    };

    /// User read-write page
    pub const USER_RW: Self = PageTableFlags {
        present: true,
        writable: true,
        user: true,
        no_execute: false,
    };

    /// Code page (no execute disabled means executable)
    pub const CODE: Self = PageTableFlags {
        present: true,
        writable: false,
        user: false,
        no_execute: false,
    };
}

/// Page table
pub struct PageTable {
    /// Entries (1024 for 2-level paging on ARM)
    entries: [u32; 1024],
}

impl PageTable {
    /// Create a new page table
    pub fn new() -> Self {
        Self {
            entries: [0; 1024],
        }
    }

    /// Map a virtual page to physical page
    pub fn map(&mut self, virt: VirtualAddress, phys: PhysicalAddress, flags: PageTableFlags) {
        let index = (virt.0 >> 12) & 0x3FF;
        let mut entry = phys.0 & 0xFFFFF000;
        
        if flags.present { entry |= 1; }
        if flags.writable { entry |= 2; }
        if flags.user { entry |= 4; }
        if flags.no_execute { entry |= 0x80000000; }
        
        self.entries[index as usize] = entry;
    }

    /// Unmap a virtual page
    pub fn unmap(&mut self, virt: VirtualAddress) {
        let index = (virt.0 >> 12) & 0x3FF;
        self.entries[index as usize] = 0;
    }

    /// Translate virtual to physical address
    pub fn translate(&self, virt: VirtualAddress) -> Option<PhysicalAddress> {
        let index = (virt.0 >> 12) & 0x3FF;
        let entry = self.entries[index as usize];
        
        if (entry & 1) == 0 {
            return None;
        }
        
        let phys_addr = entry & 0xFFFFF000;
        Some(PhysicalAddress(phys_addr | (virt.0 & 0xFFF)))
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory manager
pub struct MemoryManager {
    /// Kernel page table
    kernel_page_table: PageTable,
    /// Free frame stack
    free_frames: alloc::vec::Vec<PhysicalAddress>,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            kernel_page_table: PageTable::new(),
            free_frames: alloc::vec::Vec::new(),
        }
    }

    /// Allocate a physical frame
    pub fn alloc_frame(&mut self) -> Option<PhysicalAddress> {
        self.free_frames.pop()
    }

    /// Free a physical frame
    pub fn free_frame(&mut self, frame: PhysicalAddress) {
        self.free_frames.push(frame);
    }

    /// Create a new address space (for a new task)
    pub fn create_address_space(&self) -> PageTable {
        PageTable::new()
    }

    /// Enable paging on a page table
    pub fn enable_paging(&self, _page_table: &PageTable) {
        // Platform-specific: write page table address to MMU register
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

static mut MEMORY_MANAGER: Option<MemoryManager> = None;

/// Initialize memory manager
pub fn init() -> AeroResult<()> {
    unsafe {
        MEMORY_MANAGER = Some(MemoryManager::new());
    }
    Ok(())
}

/// Get memory manager
pub fn memory_manager() -> &'static mut MemoryManager {
    unsafe {
        MEMORY_MANAGER.as_mut().expect("Memory manager not initialized")
    }
}