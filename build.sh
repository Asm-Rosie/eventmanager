#!/bin/bash
echo "checking your OS.."

if [[ "$OSTYPE" == "darwin"* ]]; then
  echo "OS is MacOS, running build script!"
  cp "$(pwd)/rust_core/Cargo-mac.toml" "$(pwd)/rust_core/Cargo.toml"
  echo "copied necessary files"

elif [["$OSTYPE" == "linux-gnu"* ]]; then
  echo "OS is Linux, and Linux is currently not supported"
  exit 1

else
  echo "OS is not supported"
  exit 1
fi

X64_APPLE_INPUT_FOLDER="rust_core/target/x86_64-apple-darwin/release/"
ARM_APPLE_INPUT_FOLDER="rust_core/target/aarch64-apple-darwin/release/"
DESTINATION_FOLDER="eventmanager/rust_core/"


echo "building.."
command rustup target add x86_64-apple-darwin
command rustup target add aarch64-apple-darwin
echo "targets have been installed or are up to date"
command cargo build --release --target x86_64-apple-darwin --manifest-path ./rust_core/Cargo.toml
command cargo build --release --target aarch64-apple-darwin --manifest-path ./rust_core/Cargo.toml
echo "building finished!"
echo "checking if there is already an existing build.."
if [ -f "eventmanager/rust_core/binded_lib.a" ]; then
  echo "there is already an existing build. creating backup.. *just in case*"
  mkdir "$DESTINATION_FOLDER/backups"
  mv "$DESTINATION_FOLDER/binded_lib.a" "$DESTINATION_FOLDER/backups/binded_lib.a"
  echo "backup created"
fi
echo "binding libraries.."
command lipo -create -output $DESTINATION_FOLDER/binded_lib.a $X64_APPLE_INPUT_FOLDER/libeventmanager_core.a $ARM_APPLE_INPUT_FOLDER/libeventmanager_core.a
echo "binded libraries successfully successfully"
command lipo -info $DESTINATION_FOLDER/binded_lib.a
echo "building finished successfully!, you can now build your app in Xcode by just hitting the run button"
