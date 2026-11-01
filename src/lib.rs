#![allow(non_snake_case)]
#[cfg(all(windows, not(feature = "seamless-probe")))]
mod capacity;
pub mod config;
pub mod groups;
#[cfg(all(windows, not(feature = "seamless-probe")))]
mod hooks;
pub mod hud_names;
pub mod mend_policy;
pub mod policy;
#[cfg(all(windows, not(feature = "seamless-probe")))]
mod runtime;
#[cfg(all(windows, not(feature = "seamless-probe")))]
mod seamless_hooks;
pub mod session_policy;
#[cfg(all(windows, not(feature = "seamless-probe")))]
mod threads;
#[cfg(all(windows, not(feature = "seamless-probe")))]
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
        std::thread::spawn(move || {
            #[cfg(not(feature = "seamless-probe"))]
            runtime::start(module);
            #[cfg(feature = "seamless-probe")]
            seamless_probe::start(module);
        });
    }
    1
}

#[cfg(all(windows, not(feature = "seamless-probe")))]
mod hud;
#[cfg(all(windows, not(feature = "seamless-probe")))]
mod hud_movie;

#[cfg(all(windows, feature = "seamless-probe"))]
mod seamless_probe;
