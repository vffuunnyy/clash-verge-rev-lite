export APK_PATH=src-tauri/gen/android/app/build/outputs/apk/universal/release

bun tauri android build --apk --target aarch64
apksigner sign \
  --ks ~/.android/clash.keystore \
  --ks-key-alias clash-verge \
  --ks-pass pass:226589 \
  --key-pass pass:226589 \
  --out "$APK_PATH/clash-verge-release.apk" \
  "$APK_PATH/app-universal-release-unsigned.apk"
adb install -r $APK_PATH/clash-verge-release.apk