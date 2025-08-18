import { invoke } from "@tauri-apps/api/core";

/**
 * Check if VPN permission is granted (Android only)
 */
export async function checkVpnPermission(): Promise<boolean> {
  return invoke("check_vpn_permission");
}

/**
 * Request VPN permission by opening Android VPN settings
 */
export async function requestVpnPermission(): Promise<void> {
  return invoke("request_vpn_permission");
}

/**
 * Start VPN service (Android only)
 * @returns File descriptor for TUN interface, or -1 if failed
 */
export async function startVpnService(): Promise<number> {
  return invoke("start_vpn_service");
}

/**
 * Stop VPN service (Android only)
 */
export async function stopVpnService(): Promise<void> {
  return invoke("stop_vpn_service");
}

/**
 * Check if VPN service is running (Android only)
 */
export async function isVpnServiceRunning(): Promise<boolean> {
  return invoke("is_vpn_service_running");
}
