use std::alloc::{self, Layout};
use std::ptr::NonNull;
use crate::core::{Error, Result};

/// Memory region with manual memory management
pub struct Region {
    ptr: NonNull<u8>,
    capacity: usize,
    size: usize,
    layout: Layout,
}

impl Region {
    /// Create a new memory region with the given capacity
    pub fn new(capacity: usize) -> Result<Self> {
        let layout = Layout::array::<u8>(capacity)
            .map_err(|_| Error::OutOfMemory)?;
            
        let ptr = unsafe {
            NonNull::new(alloc::alloc(layout))
                .ok_or(Error::OutOfMemory)?
        };
        
        Ok(Region {
            ptr,
            capacity,
            size: 0,
            layout,
        })
    }
    
    /// Allocate a block of memory
    pub fn allocate(&mut self, size: usize) -> Result<NonNull<u8>> {
        if self.size + size > self.capacity {
            return Err(Error::OutOfMemory);
        }
        
        let offset = self.size;
        self.size += size;
        
        Ok(unsafe { NonNull::new_unchecked(self.ptr.as_ptr().add(offset)) })
    }
    
    /// Deallocate a previously allocated block
    pub fn deallocate(&mut self, ptr: NonNull<u8>, size: usize) {
        let offset = unsafe {
            ptr.as_ptr().offset_from(self.ptr.as_ptr()) as usize
        };
        
        if offset + size == self.size {
            self.size = offset;
        }
    }
    
    /// Get current size
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Get total capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get available space
    pub fn available(&self) -> usize {
        self.capacity - self.size
    }
    
    /// Clear the region
    pub fn clear(&mut self) {
        self.size = 0;
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}

// Memory page for virtual memory management
pub struct Page {
    data: Box<[u8; PAGE_SIZE]>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            data: Box::new([0; PAGE_SIZE]),
        }
    }
    
    pub fn read(&self, offset: usize) -> Result<u8> {
        if offset >= PAGE_SIZE {
            return Err(Error::InvalidOperation);
        }
        Ok(self.data[offset])
    }
    
    pub fn write(&mut self, offset: usize, value: u8) -> Result<()> {
        if offset >= PAGE_SIZE {
            return Err(Error::InvalidOperation);
        }
        self.data[offset] = value;
        Ok(())
    }
}

// Constants
pub const PAGE_SIZE: usize = 65536; // 64KB
pub const MAX_PAGES: usize = 16384; // 1GB total

// Memory manager for handling virtual memory
pub struct MemoryManager {
    pages: Vec<Option<Page>>,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            pages: Vec::new(),
        }
    }
    
    pub fn allocate_page(&mut self) -> Result<usize> {
        if self.pages.len() >= MAX_PAGES {
            return Err(Error::OutOfMemory);
        }
        
        let page_idx = self.pages.len();
        self.pages.push(Some(Page::new()));
        Ok(page_idx)
    }
    
    pub fn free_page(&mut self, page_idx: usize) -> Result<()> {
        if page_idx >= self.pages.len() {
            return Err(Error::InvalidOperation);
        }
        self.pages[page_idx] = None;
        Ok(())
    }
    
    pub fn read_memory(&self, addr: usize) -> Result<u8> {
        let page_idx = addr / PAGE_SIZE;
        let offset = addr % PAGE_SIZE;
        
        match self.pages.get(page_idx).and_then(|p| p.as_ref()) {
            Some(page) => page.read(offset),
            None => Err(Error::InvalidOperation),
        }
    }
    
    pub fn write_memory(&mut self, addr: usize, value: u8) -> Result<()> {
        let page_idx = addr / PAGE_SIZE;
        let offset = addr % PAGE_SIZE;
        
        match self.pages.get_mut(page_idx).and_then(|p| p.as_mut()) {
            Some(page) => page.write(offset, value),
            None => Err(Error::InvalidOperation),
        }
    }
}

// Memory utilities
pub mod utils {
    use super::*;
    
    pub fn copy_memory(
        src: NonNull<u8>,
        dst: NonNull<u8>,
        size: usize,
    ) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                src.as_ptr(),
                dst.as_ptr(),
                size,
            );
        }
    }
    
    pub fn zero_memory(ptr: NonNull<u8>, size: usize) {
        unsafe {
            std::ptr::write_bytes(ptr.as_ptr(), 0, size);
        }
    }
} 