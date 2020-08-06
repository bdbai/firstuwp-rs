#[path = "src/abi.rs"]
mod abi;
#[path = "src/main_page.rs"]
mod main_page;
#[path = "src/xaml_metadata_provider.rs"]
mod xaml_metadata_provider;
#[path = "src/xaml_system_base_type.rs"]
mod xaml_system_base_type;
#[path = "src/xaml_user_type.rs"]
mod xaml_user_type;

use bindings::windows::storage::StorageFolder;
use bindings::windows::ui::xaml::markup::XamlBinaryWriter;
use std::env::current_dir;
use winrt::*;

fn main() {
    /*fn run() -> Result<()> {
        let mut src_dir = current_dir().unwrap();
        src_dir.push("src");
        let src_dir = src_dir.to_string_lossy().to_string();
        let folder = StorageFolder::get_folder_from_path_async(src_dir)?.get()?;
        let input_streams = vec![];
        let output_streams = vec![];
        for file in folder
            .get_files_async_overload_default_options_start_and_count()?
            .get()?
        {
            input_streams.push(file.open_read_async()?.get()?);
            eprintln!("{}", file.path()?.to_string());
        }
        //XamlBinaryWriter::write(todo!(), todo!(), todo!())?;
        Ok(())
    }*/
    //run().unwrap();
}
