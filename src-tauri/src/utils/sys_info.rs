use once_cell::sync::Lazy;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct SystemInfo {
    pub hwid: String,
    pub os_type: String,
    pub os_ver: String,
}

#[cfg(not(target_os = "android"))]
fn get_device_id() -> String {
    machine_uid::get().unwrap_or_else(|_| "unknown_hwid".to_string())
}

#[cfg(target_os = "android")]
fn get_device_id() -> String {
    get_android_id().unwrap_or_else(|| "unknown_android_id".to_string())
}

#[cfg(target_os = "android")]
fn get_android_id() -> Option<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    
    // Use some system properties that should be consistent across app runs
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        hostname.hash(&mut hasher);
    }
    if let Ok(user) = std::env::var("USER") {
        user.hash(&mut hasher);
    }
    
    // Add some Android-specific paths/properties
    std::path::Path::new("/system").hash(&mut hasher);
    
    let hash = hasher.finish();
    Some(format!("android_{:016x}", hash))
}

pub static SYSTEM_INFO: Lazy<SystemInfo> = Lazy::new(|| {
    let os_info = os_info::get();
    SystemInfo {
        hwid: get_device_id(),
        os_type: os_info.os_type().to_string(),
        os_ver: os_info.version().to_string(),
    }
});

pub fn get_system_info() -> &'static SystemInfo {
    &SYSTEM_INFO
}

#[cfg(not(target_os = "android"))]
fn get_device_id() -> String {
    machine_uid::get().unwrap_or_else(|_| "unknown_hwid".to_string())
}

#[cfg(target_os = "android")]
fn get_device_id() -> String {
    get_android_id().unwrap_or_else(|| "unknown_android_id".to_string())
}

#[cfg(target_os = "android")]
fn get_android_id() -> Option<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        hostname.hash(&mut hasher);
    }
    if let Ok(user) = std::env::var("USER") {
        user.hash(&mut hasher);
    }

    std::path::Path::new("/system").hash(&mut hasher);
    
    let hash = hasher.finish();
    Some(format!("android_{:016x}", hash))
}