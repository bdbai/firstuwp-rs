#[link(name = "windowsapp")]
#[allow(unused)]
extern "system" {
    pub fn RoInitialize(init_type: RoInitType) -> winrt::ErrorCode;
    pub fn RoUninitialize();
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
