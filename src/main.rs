#![windows_subsystem = "windows"]
mod app;
mod interop;

use interop::init_apartment;
use winrt::*;

import!(
    dependencies
        os
    types
        windows::ui::xaml::{
            Application, ApplicationInitializationCallback,
            IApplicationFactory, IApplicationOverrides, Window, UIElement
        }
        windows::ui::xaml::controls::TextBlock
        windows::application_model::activation::{
            IActivatedEventArgs, LaunchActivatedEventArgs,
            FileActivatedEventArgs, SearchActivatedEventArgs,
            ShareTargetActivatedEventArgs, FileOpenPickerActivatedEventArgs,
            FileSavePickerActivatedEventArgs,
            CachedFileUpdaterActivatedEventArgs, IWindowCreatedEventArgs
        }
);

fn start_app() -> Result<()> {
    use windows::ui::xaml::*;
    init_apartment()?;
    Application::start(ApplicationInitializationCallback::new(|_| {
        /*let mut app = app::App::new();
        let ioverride = app.query::<Object>();
        let mut inner = Default::default();
        let complete_app = winrt::factory::<Application, IApplicationFactory>()?
            .create_instance(ioverride, &mut inner)?;
        app.set_base(inner.query::<IApplicationOverrides>());

        Ok(())*/
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
