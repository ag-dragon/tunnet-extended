use dll_syringe::{Syringe, process::OwnedProcess};
use windows::Win32::System::Threading::GetProcessTimes;
use windows::Win32::Foundation::{FILETIME, HANDLE};
use std::os::windows::io::AsRawHandle;

fn get_game_process() -> OwnedProcess {
    let mut processes = OwnedProcess::find_all_by_name("tunnet.exe");
    let mut highest_creation_time: u32 = 0;
    let mut output_index = 0;
    
    for (i, process) in processes.iter().enumerate() {
        let mut creation_time: FILETIME = Default::default();
        let mut exit_time: FILETIME = Default::default();
        let mut kernel_time: FILETIME = Default::default();
        let mut user_time: FILETIME = Default::default();
        
        let handle: HANDLE = HANDLE(process.as_raw_handle() as isize);
        
        unsafe {
            let _ = GetProcessTimes(handle, &mut creation_time, &mut exit_time, &mut kernel_time, &mut user_time);
        };
        
        if creation_time.dwLowDateTime > highest_creation_time {
            highest_creation_time = creation_time.dwLowDateTime;
            output_index = i;
        }
    }
    
    processes.remove(output_index)
}

use std::process::Command;
use std::{thread, time};

fn main() {
        
    thread::spawn(|| {
        let _ = Command::new("tunnet.exe").output().expect("failed to launch Tunnet");
    });
    
    thread::sleep(time::Duration::from_millis(1000));
    
    let tunnet_process = get_game_process();
    
    let syringe = Syringe::for_process(tunnet_process);
    
    let _ = syringe.inject("tunnet_extended.dll").unwrap();
}
