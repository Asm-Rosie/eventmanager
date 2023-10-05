# eventmanager
manage your personal tasks! 

# MacOS instructions
the following below will explain how you can build it yourself using macOS, a prebuilt app can be found in the [release tab](https://github.com/AkameTheCoder/eventmanager/releases).

### quick introduction of how you can use the prebuilt version
- download the .dmg file from the release tab
- open the dmg file move the application into your application folder
- run it!
## How to build the app(MacOS)

1. download the source code
2. open [app.xcodeproj](https://github.com/AkameTheCoder/eventmanager/tree/main/app/app.xcodeproj) with [xcode](https://apps.apple.com/de/app/xcode/id497799835?l=en-GB&mt=12)
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

## How to build the rust core for the app

1. Install [rust](https://www.rust-lang.org/tools/install) here, and follow the instructions provided by the website
2. verify if cargo is installed by running in a shell `cargo -V`
3. navigate with a terminal into the source code of the rust core `cd eventmanager-main/rust_core`
4. in the current case we have to build the rust core according to apples specific requirements for a macOS application, which means we are forced to build for x64 and aarch64
5. thats how we can do it, first compile the library source code for x64-darwin, you can do this by using this command in a shell: `cargo build --target x86_64-apple-darwin
6. now build the library source code for apple silicon using the following command: `cargo build --target aarch64-apple-darwin`
7. merge the 2 C compatible libraries together using this command: `lipo -create -output libanvil.a target/x86_64-apple-darwin/debug/libanvil.a target/aarch64-apple-darwin/debug/libanvil.a`
8. Now we have successfully combined both architectures apple requires into one library! The library can be found in rust_core/libanvil.a

### how to use the library in xcode (with my source code of the swiftui application)
1. if you use my source code for the swiftui macOS app, you will find in the main directory of the project a folder called `rust_core`
2. navigate to this folder using the finder
3. replace the libanvil.a file with your new libanvil.a file

### in case you added new functions in the library and want to use these new function now in xcode
first of all, I am assuming that you have already done the steps shown above 

it would be too much time effort to explain now anything by myself if there is already a [repo](https://github.com/thombles/dw2019rust/blob/master/modules/04%20-%20Build%20automation.md) explaining it pretty well how to use external rust libraries with swift

also credits to [@thombles](https://github.com/thombles) for giving me the idea and inspiration to start this project!
