#[allow(unused_imports)]
use crate::{ DIG_TEXT, BUILD_TEXT, NEW_LINE, TEST_STRING };

use ilhook::x64::{ Hooker, HookType, Registers, CallbackOption, HookFlags };

use std::ptr::addr_of;
use std::mem::forget;

struct StringOffsets;

#[cfg(target_os = "windows")]
#[cfg(Steam)]
impl StringOffsets {
    const LABEL_HOOK: u64 = 0x239ED5E; // annoying to port, have to manually check with cheat engine rather than just searching byte pattern
    const DRILL_PATCH: u64 = 0x246E390;
    const DRILL_DIG: u64 = 0x246E320;
}

#[cfg(target_os = "windows")]
#[cfg(Itchio)]
impl StringOffsets {
    const LABEL_HOOK: u64 = 0x228996E;
    const DRILL_PATCH: u64 = 0x2322D90;
    const DRILL_DIG: u64 = 0x2322D20; // 0x2328F20
}

#[cfg(target_os = "linux")]
#[cfg(Itchio)]
impl StringOffsets {
    const LABEL_HOOK: u64 = 0x1B82670;
    const DRILL_PATCH: u64 = 0x1BD26F9;
    const DRILL_DIG: u64 = 0x1BD26EF;
}

#[cfg(target_os = "linux")]
#[cfg(Steam)]
impl StringOffsets {
    const LABEL_HOOK: u64 = 0x1C3D870;
    const DRILL_PATCH: u64 = 0x1C8DC41;
    const DRILL_DIG: u64 = 0x1C8DC37;
}

// gets called when rendering label. Check if it is rendering certain text, then redirect pointer to our own string
#[cfg(target_os = "linux")]
unsafe extern "win64" fn label_hook(reg: *mut Registers, base_address: usize) {
    if (*reg).rsi == base_address as u64 + StringOffsets::DRILL_PATCH { // if "to patch"
        let address = (addr_of!(BUILD_TEXT) as *const u8) as u64;
        (*reg).rsi = address;
    } else if (*reg).rsi == base_address as u64 + StringOffsets::DRILL_PATCH + 0x1 {
        let address = (addr_of!(BUILD_TEXT) as *const u8) as u64;
        (*reg).rsi = address + 0x1;
    } else if (*reg).rsi == base_address as u64 + StringOffsets::DRILL_DIG { // if "to dig"\
        let address = (addr_of!(DIG_TEXT) as *const u8) as u64;
        (*reg).rsi = address;
    }
}

#[cfg(target_os = "windows")]
unsafe extern "win64" fn label_hook(reg: *mut Registers, base_address: usize) {
    if (*reg).rdx == base_address as u64 + StringOffsets::DRILL_PATCH + 0x1 {
        let address = (addr_of!(BUILD_TEXT) as *const u8) as u64;
        (*reg).rdx = address + 0x1;
    } else if (*reg).rdx == base_address as u64 + StringOffsets::DRILL_DIG { // if "to dig"\
        let address = (addr_of!(DIG_TEXT) as *const u8) as u64;
        (*reg).rdx = address;
    }
}

pub fn hook(base_address: u64) {
    let hooker = Hooker::new((base_address+StringOffsets::LABEL_HOOK).try_into().unwrap(), HookType::JmpBack(label_hook), CallbackOption::None, base_address as usize, HookFlags::empty());
    
    unsafe {
        // The hooker drop function removes the hook, so forget is used to prevent it from being called
        forget(hooker.hook().unwrap());
    };
}
