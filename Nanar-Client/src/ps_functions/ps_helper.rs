use ntapi::{
    ntpsapi::{NtSuspendProcess, NtResumeProcess},
    winapi::ctypes::c_void
};
use windows::{
    Win32::{
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS
            },
            Threading::{OpenProcess, PROCESS_SUSPEND_RESUME},
        },
        Foundation::CloseHandle,
    }
};

pub fn get_child_processes(parent_pid: u32) -> windows::core::Result<Vec<u32>>{

    unsafe {

        let snapshot: windows::Win32::Foundation::HANDLE = CreateToolhelp32Snapshot(
            TH32CS_SNAPPROCESS, 0)?;
        let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();

        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        let mut child_pids: Vec<u32> = Vec::new();

        if Process32First(snapshot, &mut process_entry).is_ok() {
            loop {
                if process_entry.th32ParentProcessID == parent_pid {
                    child_pids.push(process_entry.th32ProcessID);
                }

                if Process32Next(snapshot, &mut process_entry).is_err() {
                    break;
                }
            }
        }

        CloseHandle(snapshot)?;
        
        Ok(child_pids)
    }
}

pub fn suspend_process(pid: u32) -> windows::core::Result<()> {
    
    unsafe {
        let handle:windows::Win32::Foundation::HANDLE = OpenProcess(
            PROCESS_SUSPEND_RESUME, false, pid)?;
        
        let status: i32 = NtSuspendProcess(handle.0 as *mut c_void);
        
        CloseHandle(handle)?;

        if status == 0 {
            Ok(())
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

pub fn resume_process(pid: u32) -> windows::core::Result<()> {
    
    unsafe {
        let handle: windows::Win32::Foundation::HANDLE = OpenProcess(
            PROCESS_SUSPEND_RESUME, false, pid)?;
        
        let status: i32 = NtResumeProcess(handle.0 as *mut c_void);
        
        CloseHandle(handle)?;

        if status == 0 {
            Ok(())
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}