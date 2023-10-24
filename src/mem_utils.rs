use std::ffi::CStr;

use windows::Win32::{Foundation::{CloseHandle, HANDLE, BOOL}, System::{Diagnostics::{ToolHelp::{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32, Process32First, Process32Next}, Debug::{ReadProcessMemory, WriteProcessMemory}}, Threading::{OpenProcess, PROCESS_ALL_ACCESS}}};

pub fn safe_close_handle(handle: HANDLE) {
    unsafe {
        if let Err(e) = CloseHandle(handle) {
            eprintln!("Failed to close handle: {:?}", e);
        }
        println!("{:?} closed", handle);
    }
}

pub fn get_pid_by_name(name: &str) -> Option<u32> {
    unsafe {
        let result = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        let h_snap = match result {
            Ok(handle) => handle,
            Err(e) => {
                eprintln!("Error: {}", e);
                return None;
            },
        };

        let mut pe32 = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        if Process32First(h_snap, &mut pe32).is_err() {
            safe_close_handle(h_snap);
            return None;
        }

        loop {
            let process_name_cstr = CStr::from_ptr(pe32.szExeFile.as_ptr() as *const i8);
            if let Ok(process_name) = process_name_cstr.to_str() {
                if process_name == name {
                    safe_close_handle(h_snap);
                    return Some(pe32.th32ProcessID);
                }
            }

            if Process32Next(h_snap, &mut pe32).is_err() {
                break;
            }
        }
        eprintln!("Process not found!");
        safe_close_handle(h_snap);
    }
    None
}

pub fn read_value_from_offsets(process_id: u32, base_address: u32, offsets: &[u32]) -> windows::core::Result<u32> {
    unsafe {
        let pvz_process = OpenProcess(PROCESS_ALL_ACCESS, BOOL(0), process_id)?;

        let mut current_address = base_address;
        for &offset in offsets.iter() {
            let mut temp_pointer: u32 = 0;
            let result = ReadProcessMemory(
                pvz_process,
                current_address as *const _,
                &mut temp_pointer as *mut u32 as _,
                std::mem::size_of::<u32>(),
                Some(std::ptr::null_mut()),
            );

            if result.is_err() || temp_pointer == 0 {
                safe_close_handle(pvz_process);
                return Err(windows::core::Error::from_win32());
            }

            current_address = temp_pointer + offset;
        }

        let mut final_value: u32 = 0;
        let result = ReadProcessMemory(
            pvz_process,
            current_address as *const _,
            &mut final_value as *mut _ as _,
            std::mem::size_of::<u32>(),
            Some(std::ptr::null_mut()),
        );

        safe_close_handle(pvz_process);

        if result.is_ok() {
            Ok(final_value)
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

pub fn write_value_to_offsets(process_id: u32, base_address: u32, offsets: &[u32], value: u32) -> windows::core::Result<()> {
    unsafe {
        let pvz_process = OpenProcess(PROCESS_ALL_ACCESS, BOOL(0), process_id)?;

        let mut current_address = base_address;
        for &offset in offsets.iter() {
            let mut temp_pointer: u32 = 0;
            let result = ReadProcessMemory(
                pvz_process,
                current_address as *const _,
                &mut temp_pointer as *mut u32 as _,
                std::mem::size_of::<u32>(),
                Some(std::ptr::null_mut()),
            );

            if result.is_err() || temp_pointer == 0 {
                safe_close_handle(pvz_process);
                return Err(windows::core::Error::from_win32());
            }

            current_address = temp_pointer + offset;
        }

        let result = WriteProcessMemory(
            pvz_process,
            current_address as *mut _,
            &value as *const u32 as _,
            std::mem::size_of::<u32>(),
            Some(std::ptr::null_mut()),
        );

        safe_close_handle(pvz_process);

        if result.is_ok() {
            Ok(())
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}
