#!/bin/bash

# Set terminal colors
GREEN=$(tput setaf 2)   # Green text
RED=$(tput setaf 1)     # Red text
RESET=$(tput sgr0)      # Reset text color

BOLD="\033[1m"
RESETANSI="\033[0m"

echo "${GREEN}checking your OS..${RESET}"

if [[ "$OSTYPE" == "darwin"* ]]; then
  echo -e "OS is ${BOLD}MacOS${RESETANSI}, running build script!"

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
command cargo build --release --target x86_64-apple-darwin --manifest-path ./rust_core/Cargo.toml
command cargo build --release --target aarch64-apple-darwin --manifest-path ./rust_core/Cargo.toml
echo "building finished!"
echo "checking if there is already an existing build.."
if [ -f "eventmanager/rust_core/binded_lib.a" ]; then
  echo "there is already an existing build. creating backup.. *just in case*"
  mkdir "$DESTINATION_FOLDER/backups"
  mv "$DESTINATION_FOLDER/binded_lib.a" "$DESTINATION_FOLDER/backups/binded_lib.a"
  echo "${GREEN}backup created${RESET}"
fi
echo "binding libraries.."
command lipo -create -output $DESTINATION_FOLDER/binded_lib.a $X64_APPLE_INPUT_FOLDER/libeventmanager_core.a $ARM_APPLE_INPUT_FOLDER/libeventmanager_core.a
echo -e "${BOLD}binded libraries successfully successfully${RESETANSI}"
command lipo -info $DESTINATION_FOLDER/binded_lib.a
echo "${GREEN}operation complete!${RESET}"
echo "you can now build your app in Xcode by just hitting the run button"
