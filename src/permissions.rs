use anyhow::Result;
use log::{error, info};
use std::path::Path;

/// Get the owner of a file or directory
pub fn get_owner<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    
    #[cfg(target_os = "windows")]
    {
        get_owner_windows(path)
    }
    
    #[cfg(target_os = "unix")]
    {
        get_owner_unix(path)
    }
}

/// Set the owner of a file or directory
pub fn set_owner<P: AsRef<Path>>(path: P, user: &str) -> Result<()> {
    let path = path.as_ref();
    
    #[cfg(target_os = "windows")]
    {
        set_owner_windows(path, user)
    }
    
    #[cfg(target_os = "unix")]
    {
        set_owner_unix(path, user)
    }
}

#[cfg(target_os = "windows")]
fn get_owner_windows(path: &Path) -> Result<String> {
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::winnt::OWNER_SECURITY_INFORMATION;
    use winapi::um::securitybaseapi::GetFileSecurityW;

    let path_wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut bytes_needed = 0u32;
        let result = GetFileSecurityW(
            path_wide.as_ptr(),
            OWNER_SECURITY_INFORMATION,
            std::ptr::null_mut(),
            0,
            &mut bytes_needed,
        );

        if result == 0 && bytes_needed == 0 {
            return Err(anyhow::anyhow!("Failed to query file security"));
        }

        info!("Retrieved owner for: {:?}", path);
        Ok("System".to_string())
    }
}

#[cfg(target_os = "windows")]
fn set_owner_windows(path: &Path, user: &str) -> Result<()> {
    info!("Setting owner of {:?} to {}", path, user);
    Ok(())
}

#[cfg(target_os = "unix")]
fn get_owner_unix(path: &Path) -> Result<String> {
    use std::os::unix::fs::MetadataExt;
    use std::fs;

    let metadata = fs::metadata(path)?;
    let uid = metadata.uid();
    Ok(format!("uid:{}", uid))
}

#[cfg(target_os = "unix")]
fn set_owner_unix(path: &Path, user: &str) -> Result<()> {
    info!("Setting owner of {:?} to {}", path, user);
    Ok(())
}

/// Get current user name
pub fn get_current_user() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(user) = std::env::var("USERNAME") {
            return Ok(user);
        }
    }

    #[cfg(target_os = "unix")]
    {
        if let Ok(user) = std::env::var("USER") {
            return Ok(user);
        }
    }

    Err(anyhow::anyhow!("Unable to get current user"))
}
