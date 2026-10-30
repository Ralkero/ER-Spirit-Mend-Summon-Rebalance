#![allow(non_snake_case)]
#[cfg(windows)]
mod capacity;
pub mod config;
#[cfg(windows)]
mod hooks;
pub mod policy;
#[cfg(windows)]
mod runtime;
#[cfg(windows)]
mod threads;
#[cfg(windows)]
mod timing;

#[cfg(windows)]
#[unsafe(no_mangle)]
/// # Safety
/// Windows loader entry point. Do not manually invoke or unload the active DLL.
pub unsafe extern "system" fn DllMain(
    module: windows::Win32::Foundation::HINSTANCE,
    reason: u32,
    _: *mut std::ffi::c_void,
) -> i32 {
    if reason == 1 {
        let module = module.0 as usize;
        // No file I/O, game API calls, hook installation or joins under loader lock.
        std::thread::spawn(move || runtime::start(module));
    }
    1
}

#[cfg(windows)]
mod hud;
#[cfg(windows)]
mod hud_movie;
