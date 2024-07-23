mod settings;
mod hooks;
mod patches;

use settings::{ SETTINGS, str_to_key };

use device_query::{ DeviceQuery, DeviceState, Keycode };

#[cfg(target_os = "windows")]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::{
        System::SystemServices::*,
        System::LibraryLoader::GetModuleHandleA,
    },
};

#[cfg(target_os = "linux")]
use ctor::ctor;

use std::thread;

static TEST_STRING: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

static mut DIG_TEXT: [u8; 20] = [0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x67, 0x20, 0x28, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const DIG_OPTIONS: [&str; 2] = [
    " to dig ($\0",
    " to dig anything ($\0",
];

#[allow(dead_code)]
static NEW_LINE: [u8; 1] = [0x0A];

static mut BUILD_TEXT: [u8; 41] = [0x0A, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x6b, 0x73, 0x20, 0x28, 0x24, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const BUILDABLES: [&str; 17] = [
    "air ($\0",
    "dirt ($\0",
    "unbreakable dirt ($\0",
    "grass ($\0",
    "unbreakable grass ($\0",
    "unbreakable rock ($\0",
    "metal wall ($\0",
    "tiles ($\0",
    "metal sheets ($\0",
    "gray brick ($\0",
    "terraform material ($\0",
    "cobble ($\0",
    "alt cobble ($\0",
    "flower glyph ($\0",
    "wood planks ($\0",
    "alt tiles ($\0",
    "corrupted metal wall ($\0",
];


fn input_loop(base_address: u64) {
    let mat_down_key = SETTINGS.read().unwrap().get::<String>("keybinds.material_down").unwrap();
    let mat_up_key = SETTINGS.read().unwrap().get::<String>("keybinds.material_up").unwrap();
    let dig_anywhere_key = SETTINGS.read().unwrap().get::<String>("keybinds.dig_anywhere_toggle").unwrap();
    let light_key = SETTINGS.read().unwrap().get::<String>("keybinds.force_light").unwrap();
    
    let mut material: u8 = 3;
    let device_state = DeviceState::new();
    let mut last_right_bracket = false;
    let mut last_left_bracket = false;
    let mut last_l = false;
    let mut last_k = false;
    let mut drill_anything = false;
    
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        
        if keys.contains(&str_to_key(&mat_up_key).unwrap()) {
            if !last_right_bracket {
                if material < BUILDABLES.len() as u8 - 1 {
                    material += 1;
                }
                patches::set_drill_material(base_address, material);
            }
            last_right_bracket = true;
        } else {
            last_right_bracket = false;
        }
        
        if keys.contains(&str_to_key(&mat_down_key).unwrap()) {
            if !last_left_bracket {
                if material > 0 {
                    material -= 1;
                }
                patches::set_drill_material(base_address, material);
            }
            last_left_bracket = true;
        } else {
            last_left_bracket = false;
        }
        
        if keys.contains(&str_to_key(&light_key).unwrap()) {
            if !last_l {
                patches::set_all_light(base_address, !last_l);
            }
            last_l = true;
        } else {
            if last_l {
                patches::set_all_light(base_address, !last_l);
            }
            last_l = false;
        }
        
        if keys.contains(&str_to_key(&dig_anywhere_key).unwrap()) {
            if !last_k {
                if drill_anything {
                    drill_anything = false;
                    patches::disable_drill_anything(base_address);
                } else {
                    drill_anything = true;
                    patches::enable_drill_anything(base_address);
                }
            }
            last_k = true;
        } else {
            last_k = false;
        }
    }
}

#[cfg(target_os = "windows")]
fn attach() {
    let base_address = unsafe {
        let module = GetModuleHandleA(PCSTR("tunnet.exe".as_ptr()));
        
        module.unwrap().0 as u64
    };
    
    patches::set_drill_material(base_address, 3); // idk why but the first time this gets called the string length doesnt get set properly
    patches::set_drill_material(base_address, 3); // so I just call it twice at first (after this it works fine)
    patches::init_patches(base_address);
    
    hooks::hook(base_address);
    
    thread::spawn(move || {
        input_loop(base_address);
    });
}

#[cfg(target_os = "windows")]
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        _ => ()
    }
    
    true
}

#[cfg(target_os = "linux")]
#[ctor]
fn entry_point() {
    use procfs::process::Process;
    let base_address = Process::new(std::process::id() as i32).unwrap().maps().unwrap().into_iter().nth(0).unwrap().address.0;

    patches::set_drill_material(base_address, 3);
    patches::init_patches(base_address);

    hooks::hook(base_address);

    thread::spawn(move || {
        input_loop(base_address);
    });
}
