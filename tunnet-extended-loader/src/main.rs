#[cfg(target_os = "windows")]
use dll_syringe::{Syringe, process::OwnedProcess};
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::GetProcessTimes;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{FILETIME, HANDLE};
#[cfg(target_os = "windows")]
use std::os::windows::io::AsRawHandle;

#[cfg(target_os = "linux")]
use ptrace_inject::{Injector, Process};

use std::path::PathBuf;
use std::process::Command;
use std::{thread, time};

#[cfg(target_os = "windows")]
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

fn main() {
    #[cfg(target_os = "windows")]
    thread::spawn(|| {
        let _ = Command::new("tunnet.exe").output().expect("failed to launch Tunnet");
    });

    #[cfg(target_os = "linux")]
    thread::spawn(|| {
        let _ = Command::new("./tunnet").arg("--bypass-launcher").output().expect("failed to launch Tunnet");
    });
    
    thread::sleep(time::Duration::from_millis(1000));

    #[cfg(target_os = "windows")]
    {
        let tunnet_process = get_game_process();

        let syringe = Syringe::for_process(tunnet_process);
        
        let _ = syringe.inject("tunnet_extended.dll").unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        let library = PathBuf::from("target/debug/libtunnet_extended.so");
        let proc = Process::by_name("tunnet").unwrap().unwrap();
        let _ = Injector::attach(proc).unwrap().inject(&library);
    }
}
