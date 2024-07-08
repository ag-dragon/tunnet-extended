use crate::{
    DIG_TEXT,
    DIG_OPTIONS,
    BUILD_TEXT,
    BUILDABLES,
    settings::SETTINGS,
};

use windows::Win32::System::{
    Threading::GetCurrentProcess,
    Diagnostics::Debug::WriteProcessMemory,
    Memory::{ VirtualProtect, PAGE_PROTECTION_FLAGS },
};

use std::ffi::c_void;

pub fn enable_drill_anything(base_address: u64) {
    unsafe {
        let process = GetCurrentProcess();
        
        let buffer: [u8; 2] = [0x90, 0x90];   // just nops
        let _ = WriteProcessMemory(process, (base_address + 0x355A48) as *const c_void, buffer.as_ptr().cast(), 2, None);
        
        let new_text = DIG_OPTIONS.iter().nth(1).unwrap();
        for (i, b) in new_text.bytes().enumerate() {
            DIG_TEXT[i] = b;
        }
        
        let mut old = Vec::with_capacity(20);
        let _ = VirtualProtect((base_address+0x2478D00) as *const c_void, 0x2000, PAGE_PROTECTION_FLAGS(0x04), old.as_mut_ptr() as *mut PAGE_PROTECTION_FLAGS);
        
        let replacement_length: [u8; 1] = [new_text.len() as u8];
        let _ = WriteProcessMemory(process, (base_address + 0x2478D28) as *const c_void, replacement_length.as_ptr().cast(), 1, None);
        
        let _ = VirtualProtect((base_address+0x2478D00) as *const c_void, 0x2000, PAGE_PROTECTION_FLAGS(0x02), old.as_mut_ptr() as *mut PAGE_PROTECTION_FLAGS);
    };
}

pub fn disable_drill_anything(base_address: u64) {
    unsafe {
        let process = GetCurrentProcess();
        
        let buffer: [u8; 2] = [0x73, 0x26];   // original jnc opcode
        let _ = WriteProcessMemory(process, (base_address + 0x355A48) as *const c_void, buffer.as_ptr().cast(), 2, None);
        
        let new_text = DIG_OPTIONS.iter().nth(0).unwrap();
        for (i, b) in new_text.bytes().enumerate() {
            DIG_TEXT[i] = b;
        }
        
        let mut old = Vec::with_capacity(20);
        let _ = VirtualProtect((base_address+0x2478D00) as *const c_void, 0x2000, PAGE_PROTECTION_FLAGS(0x04), old.as_mut_ptr() as *mut PAGE_PROTECTION_FLAGS);
        
        let replacement_length: [u8; 1] = [new_text.len() as u8];
        let _ = WriteProcessMemory(process, (base_address + 0x2478D28) as *const c_void, replacement_length.as_ptr().cast(), 1, None);
        
        let _ = VirtualProtect((base_address+0x2478D00) as *const c_void, 0x2000, PAGE_PROTECTION_FLAGS(0x02), old.as_mut_ptr() as *mut PAGE_PROTECTION_FLAGS);
    };
}

pub fn set_drill_material(base_address: u64, material: u8) {
    unsafe {
        let process = GetCurrentProcess();
        
        let buffer: [u8; 5] = [0xba, material, 0x00, 0x00, 0x00];     // teraform material swap
        let _ = WriteProcessMemory(process, (base_address + 0x355A53) as *const c_void, buffer.as_ptr().cast(), 5, None);
        
        let mut old = Vec::with_capacity(20);
        let _ = VirtualProtect((base_address+0x2478D70) as *const c_void, 0x2000, PAGE_PROTECTION_FLAGS(0x04), old.as_mut_ptr() as *mut PAGE_PROTECTION_FLAGS);
        
        let buildable = BUILDABLES.iter().nth(material as usize).unwrap();
        for (i, b) in buildable.bytes().enumerate() {
            BUILD_TEXT[i+11] = b;
        }
        
        let replacement_length: [u8; 1] = [10 + buildable.len() as u8];
        let _ = WriteProcessMemory(process, (base_address + 0x2478D98) as *const c_void, replacement_length.as_ptr().cast(), 1, None);
        
        let _ = VirtualProtect((base_address+0x2478D70) as *const c_void, 0x2000, PAGE_PROTECTION_FLAGS(0x02), old.as_mut_ptr() as *mut PAGE_PROTECTION_FLAGS);
    };
}

pub fn set_all_light(base_address: u64, light: bool) {
    unsafe {
        let process = GetCurrentProcess();
        
        if light {
            let lbuf1: [u8; 2] = [0x90, 0x90];
            let lbuf2: [u8; 4] = [0x48, 0x83, 0xfa, 0x00];
            
            let _ = WriteProcessMemory(process, (base_address + 0x292D33) as *const c_void, lbuf1.as_ptr().cast(), 2, None);
            let _ = WriteProcessMemory(process, (base_address + 0x292D80) as *const c_void, lbuf2.as_ptr().cast(), 4, None);
        } else {
            let lbuf1: [u8; 2] = [0x74, 0x29];
            let lbuf2: [u8; 4] = [0x48, 0x83, 0xfa, 0x05];
            
            let _ = WriteProcessMemory(process, (base_address + 0x292D33) as *const c_void, lbuf1.as_ptr().cast(), 2, None);
            let _ = WriteProcessMemory(process, (base_address + 0x292D80) as *const c_void, lbuf2.as_ptr().cast(), 4, None);
        }
    };
}

pub fn init_patches(base_address: u64) {
    unsafe {
        let process = GetCurrentProcess();
        
        if SETTINGS.read().unwrap().get::<bool>("patches.infinite_stamina").unwrap() {
            let buffer: [u8; 4] = [0x90, 0x90, 0x90, 0x90];          // no sprint stamina loss
            let _ = WriteProcessMemory(process, (base_address + 0x6DB739) as *const c_void, buffer.as_ptr().cast(), 4, None);
        }
        if SETTINGS.read().unwrap().get::<bool>("patches.build_in_rooms").unwrap() {
            let buffer: [u8; 5] = [0xe9, 0x1e, 0xfe, 0xff, 0xff];    // build in rooms
            let _ = WriteProcessMemory(process, (base_address + 0x18097D) as *const c_void, buffer.as_ptr().cast(), 5, None);
        }
        if SETTINGS.read().unwrap().get::<bool>("patches.dig_rock_fast").unwrap() {
            let buffer: [u8; 5] = [0xba, 0x00, 0x00, 0x00, 0x00];    // dig rock instantly
            let _ = WriteProcessMemory(process, (base_address + 0x355A5B) as *const c_void, buffer.as_ptr().cast(), 5, None);
        }
        // let buffer_: [u8; 2] = [0x90, 0x90];                   // stop money counter from changing
        // let _ = WriteProcessMemory(process, (base_address + 0x74FC60) as *const c_void, buffer_.as_ptr().cast(), 2, None);
    };
}