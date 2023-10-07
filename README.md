# eventmanager
manage your personal tasks! 

# MacOS instructions
the following below will explain how you can build it yourself using macOS, a prebuilt app can be found in the [release tab](https://github.com/AkameTheCoder/eventmanager/releases).

### quick introduction of how you can use the prebuilt version
- download the .dmg file from the release tab
- open the dmg file move the application into your application folder
- run it!
## ~~How to build the app (MacOS)~~ there are errors right now with compiling in xcode, the issue is being investigated

1. download the source code
2. open [app.xcodeproj](https://github.com/AkameTheCoder/eventmanager/tree/main/app/app.xcodeproj) with [xcode](https://apps.apple.com/de/app/xcode/id497799835?l=en-GB&mt=12)
3. now we need to combine the x64 and arm library together.

We have now 2 methods to build our source code
- using the `build.sh` script which does anything for you basically
- building it manually (tuto coming later)


### for the first approach
- navigate into the source code using cd in a shell
- run the following command in a terminal: `chmod +x install.sh`
- run the install script: `./install.sh`
- follow the instructions in the script
- after installing you can build the source code anytime with `./build.sh`

4. now you can run and compile the code with xcode



