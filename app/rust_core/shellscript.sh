#!/bin/bash

x86_64_library="libanvil_x86_64.a"
aarch64_library="libanvil_aarch64.a"
output_library="libanvil.a"
current_directory=$(pwd)


lipo -create -output "$output_library" "$x86_64_library" "$aarch64_library"
echo "successfully created combined library at: ${current_directory}/${output_library}"

echo "cleaning up.."
rm "$x86_64_library" "$aarch64_library"
echo "deleted \n${current_directory}/${x86_64_library}\n${current_directory}/${aarch64_library}"
echo "cleaned up! :)"