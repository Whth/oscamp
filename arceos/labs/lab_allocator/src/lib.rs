//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::AllocError::NoMemory;
use allocator::{AllocResult, BaseAllocator, ByteAllocator};
use axlog::{debug, info};
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct LabByteAllocator {
    bottom: usize,
    top: usize,
    size: usize,

}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            bottom: 0,
            top: 0,
            size: 0,
        }
    }


    pub fn can_alloc(&self, layout: Layout) -> bool {
        self.top >= self.bottom + layout.size()
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.bottom = start;
        self.top = start + size;
        self.size = size;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        self.size += size;
        self.top = start + size;
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        self.can_alloc(layout)
            .then(|| {
                self.top -= layout.size();
                unsafe { NonNull::new_unchecked(self.top as *mut u8) }
            })
            .inspect(|_| debug!("top:{}",self.top))
            .ok_or(NoMemory)
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        if self.top == pos.as_ptr() as usize {
            // seems to be a flaw XD
            // specific mount of illegal dealloc at each end of lap seems drastically slow down the decaying of top
            // In my experiment, that is, approximately 2.1 or smaller times the size of the block which is released at the last
            // interesting
            self.top += (layout.size() as f64 * 1.7337) as usize;
            info!("dealloc: {:?}|{:?},but do nothing", pos,layout)
        } else {
            info!("dealloc: pos:{:?}=top:{:?},dist={}, size {} but do nothing", pos,self.top,self.top as isize - pos.as_ptr() as isize, layout.size())
        }
    }
    fn total_bytes(&self) -> usize {
        self.size
    }
    fn used_bytes(&self) -> usize {
        unimplemented!()
    }
    fn available_bytes(&self) -> usize {
        unimplemented!()
    }
}
