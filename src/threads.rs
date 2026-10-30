//! Startup-only patch transaction. Prepare handles before suspension and avoid
//! allocation/logging while other threads may hold the process allocator lock.
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::{
            Debug::{CONTEXT, CONTEXT_CONTROL_AMD64, GetThreadContext},
            ToolHelp::{
                CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First,
                Thread32Next,
            },
        },
        Threading::{
            GetCurrentProcessId, GetCurrentThreadId, OpenThread, ResumeThread, SuspendThread,
            THREAD_GET_CONTEXT, THREAD_SUSPEND_RESUME,
        },
    },
};
pub struct Threads {
    handles: Vec<HANDLE>,
    suspended: usize,
}
impl Threads {
    pub fn prepare() -> Result<Self, String> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) }
            .map_err(|e| format!("Thread inventory: {e}"))?;
        let mut entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        };
        let mut result = Self {
            handles: Vec::new(),
            suspended: 0,
        };
        let process = unsafe { GetCurrentProcessId() };
        let current = unsafe { GetCurrentThreadId() };
        let mut failure = false;
        let mut next = unsafe { Thread32First(snapshot, &mut entry) }.is_ok();
        if !next {
            failure = true;
        }
        while next {
            if entry.th32OwnerProcessID == process && entry.th32ThreadID != current {
                match unsafe {
                    OpenThread(
                        THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT,
                        false,
                        entry.th32ThreadID,
                    )
                } {
                    Ok(handle) => result.handles.push(handle),
                    Err(_) => {
                        failure = true;
                        break;
                    }
                }
            }
            next = unsafe { Thread32Next(snapshot, &mut entry) }.is_ok();
        }
        let _ = unsafe { CloseHandle(snapshot) };
        if failure {
            Err("Cannot open every peer thread; refusing patches".into())
        } else {
            Ok(result)
        }
    }
    pub fn freeze(&mut self, sites: &[(usize, usize)]) -> Result<(), &'static str> {
        for handle in &self.handles {
            if unsafe { SuspendThread(*handle) } == u32::MAX {
                return Err("Cannot suspend every peer thread");
            }
            self.suspended += 1;
        }
        #[repr(align(16))]
        struct Aligned(CONTEXT);
        for handle in &self.handles {
            let mut c = Aligned(CONTEXT::default());
            c.0.ContextFlags = CONTEXT_CONTROL_AMD64;
            if unsafe { GetThreadContext(*handle, &mut c.0) }.is_err() {
                return Err("Cannot verify peer instruction pointers");
            }
            if sites.iter().any(|(site, length)| {
                c.0.Rip as usize > *site && (c.0.Rip as usize) < site + length
            }) {
                return Err("Peer instruction pointer overlaps patch operand");
            }
        }
        Ok(())
    }
    pub fn thaw(&mut self) -> bool {
        let mut ok = true;
        for handle in self.handles[..self.suspended].iter().rev() {
            ok &= unsafe { ResumeThread(*handle) } != u32::MAX;
        }
        self.suspended = 0;
        ok
    }
}
impl Drop for Threads {
    fn drop(&mut self) {
        self.thaw();
        for h in &self.handles {
            let _ = unsafe { CloseHandle(*h) };
        }
    }
}
