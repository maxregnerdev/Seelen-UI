use std::path::PathBuf;

use windows::Win32::Storage::Packaging::Appx::GetCurrentPackageId;

pub fn is_windows_10() -> bool {
    matches!(os_info::get().version(), os_info::Version::Semantic(_, _, x) if (&10240..&22000).contains(&x))
}

#[allow(dead_code)]
pub fn is_windows_11() -> bool {
    matches!(os_info::get().version(), os_info::Version::Semantic(_, _, x) if x >= &22000)
}

/// Check if running on Windows 11 or Windows 12
/// Windows 12 features are enabled on Windows 11
pub fn is_windows_11_or_12() -> bool {
    is_windows_11()
}

/// Check if Windows 12 features should be enabled
/// Returns true for Windows 11 and Windows 12
pub fn is_windows_12_features_enabled() -> bool {
    is_windows_11_or_12()
}

/// Get the Windows version as a numeric value for feature comparison
pub fn windows_version_num() -> u32 {
    match os_info::get().version() {
        os_info::Version::Semantic(major, minor, build) => {
            *major as u32 * 10000 + *minor as u32 * 100 + *build as u32
        }
        _ => 0,
    }
}

/// Check if running on Windows 12 (build 26100+)
/// But features are also enabled on Windows 11
pub fn is_windows_12() -> bool {
    match os_info::get().version() {
        os_info::Version::Semantic(_, _, x) => x >= &26100,
        _ => false,
    }
}

pub fn has_fixed_runtime() -> bool {
    std::env::var_os("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER").is_some()
}

pub fn get_fixed_runtime_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let install_dir = exe.parent()?;
    let read_dir = install_dir.join("runtime").read_dir().ok()?;
    let runtime = read_dir.last()?.ok()?.path();
    if runtime.join("msedgewebview2.exe").exists() {
        Some(runtime)
    } else {
        None
    }
}

pub fn is_running_as_appx() -> bool {
    static CACHE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHE.get_or_init(|| unsafe {
        let mut len = 0u32;
        let _ = GetCurrentPackageId(&mut len, None);
        let mut buffer = vec![0u8; len as usize];
        GetCurrentPackageId(&mut len, Some(buffer.as_mut_ptr())).is_ok()
    })
}

pub fn was_installed_using_msix() -> bool {
    static CACHE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHE.get_or_init(|| {
        std::env::current_exe().is_ok_and(|p| p.with_file_name("AppxManifest.xml").exists())
    })
}
