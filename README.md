# eventmanager
manage your personal tasks! 


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

*now we have successfully combined the libs to one lib*
