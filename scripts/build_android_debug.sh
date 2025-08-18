export APK_PATH=src-tauri/gen/android/app/build/outputs/apk/universal/debug/

export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android28-clang"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_AR="aarch64-linux-android-ar"

bun tauri android build --debug --apk --target aarch64
# apksigner sign \
#   --ks ~/.android/clash.keystore \
#   --ks-key-alias clash-verge \
#   --ks-pass pass:226589 \
#   --key-pass pass:226589 \
#   --out "$APK_PATH/clash-verge-release.apk" \
#   "$APK_PATH/app-universal-release-unsigned.apk"
adb install -r $APK_PATH/app-universal-debug.apk