use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

#[link(name = "windowsapp")]
#[allow(unused)]
extern "system" {
    fn RoInitialize(init_type: RoInitType) -> winrt::ErrorCode;
    fn RoUninitialize();
}

#[link(name = "Kernel32")]
extern "system" {
    fn OutputDebugStringW(lp_output_string: *const u16);
}

#[allow(unused)]
#[repr(i32)]
pub enum RoInitType {
    SingleThreaded = 0,
    MultiThreaded = 1,
}

pub fn init_apartment() -> winrt::Result<()> {
    unsafe { RoInitialize(RoInitType::MultiThreaded) }.ok()
}

#[allow(unused)]
pub fn debug_log(log: impl AsRef<OsStr>) {
    let bytes: Vec<u16> = log.as_ref().encode_wide().collect();
    unsafe { OutputDebugStringW(bytes.as_ptr()) };
}
