use std::ffi::c_void;
use std::string::FromUtf16Error;
use std::time::Duration;
use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Memory::{
    MapViewOfFile, OpenFileMappingA, UnmapViewOfFile, FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS,
};

#[derive(Debug)]
pub struct SafeHandle {
    inner: HANDLE,
}

impl SafeHandle {
    pub fn new(inner: HANDLE) -> Option<Self> {
        if inner.is_invalid() {
            None
        } else {
            Some(Self { inner })
        }
    }

    pub unsafe fn get(&self) -> HANDLE {
        self.inner
    }
}

impl Drop for SafeHandle {
    fn drop(&mut self) {
        unsafe {
            // Can't really do much here if there's an error, let's ignore it.
            let _ = CloseHandle(self.inner);
        }
    }
}

#[derive(Debug)]
pub struct SafeFileView {
    pub inner: MEMORY_MAPPED_VIEW_ADDRESS,
}

unsafe impl Send for SafeFileView {}
unsafe impl Sync for SafeFileView {}

impl SafeFileView {
    pub fn new(inner: MEMORY_MAPPED_VIEW_ADDRESS) -> Option<Self> {
        if inner.Value.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }

    pub unsafe fn get(&self) -> *const c_void {
        self.inner.Value
    }
}

impl Drop for SafeFileView {
    fn drop(&mut self) {
        unsafe {
            // Can't really do much here if there's an error, let's ignore it.
            let _ = UnmapViewOfFile(self.inner);
        }
    }
}

pub struct SharedMemory {
    _handle: SafeHandle,
    file_view: SafeFileView,
}

impl SharedMemory {
    pub async fn connect(name: &[u8], poll_delay: Duration) -> Self {
        let handle;
        loop {
            {
                let handle_opt = unsafe {
                    OpenFileMappingA(FILE_MAP_READ.0, false, PCSTR::from_raw(name.as_ptr()))
                }
                .ok()
                .and_then(SafeHandle::new);
                if let Some(val) = handle_opt {
                    handle = val;
                    break;
                }
            }
            tokio::time::sleep(poll_delay).await;
        }

        let file_view;
        loop {
            {
                let file_view_opt = SafeFileView::new(unsafe {
                    MapViewOfFile(handle.get(), FILE_MAP_READ, 0, 0, 0)
                });
                if let Some(val) = file_view_opt {
                    file_view = val;
                    break;
                }
            }
            tokio::time::sleep(poll_delay).await;
        }

        Self {
            _handle: handle,
            file_view,
        }
    }

    pub unsafe fn get(&self) -> *const c_void {
        self.file_view.get()
    }

    pub unsafe fn get_as<T>(&self) -> &T {
        &(*(self.get() as *const T))
    }

    pub unsafe fn copy_as<T: Copy>(&self) -> T {
        *(self.get() as *const T)
    }
}
