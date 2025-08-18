use anyhow::{anyhow, Result};
use crate::utils::logging::Type;
use crate::logging;

#[cfg(target_os = "android")]
use jni::{
    objects::{JClass, JObject, JString, JValue},
    sys::{jboolean, JNI_FALSE, JNI_TRUE},
    JNIEnv, JavaVM,
};

#[cfg(target_os = "android")]
use ndk_context;

/// Android VPN integration using JNI
pub struct AndroidVpnManager;

impl AndroidVpnManager {
    /// Check if VPN permission is granted
    pub async fn check_vpn_permission() -> Result<bool> {
        #[cfg(target_os = "android")]
        {
            logging!(info, Type::Service, true, "[AndroidVPN] Checking VPN permission via JNI");
            
            match Self::call_java_boolean_method("checkVpnPermission") {
                Ok(has_permission) => {
                    logging!(info, Type::Service, true, "[AndroidVPN] VPN permission check result: {}", has_permission);
                    Ok(has_permission)
                }
                Err(e) => {
                    logging!(error, Type::Service, true, "[AndroidVPN] Failed to check VPN permission via JNI: {}", e);
                    Err(e)
                }
            }
        }
        
        #[cfg(not(target_os = "android"))]
        {
            logging!(warn, Type::Service, true, "[AndroidVPN] VPN permission check called on non-Android platform");
            Ok(false)
        }
    }

    /// Request VPN permission using native Android dialog
    pub async fn request_vpn_permission() -> Result<()> {
        #[cfg(target_os = "android")]
        {
            logging!(info, Type::Service, true, "[AndroidVPN] Requesting VPN permission via JNI");
            
            match Self::call_java_void_method("requestVpnPermissionJni") {
                Ok(_) => {
                    logging!(info, Type::Service, true, "[AndroidVPN] VPN permission request initiated successfully");
                    Ok(())
                }
                Err(e) => {
                    logging!(error, Type::Service, true, "[AndroidVPN] Failed to request VPN permission via JNI: {}", e);
                    Err(e)
                }
            }
        }
        
        #[cfg(not(target_os = "android"))]
        {
            logging!(warn, Type::Service, true, "[AndroidVPN] VPN permission request called on non-Android platform");
            Ok(())
        }
    }

    /// Start VPN service
    pub async fn start_vpn_service() -> Result<()> {
        #[cfg(target_os = "android")]
        {
            logging!(info, Type::Service, true, "[AndroidVPN] Starting VPN service via JNI");
            
            match Self::call_java_void_method("startVpnServiceJni") {
                Ok(_) => {
                    logging!(info, Type::Service, true, "[AndroidVPN] VPN service start initiated successfully");
                    Ok(())
                }
                Err(e) => {
                    logging!(error, Type::Service, true, "[AndroidVPN] Failed to start VPN service via JNI: {}", e);
                    Err(e)
                }
            }
        }
        
        #[cfg(not(target_os = "android"))]
        {
            logging!(warn, Type::Service, true, "[AndroidVPN] VPN service start called on non-Android platform");
            Ok(())
        }
    }

    /// Stop VPN service
    pub async fn stop_vpn_service() -> Result<()> {
        #[cfg(target_os = "android")]
        {
            logging!(info, Type::Service, true, "[AndroidVPN] Stopping VPN service via JNI");
            
            match Self::call_java_void_method("stopVpnServiceJni") {
                Ok(_) => {
                    logging!(info, Type::Service, true, "[AndroidVPN] VPN service stop initiated successfully");
                    Ok(())
                }
                Err(e) => {
                    logging!(error, Type::Service, true, "[AndroidVPN] Failed to stop VPN service via JNI: {}", e);
                    Err(e)
                }
            }
        }
        
        #[cfg(not(target_os = "android"))]
        {
            logging!(warn, Type::Service, true, "[AndroidVPN] VPN service stop called on non-Android platform");
            Ok(())
        }
    }

    /// Check if VPN service is running
    pub async fn is_vpn_service_running() -> Result<bool> {
        #[cfg(target_os = "android")]
        {
            logging!(info, Type::Service, true, "[AndroidVPN] Checking VPN service status via JNI");
            
            match Self::call_java_boolean_method("isVpnServiceRunning") {
                Ok(is_running) => {
                    logging!(info, Type::Service, true, "[AndroidVPN] VPN service running status: {}", is_running);
                    Ok(is_running)
                }
                Err(e) => {
                    logging!(error, Type::Service, true, "[AndroidVPN] Failed to check VPN service status via JNI: {}", e);
                    Err(e)
                }
            }
        }
        
        #[cfg(not(target_os = "android"))]
        {
            logging!(warn, Type::Service, true, "[AndroidVPN] VPN service status check called on non-Android platform");
            Ok(false)
        }
    }

    #[cfg(target_os = "android")]
    fn call_java_boolean_method(method_name: &str) -> Result<bool> {
        // Get the Android context and JVM
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;
        
        // Get the current activity instance
        let activity = unsafe { JObject::from_raw(ctx.context().cast()) };

        // Call the method and extract boolean result
        let result = env.call_method(
            &activity,
            method_name,
            "()Z",
            &[],
        )?;

        Ok(result.z().unwrap_or(false))
    }
    
    #[cfg(target_os = "android")]
    fn call_java_void_method(method_name: &str) -> Result<()> {
        // Get the Android context and JVM
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;
        
        // Get the current activity instance
        let activity = unsafe { JObject::from_raw(ctx.context().cast()) };

        // Call the method
        env.call_method(
            &activity,
            method_name,
            "()V",
            &[],
        )?;

        Ok(())
    }
}

/// Initialize Android VPN integration
pub fn init_android_vpn() -> Result<()> {
    #[cfg(target_os = "android")]
    {
        logging!(info, Type::Service, true, "[AndroidVPN] Initializing Android VPN integration");
        // Any initialization code can go here
        Ok(())
    }
    
    #[cfg(not(target_os = "android"))]
    {
        logging!(info, Type::Service, true, "[AndroidVPN] Android VPN integration not available on this platform");
        Ok(())
    }
}
