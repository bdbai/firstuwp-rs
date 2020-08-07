use std::env::{current_dir, var};
use std::fs::{copy, create_dir, read, read_dir, write};
use std::io::ErrorKind;
use std::path::PathBuf;

fn main() {
    let mut out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    out_dir.pop();
    out_dir.pop();
    out_dir.pop();
    let mut workspace_dir = current_dir().unwrap();
    workspace_dir.pop();

    // Copy XAML .xbf files
    let mut xaml_dir = workspace_dir.clone();
    xaml_dir.push("xaml");
    println!("cargo:rerun-if-changed={}", xaml_dir.display());
    for xaml_path in read_dir(xaml_dir)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .unwrap()
                .to_lowercase()
                .ends_with(".xbf")
        })
    {
        let mut target_path = out_dir.clone();
        target_path.push(xaml_path.file_name().unwrap());
        eprintln!("Copying XAML {:?} to {:?}", xaml_path, target_path);
        println!("cargo:rerun-if-changed={}", xaml_path.display());
        copy(xaml_path, target_path).unwrap();
    }

    let mut assets_path = workspace_dir.clone();
    assets_path.push("Assets");
    println!("cargo:rerun-if-changed={}", assets_path.display());

    // Create Assets folder
    let mut assets_target_path = out_dir.clone();
    assets_target_path.push("Assets");
    create_dir(&assets_target_path)
        .or_else(|e| {
            if matches!(e.kind(), ErrorKind::AlreadyExists) {
                Ok(())
            } else {
                Err(e)
            }
        })
        .unwrap();

    // Copy Assets
    for asset_path in read_dir(assets_path)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .map(|e| e.path())
        .filter(|p| p.is_file())
    {
        let mut target_path = assets_target_path.clone();
        target_path.push(asset_path.file_name().unwrap());
        eprintln!("Copying asset {:?} to {:?}", asset_path, target_path);
        println!("cargo:rerun-if-changed={}", asset_path.display());
        copy(asset_path, target_path).unwrap();
    }

    // Prepare appxmapping.ini
    let mut appxmapping_path = workspace_dir.clone();
    appxmapping_path.push("appxmapping.template.ini");
    println!("cargo:rerun-if-changed={}", appxmapping_path.display());
    let appxmapping = String::from_utf8(read(&appxmapping_path).unwrap())
        .unwrap()
        .replace("$target_dir$", &out_dir.display().to_string());
    appxmapping_path.set_file_name("appxmapping.ini");
    eprintln!("Writing appxmapping");
    write(appxmapping_path, appxmapping).unwrap();

    // Prepare AppxManifest.xml
    let mut manifest_path = workspace_dir.clone();
    manifest_path.push("AppxManifest.template.xml");
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    let manifest = String::from_utf8(read(&manifest_path).unwrap())
        .unwrap()
        .replace(
            "$arch$",
            match &*var("CARGO_CFG_TARGET_ARCH").unwrap() {
                "x86_64" | "x64" => "x64",
                "i686" | "x86" => "x86",
                "thumbv7a" | "arm" => "arm",
                "aarch64" | "arm64" => "arm64",
                arch => panic!(format!("Unknown architecture {}", arch)),
            },
        )
        .replace(
            "$exe_prefix$",
            if var("CARGO_CFG_TARGET_VENDOR").unwrap() == "uwp" {
                "".to_string()
            } else {
                out_dir
                    .strip_prefix(workspace_dir)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    + "\\"
            }
            .as_str(),
        );

    manifest_path.set_file_name("AppxManifest.xml");
    eprintln!("Writing AppxManifest");
    write(manifest_path, manifest).unwrap();
}
