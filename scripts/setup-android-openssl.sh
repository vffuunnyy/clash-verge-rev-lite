#!/bin/bash

# Setup script for Android OpenSSL cross-compilation
# This script helps set up OpenSSL for Android builds

echo "Setting up Android OpenSSL cross-compilation..."

# Check if Android NDK is installed
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "❌ ANDROID_NDK_HOME is not set!"
    echo "Please install Android NDK and set ANDROID_NDK_HOME environment variable"
    echo "Example: export ANDROID_NDK_HOME=/path/to/android-ndk"
    exit 1
fi

# Check if Android NDK exists
if [ ! -d "$ANDROID_NDK_HOME" ]; then
    echo "❌ Android NDK directory not found: $ANDROID_NDK_HOME"
    exit 1
fi

echo "✅ Found Android NDK at: $ANDROID_NDK_HOME"

# Set up environment variables for aarch64-linux-android
export ANDROID_API=21
export TARGET=aarch64-linux-android
export NDK_TOOLCHAIN=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64

# Set up compiler paths
export CC=$NDK_TOOLCHAIN/bin/aarch64-linux-android${ANDROID_API}-clang
export CXX=$NDK_TOOLCHAIN/bin/aarch64-linux-android${ANDROID_API}-clang++
export AR=$NDK_TOOLCHAIN/bin/llvm-ar
export RANLIB=$NDK_TOOLCHAIN/bin/llvm-ranlib
export STRIP=$NDK_TOOLCHAIN/bin/llvm-strip

# Set up OpenSSL cross-compilation
export OPENSSL_STATIC=1
export OPENSSL_DIR=/tmp/openssl-android-$TARGET
export PKG_CONFIG_ALLOW_CROSS=1

echo "Environment variables set for Android cross-compilation:"
echo "TARGET: $TARGET"
echo "ANDROID_API: $ANDROID_API"
echo "CC: $CC"
echo "OPENSSL_DIR: $OPENSSL_DIR"

# Check if OpenSSL for Android is already built
if [ ! -d "$OPENSSL_DIR" ]; then
    echo "📦 Building OpenSSL for Android..."
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd $TEMP_DIR
    
    # Download OpenSSL
    wget https://www.openssl.org/source/openssl-1.1.1w.tar.gz
    tar xzf openssl-1.1.1w.tar.gz
    cd openssl-1.1.1w
    
    # Configure OpenSSL for Android aarch64
    ./Configure android-arm64 \
        -D__ANDROID_API__=$ANDROID_API \
        --prefix=$OPENSSL_DIR \
        --openssldir=$OPENSSL_DIR \
        no-shared
    
    # Build OpenSSL
    make
    make install_sw
    
    echo "✅ OpenSSL built and installed to: $OPENSSL_DIR"
    
    # Cleanup
    cd /
    rm -rf $TEMP_DIR
else
    echo "✅ OpenSSL for Android already exists at: $OPENSSL_DIR"
fi

echo "🎉 Setup complete! You can now build with:"
echo "cargo build --target aarch64-linux-android --release"
