mod process;
use process::*;


use std::mem;
use std::ptr::null_mut;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use sysinfo::System;
use windows::core::PCSTR;
use windows::Win32::{
    Foundation::{
        HWND,
        HANDLE,
        RECT,
    },
    System::{
        Diagnostics::{
            Debug::{
                ReadProcessMemory,
                WriteProcessMemory,
            }
        },
        Threading::{
            GetProcessId,
            OpenProcess,
            PROCESS_ACCESS_RIGHTS,
            PROCESS_ALL_ACCESS,
        }
    },
    UI::WindowsAndMessaging::{
        GetWindowRect,
        GetForegroundWindow,
        FindWindowA,
        IsWindowVisible,
        IsIconic,
    },
    Graphics::Dwm::{
        DwmGetWindowAttribute,
        DWMWA_CLOAKED,
    }
};

// SAFETY: HANDLE is thread-safe as it's just an identifier
// and base_address is only used for reading
// Not sure why this is necessary though.
unsafe impl Send for Process {}
unsafe impl Sync for Process {}

#[derive(Clone, Debug)]
pub struct Process {
    name: String,
    pid: u32,
    handle: HANDLE,
    window_handle: HWND,
    //is_focused: Arc<AtomicBool>,
    pub(crate) base_address: u32,
}

impl Process {
    pub fn new(process_name: impl ToString + std::fmt::Display) -> Self {
        let name = process_name.to_string();

        let pid = get_pid_by_name(&name)
            .unwrap_or_else(|| panic!("Could not get pid!"));

        log::debug!("Got pid! - {}", &pid);

        let handle = unsafe {
            match open_process_handle(pid) {
                Ok(handle) => handle,
                Err(error) => {
                    panic!("Failed to open handle for {}. Error: {}", name, error)
                }
            }
        };

        let window_handle = unsafe {
            // as_ptr() avoids allocating strings
            FindWindowA(
                PCSTR::from_raw("SDL_app\0".as_ptr()),
                PCSTR::from_raw("AssaultCube\0".as_ptr()),
            ).expect("Failed to find assault cube window")
        };

        log::debug!("Got handle! - {:?}", &handle);

        let base_address = unsafe {
            match get_mod_base(pid, &name) {
                Ok(mod_base) => {
                    if mod_base.is_null() {
                        panic!("Could not find module base for {}", name);
                    }
                    mod_base as u32  // Return the base address if not null
                },
                Err(error) => {
                    panic!("Failed to get module base for {}. Error: {}", name, error)
                },
            }
        };

        log::debug!("Got base address! - {:?}", base_address);

        Self {
            name,
            pid,
            handle,
            window_handle,
            base_address
        }
    }

    /// Generic wrapper that uses try_read_bytes_into under the hood
    pub fn read<T>(&self, address: u32) -> Option<T>
    where T: Copy {
        unsafe {
            // Allocate zeroed memory
            let mut buffer = std::mem::zeroed::<T>();

            let buffer_slice = std::slice::from_raw_parts_mut(
                &mut buffer as *mut T as *mut u8,
                std::mem::size_of::<T>()
            );

            self.try_read_bytes_into(address, buffer_slice)?;
            Some(buffer)
        }
    }

    pub fn write<T>(&self, address: u32, value: T) -> Option<()>
    where T: Copy {
        let buffer = unsafe {
            std::slice::from_raw_parts(
                // Convert &T to *const T (raw pointer) then to *const u8 (byte pointer)
                &value as *const T as *const u8,
                // Get the size of T in bytes
                std::mem::size_of::<T>()
            )
        };

        self.try_write_bytes(address, buffer)
    }

    // Original function that does the actual reading
    fn try_read_bytes_into(&self, address: u32, buffer: &mut [u8]) -> Option<()> {
        if buffer.len() == 0 {
            return Some(());
        }
        let status = unsafe {
            ReadProcessMemory(
                self.handle,
                address as _,
                buffer.as_mut_ptr() as _,
                mem::size_of_val(buffer) as _,
                None,
            )
        };

        match status {
            Ok(_) => Some(()),
            Err(error) => {
                log::error!("ReadProcessMemory failed: {}", error);
                None
            }
        }
    }

    fn try_write_bytes(&self, address: u32, buffer: &[u8]) -> Option<()> {
        if buffer.len() == 0 {
            return Some(());
        }

        let status = unsafe {
            WriteProcessMemory(
                self.handle,
                address as _,
                buffer.as_ptr() as _,
                mem::size_of_val(buffer) as _,
                None,
            )
        };

        match status {
            Ok(_) => Some(()),
            Err(error) => {
                log::error!("WriteProcessMemory failed: {}", error);
                None
            }
        }
    }

    pub fn get_screen_bounds(&self) -> Option<glam::Vec2> {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };

        unsafe {
            match GetWindowRect(self.window_handle, &mut rect) {
                Ok(_) => {
                    // Calculate width and height from rect coordinates
                    let width = (rect.right - rect.left) as f32;
                    let height = (rect.bottom - rect.top) as f32;
                    Some(glam::Vec2::new(width, height))
                },
                Err(error) => {
                    log::error!("Failed to get window rect: {}", error);
                    None
                }
            }
        }
    }

    pub fn is_focused(&self) -> bool {
        unsafe {
            // Get the currently focused window and with our window handle
            GetForegroundWindow() == self.window_handle
        }
    }

    /// This is not functional. I want to figure out how to make it functional though.
    pub fn is_visible(&self) -> bool {
        unsafe {
            // First check if window is marked as visible
            if !IsWindowVisible(self.window_handle).as_bool() {
                return false;
            }

            // Check if window is minimized
            if IsIconic(self.window_handle).as_bool() {
                return false;
            }

            // Check if window is cloaked (occluded/hidden by Windows)
            let mut cloaked: u32 = 0;
            if DwmGetWindowAttribute(
                self.window_handle,
                DWMWA_CLOAKED,
                &mut cloaked as *mut _ as *mut _,
                size_of::<u32>() as u32,
            ).is_ok() {
                if cloaked != 0 {
                    return false;
                }
            }

            true
        }
    }
}
