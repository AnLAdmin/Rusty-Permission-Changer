use anyhow::Result;
use log::{error, info};
use std::path::Path;
use std::fs;
use crate::permissions;
use crate::database;

pub struct OwnershipCheckResult {
    pub path: String,
    pub owner: String,
    pub needs_change: bool,
}

pub struct ChangeResult {
    pub path: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Check ownership of files/folders
pub fn check_ownership(paths: &[String], current_user: &str) -> Result<Vec<OwnershipCheckResult>> {
    let mut results = Vec::new();
    
    for path in paths {
        match permissions::get_owner(path) {
            Ok(owner) => {
                let needs_change = owner != current_user;
                results.push(OwnershipCheckResult {
                    path: path.clone(),
                    owner,
                    needs_change,
                });
                info!("Checked: {} - Owner: {}", path, owner);
            }
            Err(e) => {
                error!("Error checking {}: {}", path, e);
                results.push(OwnershipCheckResult {
                    path: path.clone(),
                    owner: "Unknown".to_string(),
                    needs_change: true,
                });
            }
        }
    }
    
    Ok(results)
}

/// Change ownership of selected items
pub fn change_ownership(items: &[(String, String)], current_user: &str) -> Result<Vec<ChangeResult>> {
    let mut results = Vec::new();
    
    for (path, original_owner) in items {
        match permissions::set_owner(path, current_user) {
            Ok(_) => {
                if let Ok(new_owner) = permissions::get_owner(path) {
                    let _ = database::record_change(path, original_owner, &new_owner);
                    results.push(ChangeResult {
                        path: path.clone(),
                        success: true,
                        error: None,
                    });
                    info!("Ownership changed for {} to {}", path, new_owner);
                } else {
                    results.push(ChangeResult {
                        path: path.clone(),
                        success: true,
                        error: None,
                    });
                }
            }
            Err(e) => {
                error!("Failed to change owner for {}: {}", path, e);
                results.push(ChangeResult {
                    path: path.clone(),
                    success: false,
                    error: Some(e.to_string()),
                });
            }
        }
    }
    
    Ok(results)
}

/// Revert ownership changes
pub fn revert_ownership(records: &[(i32, String, String)]) -> Result<Vec<ChangeResult>> {
    let mut results = Vec::new();
    
    for (id, path, original_owner) in records {
        match permissions::set_owner(path, original_owner) {
            Ok(_) => {
                let _ = database::delete_change(*id);
                results.push(ChangeResult {
                    path: path.clone(),
                    success: true,
                    error: None,
                });
                info!("Reverted ownership for {}", path);
            }
            Err(e) => {
                error!("Failed to revert owner for {}: {}", path, e);
                results.push(ChangeResult {
                    path: path.clone(),
                    success: false,
                    error: Some(e.to_string()),
                });
            }
        }
    }
    
    Ok(results)
}

/// Walk directory and collect all files
pub fn walk_directory(path: &str) -> Result<Vec<String>> {
    let mut paths = Vec::new();
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                let entry_path = entry.path();
                if metadata.is_dir() {
                    if let Ok(mut subpaths) = walk_directory(&entry_path.to_string_lossy()) {
                        paths.append(&mut subpaths);
                    }
                } else {
                    paths.push(entry_path.to_string_lossy().to_string());
                }
            }
        }
    }
    
    Ok(paths)
}
