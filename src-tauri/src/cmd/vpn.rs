use super::CmdResult;
use crate::{logging, platform::AndroidVpnManager, utils::logging::Type};
use anyhow::Result;
use tauri::{command, AppHandle};
use tauri_plugin_shell::ShellExt;

/// Check if VPN permission is granted (Android only)
#[tauri::command]
pub async fn check_vpn_permission(_app_handle: tauri::AppHandle) -> CmdResult<bool> {
    logging!(
        info,
        Type::Service,
        true,
        "[VPN] Checking VPN permission via JNI bridge"
    );
    
    match AndroidVpnManager::check_vpn_permission().await {
        Ok(has_permission) => {
            logging!(
                info,
                Type::Service,
                true,
                "[VPN] VPN permission check result: {}",
                has_permission
            );
            Ok(has_permission)
        }
        Err(e) => {
            logging!(
                error,
                Type::Service,
                true,
                "[VPN] Failed to check VPN permission: {}",
                e
            );
            Ok(false)
        }
    }
}

/// Request VPN permission using native Android VPN dialog
#[tauri::command]
pub async fn request_vpn_permission(_app_handle: tauri::AppHandle) -> CmdResult<()> {
    logging!(
        info,
        Type::Service,
        true,
        "[VPN] Requesting VPN permission via JNI bridge"
    );
    
    match AndroidVpnManager::request_vpn_permission().await {
        Ok(()) => {
            logging!(
                info,
                Type::Service,
                true,
                "[VPN] VPN permission request initiated successfully"
            );
            Ok(())
        }
        Err(e) => {
            logging!(
                error,
                Type::Service,
                true,
                "[VPN] Failed to request VPN permission: {}",
                e
            );
            Err(format!("Failed to request VPN permission: {}", e))
        }
    }
}

/// Start VPN service using native Android API
#[tauri::command]
pub async fn start_vpn_service(_app_handle: tauri::AppHandle) -> CmdResult<i32> {
    logging!(
        info,
        Type::Service,
        true,
        "[VPN] Starting VPN service via JNI bridge"
    );
    
    match AndroidVpnManager::start_vpn_service().await {
        Ok(()) => {
            logging!(
                info,
                Type::Service,
                true,
                "[VPN] VPN service start initiated successfully"
            );
            // Return a success file descriptor
            // In a real implementation, this would come from the VPN service
            Ok(1)
        }
        Err(e) => {
            logging!(
                error,
                Type::Service,
                true,
                "[VPN] Failed to start VPN service: {}",
                e
            );
            Ok(-1)
        }
    }
}

/// Stop VPN service using native Android API
#[tauri::command]
pub async fn stop_vpn_service(_app_handle: tauri::AppHandle) -> CmdResult<()> {
    logging!(
        info,
        Type::Service,
        true,
        "[VPN] Stopping VPN service via JNI bridge"
    );
    
    match AndroidVpnManager::stop_vpn_service().await {
        Ok(()) => {
            logging!(
                info,
                Type::Service,
                true,
                "[VPN] VPN service stop initiated successfully"
            );
            Ok(())
        }
        Err(e) => {
            logging!(
                error,
                Type::Service,
                true,
                "[VPN] Failed to stop VPN service: {}",
                e
            );
            Err(format!("Failed to stop VPN service: {}", e))
        }
    }
}

/// Check if VPN service is running using native Android API
#[tauri::command]
pub async fn is_vpn_service_running(_app_handle: tauri::AppHandle) -> CmdResult<bool> {
    logging!(
        info,
        Type::Service,
        true,
        "[VPN] Checking VPN service status via JNI bridge"
    );
    
    match AndroidVpnManager::is_vpn_service_running().await {
        Ok(is_running) => {
            logging!(
                info,
                Type::Service,
                true,
                "[VPN] VPN service running status: {}",
                is_running
            );
            Ok(is_running)
        }
        Err(e) => {
            logging!(
                error,
                Type::Service,
                true,
                "[VPN] Failed to check VPN service status: {}",
                e
            );
            Ok(false)
        }
    }
}
