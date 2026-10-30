//! Loads a copy of the DLL into this non-game host to exercise the refusal path.
use std::{path::PathBuf, time::Duration};
use windows::{Win32::System::LibraryLoader::LoadLibraryW, core::PCWSTR};
fn main() {
    let path = PathBuf::from(std::env::args_os().nth(1).expect("DLL path required"))
        .canonicalize()
        .unwrap();
    use std::os::windows::ffi::OsStrExt;
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    unsafe { LoadLibraryW(PCWSTR(wide.as_ptr())) }.expect("LoadLibraryW failed");
    let log = path.parent().unwrap().join("SpiritAshResummon.log");
    for _ in 0..40 {
        if std::fs::read_to_string(&log)
            .is_ok_and(|s| s.contains("host is not eldenring.exe; no game memory accessed"))
        {
            println!("PASS: DLL loaded and rejected non-game host without applying hooks");
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!("Expected refusal diagnostic was not produced");
}
