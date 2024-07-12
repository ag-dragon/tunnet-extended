use crate::{ DIG_TEXT, BUILD_TEXT };

use ilhook::x64::{ Hooker, HookType, Registers, CallbackOption, HookFlags };

use std::ptr::addr_of;
use std::mem::forget;

enum StringAddresses {
    DrillPatch = if cfg!(Steam) { // "to patch"
        0x2478D71
    } else {
        0x2338f20
    },
    DrillDig = if cfg!(Steam) {   // "to dig"
        0x2478D00
    } else {
        0x2338EB0
    },
}

// gets called when rendering label. Check if it is rendering certain text, then redirect pointer to our own string
unsafe extern "win64" fn label_hook(reg: *mut Registers, base_address: usize) {
    use crate::hooks::StringAddresses::*;
    
    if (*reg).rdx == base_address as u64 + DrillPatch as u64 { // if "to patch"
        let address = (addr_of!(BUILD_TEXT) as *const u8) as u64;
        (*reg).rdx = address + 0x01;
    } else if (*reg).rdx == base_address as u64 + DrillDig as u64 { // if "to dig"\
        let address = (addr_of!(DIG_TEXT) as *const u8) as u64;
        (*reg).rdx = address;
    }
}

pub fn hook(base_address: u64) {
    let hooker = Hooker::new((base_address+0x23A9F9E).try_into().unwrap(), HookType::JmpBack(label_hook), CallbackOption::None, base_address as usize, HookFlags::empty());
    
    unsafe {
        // The hooker drop function removes the hook, so forget is used to prevent it from being called
        forget(hooker.hook().unwrap());
    };
}