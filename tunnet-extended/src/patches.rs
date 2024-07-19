use crate::{
    DIG_TEXT,
    DIG_OPTIONS,
    BUILD_TEXT,
    BUILDABLES,
    settings::SETTINGS,
};

#[cfg(target_os = "windows")]
use windows::Win32::System::{
    Threading::GetCurrentProcess,
    Diagnostics::Debug::WriteProcessMemory,
    Memory::{ VirtualProtect, PAGE_PROTECTION_FLAGS },
};

#[cfg(target_os = "linux")]
use procfs::process::Process;
use std::ffi::c_void;

struct Patches;

#[cfg(target_os = "windows")]
#[cfg(Steam)]
impl Patches {
    const INFINITE_STAMINA_PATCH: [u8; 4] = [0x90, 0x90, 0x90, 0x90];
    const INFINITE_STAMINA_OFFSET: u64 = 0x6DB739;
    
    const BUILD_IN_ROOMS_PATCH: [u8; 5] = [0x39, 0xC0, 0xEB, 0xF6, 0x90];
    const BUILD_IN_ROOMS_OFFSET: u64 =  0x18097D;
    
    const DIG_ROCK_FAST_PATCH: [u8; 5] = [0xba, 0x00, 0x00, 0x00, 0x00];
    const DIG_ROCK_FAST_OFFSET: u64 = 0x355A5B;
    
    const DRILL_ANYTHING_PATCH: [u8; 2] = [0x90, 0x90];
    const DRILL_ANYTHING_UNDO: [u8; 2] = [0x73, 0x26];
    const DRILL_ANYTHING_OFFSET: (u64, u64) = (0x355A48, 0x2478D28);
    
    const DRILL_MATERIAL_OFFSET: (u64, u64) = (0x355A53, 0x2478D98);
    
    const ALL_LIGHT_PATCH: [u8; 2] = [0x90, 0x90];
    const ALL_LIGHT_OFFSET: (u64, u64) = (0x292D33, 0x292D80);
    const ALL_LIGHT_UNDO: [u8; 2] = [0x74, 0x29];
}

#[cfg(target_os = "windows")]
#[cfg(Itchio)]
impl Patches {
    const INFINITE_STAMINA_PATCH: [u8; 4] = [0x90, 0x90, 0x90, 0x90];
    const INFINITE_STAMINA_OFFSET: u64 = 0x6C30B9;
    
    const BUILD_IN_ROOMS_PATCH: [u8; 5] = [0x39, 0xC0, 0xEB, 0xF6, 0x90];
    const BUILD_IN_ROOMS_OFFSET: u64 =  0x44818D;
    
    const DIG_ROCK_FAST_PATCH: [u8; 5] = [0xba, 0x00, 0x00, 0x00, 0x00];
    const DIG_ROCK_FAST_OFFSET: u64 = 0x35570B;
    
    const DRILL_ANYTHING_PATCH: [u8; 2] = [0x90, 0x90];
    const DRILL_ANYTHING_UNDO: [u8; 2] = [0x73, 0x26];
    const DRILL_ANYTHING_OFFSET: (u64, u64) = (0x3556F8, 0x2338ED8);
    
    const DRILL_MATERIAL_OFFSET: (u64, u64) = (0x355703, 0x2338F48);
    
    const ALL_LIGHT_PATCH: [u8; 2] = [0x90, 0x90];
    const ALL_LIGHT_OFFSET: (u64, u64) = (0x2F35F3, 0x2F3640);
    const ALL_LIGHT_UNDO: [u8; 2] = [0x74, 0x29];
}

#[cfg(target_os = "linux")]
#[cfg(Itchio)]
impl Patches {
    const INFINITE_STAMINA_PATCH: [u8; 5] = [0x90, 0x90, 0x90, 0x90, 0x90];
    const INFINITE_STAMINA_OFFSET: u64 = 0x8DBAE7;
    
    const BUILD_IN_ROOMS_PATCH: [u8; 5] = [0xE9, 0x0F, 0xFE, 0xFF, 0xFF];
    const BUILD_IN_ROOMS_OFFSET: u64 =  0x6C361C;
    
    const DIG_ROCK_FAST_PATCH: [u8; 5] = [0xba, 0x00, 0x00, 0x00, 0x00];
    const DIG_ROCK_FAST_OFFSET: u64 = 0x5F8337;
    
    const DRILL_ANYTHING_PATCH: [u8; 2] = [0x90, 0x90];
    const DRILL_ANYTHING_UNDO: [u8; 2] = [0x73, 0x2A];
    const DRILL_ANYTHING_OFFSET: (u64, u64) = (0x5F8324, 0x4D57D80);
    
    const DRILL_MATERIAL_OFFSET: (u64, u64) = (0x5F832F, 0x4d57DE0);
    
    const ALL_LIGHT_PATCH: [u8; 2] = [0x90, 0x90];
    const ALL_LIGHT_OFFSET: (u64, u64) = (0x58ACA4, 0x58ACDE);
    const ALL_LIGHT_UNDO: [u8; 2] = [0x74, 0x22];
}

#[cfg(target_os = "linux")]
#[cfg(Steam)]
impl Patches {
    const INFINITE_STAMINA_PATCH: [u8; 5] = [0x90, 0x90, 0x90, 0x90, 0x90];
    const INFINITE_STAMINA_OFFSET: u64 = 0x906445;
    
    const BUILD_IN_ROOMS_PATCH: [u8; 5] = [0xE9, 0x0E, 0xFE, 0xFF, 0xFF];
    const BUILD_IN_ROOMS_OFFSET: u64 =  0x466E8D;
    
    const DIG_ROCK_FAST_PATCH: [u8; 5] = [0xba, 0x00, 0x00, 0x00, 0x00];
    const DIG_ROCK_FAST_OFFSET: u64 = 0x668EC7;
    
    const DRILL_ANYTHING_PATCH: [u8; 2] = [0x90, 0x90];
    const DRILL_ANYTHING_UNDO: [u8; 2] = [0x73, 0x2A];
    const DRILL_ANYTHING_OFFSET: (u64, u64) = (0x668EB4, 0x4E45058);
    
    const DRILL_MATERIAL_OFFSET: (u64, u64) = (0x668EBF, 0x4E450B8);
    
    const ALL_LIGHT_PATCH: [u8; 2] = [0x90, 0x90];
    const ALL_LIGHT_OFFSET: (u64, u64) = (0x554AB4, 0x554AEE);
    const ALL_LIGHT_UNDO: [u8; 2] = [0x74, 0x22];
}

fn patch<T>(destination: u64, source: *const T, size: usize) {
    #[cfg(target_os = "windows")]
    unsafe {
        let process = GetCurrentProcess();

        let mut old = Vec::with_capacity(20);
        let _ = VirtualProtect((destination) as *const c_void, size, PAGE_PROTECTION_FLAGS(0x40), old.as_mut_ptr() as *mut PAGE_PROTECTION_FLAGS);

        let _ = WriteProcessMemory(process, destination as *const c_void, source.cast(), size, None);

        let mut new = Vec::with_capacity(20);
        let _ = VirtualProtect((destination) as *const c_void, size, old.pop().unwrap(), new.as_mut_ptr() as *mut PAGE_PROTECTION_FLAGS);

    }
    #[cfg(target_os = "linux")]
    unsafe {
        for region in Process::new(std::process::id() as i32).unwrap().maps().unwrap().iter() {
            if region.address.0 < destination && region.address.1 > destination {
                // Note: this assumes that patch doesn't go accross region boundaries
                let old_flags = region.perms;
                libc::mprotect(region.address.0 as *mut c_void, (region.address.1 - region.address.0) as usize, 7);
                std::ptr::copy_nonoverlapping(source, destination as *mut T, size);
                libc::mprotect(region.address.0 as *mut c_void, (region.address.1 - region.address.0) as usize, old_flags.bits() as i32);
            }
        }
    }
}

pub fn enable_drill_anything(base_address: u64) {
    unsafe {
        patch(base_address+Patches::DRILL_ANYTHING_OFFSET.0, Patches::DRILL_ANYTHING_PATCH.as_ptr(), Patches::DRILL_ANYTHING_PATCH.len());
    
        let new_text = DIG_OPTIONS.iter().nth(1).unwrap();
        for (i, b) in new_text.bytes().enumerate() {
            DIG_TEXT[i] = b;
        }

        let replacement_length: u8 = new_text.len() as u8;
        patch(base_address+Patches::DRILL_ANYTHING_OFFSET.1, &replacement_length, 1);
    }
}

pub fn disable_drill_anything(base_address: u64) {
    unsafe {
        patch(base_address+Patches::DRILL_ANYTHING_OFFSET.0, Patches::DRILL_ANYTHING_UNDO.as_ptr(), Patches::DRILL_ANYTHING_UNDO.len());
        
        let new_text = DIG_OPTIONS.iter().nth(0).unwrap();
        for (i, b) in new_text.bytes().enumerate() {
            DIG_TEXT[i] = b;
        }

        let replacement_length: u8 = new_text.len() as u8;
        patch(base_address+Patches::DRILL_ANYTHING_OFFSET.1, &replacement_length, 1);
    };
}

pub fn set_drill_material(base_address: u64, material: u8) {
    unsafe {
        patch(base_address+Patches::DRILL_MATERIAL_OFFSET.0+0x1, &material, 1);

        let buildable = BUILDABLES.iter().nth(material as usize).unwrap();
        for (i, b) in buildable.bytes().enumerate() {
            BUILD_TEXT[i+10] = b;
        }

        let replacement_length: u8 = 10 + buildable.len() as u8;
        patch(base_address+Patches::DRILL_MATERIAL_OFFSET.1, &replacement_length, 1);
    };
}

pub fn set_all_light(base_address: u64, light: bool) {
    if light {
        let new_val: u8 = 0x0;
        patch(base_address+Patches::ALL_LIGHT_OFFSET.0, Patches::ALL_LIGHT_PATCH.as_ptr(), Patches::ALL_LIGHT_PATCH.len());
        patch(base_address+Patches::ALL_LIGHT_OFFSET.1+0x3, &new_val, 1);
    } else {
        let new_val: u8 = 0x5;
        patch(base_address+Patches::ALL_LIGHT_OFFSET.0, Patches::ALL_LIGHT_UNDO.as_ptr(), Patches::ALL_LIGHT_UNDO.len());
        patch(base_address+Patches::ALL_LIGHT_OFFSET.1+0x3, &new_val, 1);
    }
}

pub fn init_patches(base_address: u64) {
    if SETTINGS.read().unwrap().get::<bool>("patches.infinite_stamina").unwrap() {
        patch(base_address+Patches::INFINITE_STAMINA_OFFSET, Patches::INFINITE_STAMINA_PATCH.as_ptr(), Patches::INFINITE_STAMINA_PATCH.len());
    }
    if SETTINGS.read().unwrap().get::<bool>("patches.build_in_rooms").unwrap() {
        patch(base_address+Patches::BUILD_IN_ROOMS_OFFSET, Patches::BUILD_IN_ROOMS_PATCH.as_ptr(), Patches::BUILD_IN_ROOMS_PATCH.len());
    }
    if SETTINGS.read().unwrap().get::<bool>("patches.dig_rock_fast").unwrap() {
        patch(base_address+Patches::DIG_ROCK_FAST_OFFSET, Patches::DIG_ROCK_FAST_PATCH.as_ptr(), Patches::DIG_ROCK_FAST_PATCH.len());
    }
    /*
    // let buffer_: [u8; 2] = [0x90, 0x90];                   // stop money counter from changing
    // let _ = WriteProcessMemory(process, (base_address + 0x74FC60) as *const c_void, buffer_.as_ptr().cast(), 2, None);
    */
}
