use rand_core::RngCore;
use core::mem;
use crate::sys::{self, SceKernelUtilsMt19937Context};

pub struct Mt19937(SceKernelUtilsMt19937Context);

impl Mt19937 {
    pub fn new(seed: u32) -> Result<Mt19937, ()> { 
        let mut context: SceKernelUtilsMt19937Context = unsafe { mem::zeroed() };
        let ret_val = 
            unsafe {
                sys::sceKernelUtilsMt19937Init(
                    &mut context as *mut SceKernelUtilsMt19937Context,
                    seed
                )
            };
        if ret_val >= 0 {
            Ok(Mt19937(context))
        } else {
            Err(())
        }
    }

    pub fn random(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(4) { 
            chunk.copy_from_slice(&self.random_u32().to_le_bytes()[..chunk.len()]); 
        }    
    }


    pub fn random_u8(&mut self) -> u8 {
        self.random_u32() as u8
    }

    pub fn random_u16(&mut self) -> u16 {
        self.random_u32() as u16
    }

    pub fn random_u32(&mut self) -> u32 {
        unsafe { 
            sys::sceKernelUtilsMt19937UInt(
                &mut self.0 as *mut SceKernelUtilsMt19937Context
            )
        }
    }

    pub fn random_u64(&mut self) -> u64 {
        let lower_half = self.random_u32(); 
        let upper_half = self.random_u32(); 
        ((upper_half as u64) << 32) | lower_half as u64
    }
}

impl RngCore for Mt19937 {
    fn next_u32(&mut self) -> u32 {
        self.random_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.random_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.random(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}
