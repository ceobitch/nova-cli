//! Build script for CyberSec AI Terminal desktop application

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Set up icon and resources
    setup_app_resources();
    
    // Configure build for different platforms
    configure_platform_specific();
    
    println!("cargo:rerun-if-changed=app-config/");
}

fn setup_app_resources() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).join("app-resources");
    
    // Create target directory
    fs::create_dir_all(&target_dir).unwrap();
    
    // Copy application manifest and metadata
    if cfg!(windows) {
        create_windows_manifest(&target_dir);
    }
    
    if cfg!(target_os = "macos") {
        create_macos_info_plist(&target_dir);
    }
    
    if cfg!(unix) && !cfg!(target_os = "macos") {
        create_linux_desktop_file(&target_dir);
    }
}

fn configure_platform_specific() {
    // Windows-specific configuration
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
        embed_windows_manifest();
    }
    
    // macOS-specific configuration
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
    }
    
    // Linux-specific configuration
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=Xclipboard");
    }
}

#[cfg(windows)]
fn embed_windows_manifest() {
    let manifest = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity
    version="1.0.0.0"
    processorArchitecture="*"
    name="CyberSecAITerminal"
    type="win32" />
  <description>CyberSec AI Terminal - Cybersecurity Analysis Tool</description>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v2">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="asInvoker" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>
"#;
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_path = Path::new(&out_dir).join("app.manifest");
    fs::write(manifest_path, manifest).unwrap();
}

#[cfg(windows)]
fn create_windows_manifest(target_dir: &Path) {
    let manifest_content = r#"
[Application]
ApplicationName=CyberSec AI Terminal
ApplicationVersion=1.0.0
ApplicationDescription=AI-powered cybersecurity terminal companion
ApplicationPublisher=CyberSec AI
ApplicationCategory=Security

[Compatibility]
Windows10=true
Windows11=true

[Features]
ClipboardAccess=true
FileSystemAccess=true
NetworkAccess=true
ProcessMonitoring=true
"#;
    
    fs::write(target_dir.join("app.ini"), manifest_content).unwrap();
}

#[cfg(target_os = "macos")]
fn create_macos_info_plist(target_dir: &Path) {
    let plist_content = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDisplayName</key>
    <string>CyberSec AI Terminal</string>
    <key>CFBundleExecutable</key>
    <string>cybersec-terminal</string>
    <key>CFBundleIdentifier</key>
    <string>com.cybersec.ai.terminal</string>
    <key>CFBundleName</key>
    <string>CyberSec AI Terminal</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.security</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2024 CyberSec AI. All rights reserved.</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
</dict>
</plist>
"#;
    
    fs::write(target_dir.join("Info.plist"), plist_content).unwrap();
}

#[cfg(all(unix, not(target_os = "macos")))]
fn create_linux_desktop_file(target_dir: &Path) {
    let desktop_content = r#"
[Desktop Entry]
Version=1.0
Type=Application
Name=CyberSec AI Terminal
Comment=AI-powered cybersecurity terminal companion
Exec=cybersec-terminal
Icon=cybersec-terminal
Terminal=false
Categories=Security;System;Network;
Keywords=security;cybersecurity;malware;antivirus;scanner;
StartupNotify=true
MimeType=application/x-cybersec-report;
"#;
    
    fs::write(target_dir.join("cybersec-terminal.desktop"), desktop_content).unwrap();
}

