# First UWP in Rust
The first Universal Windows Platform app written in pure Rust.

## Build and Run
```powershell
# Build the project using x86_64-pc-windows-msvc default target
cargo build

# Register the package layout
Add-AppxPackage -Register AppxManifest.xml

# Then click "First UWP in rs" in your Start Menu
```
