# First UWP in Rust
The first Universal Windows Platform app written in pure Rust.

**Note: this project is an early-stage experiment. Code quality is not guaranteed. Crashes or memory leak may occur.**

## Build and Run
```powershell
# Build the project using x86_64-pc-windows-msvc default target
cargo build

# Register the package layout
Add-AppxPackage -Register AppxManifest.xml

# Then click "First UWP in rs" in your Start Menu
```

## Cross compilation and Packaging
The default targets `*-pc-windows-*` has some link options unsuitable for UWP applications. A package that contains executable files generated with such target fails Windows App Cert Kit (WACK) in terms of `AppContainerCheck` and unsupported APIs used. To pass WACK, `*-uwp-windows-*` targets should be used.

[Xargo](https://github.com/japaric/xargo) is recommended for cross-compilation in this project. Using Xargo, it is not necessary to build the whole Rust toolchain in order to consume tier-3 targets.

Follow the steps below in `powershell` to build and generate a `.appx` package:
1. Set up Rust nightly toolchain override for this project
```powershell
rustup override set nightly
```
2. Install xargo
```powershell
cargo install xargo
```
3. Build with xargo
```powershell
xargo build --release --target x86_64-uwp-windows-msvc
```
`i686-uwp-windows-msvc`, `thumbv7a-uwp-windows-msvc` and `aarch64-uwp-windows-msvc` targets can be used for x64, ARM and ARM64.

4. Set up environment variables before consuming Windows SDK command line tools
```powershell
$env:Path+=";${env:ProgramFiles(x86)}\Windows Kits\10\bin\%SDK_VERSION%\x64"
```
where `%SDK_VERSION%` is the version of an installed Windows SDK that will provide the necessary command line tools, such as `10.0.18362.0`.

5. Create a `.appx` package
```powershell
makeappx pack /p FirstUwp_0.0.1.0_x64_Test.appx /v /f .\appxmapping.ini
```
6. Generate a certificate for self-signing
```powershell
$cert=New-SelfSignedCertificate -Type Custom -Subject "CN=25C90434-4343-4A2A-BB16-CF3209256BD3" -KeyUsage DigitalSignature -FriendlyName "firstuwpcert" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")
$data=$cert.Export([System.Security.Cryptography.X509Certificates.X509ContentType]::Pfx)
[io.file]::WriteAllBytes('firstuwp_TemporaryKey.pfx', $data)
```
7. Import the generated certificate

See https://docs.microsoft.com/en-us/windows/application-management/sideload-apps-in-windows-10#how-do-i-sideload-an-app-on-desktop .

8. Sign the package
```powershell
signtool sign /v /fd SHA256 /a /f firstuwp_TemporaryKey.pfx FirstUwp_0.0.1.0_x64_Test.appx
```

Then the package is ready to deploy through [Device Portal](https://docs.microsoft.com/en-us/windows/uwp/debug-test-perf/device-portal) or [App Install](https://docs.microsoft.com/en-us/windows/msix/app-installer/app-installer-root) at your option.
