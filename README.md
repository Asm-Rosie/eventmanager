# eventmanager
manage your personal tasks! 

# MacOS instructions
the following below will explain how you can build it yourself using macOS, a prebuilt app can be found in the [release tab](https://github.com/AkameTheCoder/eventmanager/releases). 
### quick introduction of how you can use the prebuilt version
- download the .dmg file from the release tab
- open the dmg file move the application into your application folder
- run it!
## How to build

1. download the source code
2. open [app.xcodeproj](https://github.com/AkameTheCoder/eventmanager/tree/main/app/app.xcodeproj) with xcode
3. now we need to combine the x64 and arm library together.

- first approach, execute the shell script in which will combine these 2 librarys automatically for you (recommended if you dont wanna mess with the shell)
- second approach, we do it manually with lipo in a shell

### for the first approach
- navigate from the source code into the app folder, right click the folder named `rust_core` and click copy
- open a terminal and enter `cd` then press cmd + v on your keyboard to paste in the copied path, in my case it would look like this `cd /Users/maya/Desktop/app/rust_core/` press enter
- now enter in your terminal `chmod +x combine_libs.sh` in order to make the script executable
- run the script using the following command: `./combine_libs.sh`

*now we have successfully combined the libs to one lib, the script also removes the 2 input files `[libanvil_x86_64.a, libanvil_aarch64.a]`*

### for the second approach 
- navigate from the source code into the app folder, right click the folder named `rust_core` and click copy
- open a terminal and enter `cd` then press cmd + v on your keyboard to paste in the copied path, in my case it would look like this `cd /Users/maya/Desktop/app/rust_core/` press enter
- now paste in the following command into the terminal: `lipo -create -output libanvil.a libanvil_x86_64.a libanvil_aarch64.a`, now you combined the libs
- optionally you can delete the 2 libs `[libanvil_x86_64.a, libanvil_aarch64.a]`, but you dont have to

4. now you can run and compile the code with xcode
