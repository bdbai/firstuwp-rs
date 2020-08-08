#![windows_subsystem = "windows"]
mod abi;
mod app;
mod interop;
mod main_page;
mod weak_ref;
mod xaml_metadata_provider;
mod xaml_system_base_type;
mod xaml_user_type;

use interop::init_apartment;
use winrt::*;

fn start_app() -> Result<()> {
    use bindings::windows::ui::xaml::*;
    init_apartment()?;
    Application::start(ApplicationInitializationCallback::new(|_| {
        app::App::new()?;
        Ok(())
    }))
}

fn main() -> Result<()> {
    start_app()
}

#[no_mangle]
extern "system" fn wWinMain(
    _h_instance: *const i32,
    _h_prev_instance: *const i32,
    _cmd_line: *const u16,
    _n_cmd_show: i32,
) -> i32 {
    match start_app() {
        Ok(()) => 0,
        Err(e) => e.code().0 as i32,
    }
}
