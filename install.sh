#!/bin/bash

# Set terminal colors
GREEN=$(tput setaf 2)   # Green text
RED=$(tput setaf 1)     # Red text
RESET=$(tput sgr0)      # Reset text color


echo "${GREEN}Checking your OS..${RESET}"

if [[ "$OSTYPE" == "darwin"* ]]; then
  echo "OS is MacOS, running build script!"

elif [["$OSTYPE" == "linux-gnu"* ]]; then
  echo "OS is Linux, and Linux is currently not supported"
  exit 1

else
  echo "OS is not supported"
  exit 1
fi

if command -v cargo >/dev/null 2>&1; then
  echo "${GREEN}cargo is installed${RESET}"
  echo "checking if targets are up to date.."
  command rustup target add x86_64-apple-darwin
  command rustup target add aarch64-apple-darwin
else
  echo "cargo is not installed"
  echo "this script can install cargo and all needed resources automatically for you, if you prefer to install cargo manually, go to https://www.rust-lang.org/tools/install and follow the instructions"
  echo "do you want that I install cargo for you?"
  echo "y/n"
  read answer
  if [ "$answer" == "y" ]; then
    command curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo "installed cargo!"
    echo "installing necessary resources.."
    command rustup target add x86_64-darwin
    command rustup target add aarch64-apple-darwin
    echo "installed resources!"
    echo "checking if targets are up to date.."
    command rustup target add x86_64-apple-darwin
    command rustup target add aarch64-apple-darwin
  elif [ "$answer" == "n" ]; then
    echo "exiting.."
    exit 1
  fi
fi

command chmod +x build.sh

echo "---------------------------------------------------------------------------"
echo "do you want to build the source code right now or do you want to do it later?"
echo "you can also build the source later using following command: ./build.sh"
# Set terminal colors

# Your input text
text="${GREEN}y${RESET}/${RED}n${RESET}"

# Print the colored text
echo "$text"

read answer
if [ "$answer" == "y" ]; then
  echo "starting build process"
  command ./build.sh
fi