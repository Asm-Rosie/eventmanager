# eventmanager
manage your personal tasks! 

# IMPORTANT!
I have acknowleged that the approaches for making the UI were unefficent and definetly not needed for such an simple application, I apologise for making this project harder as it should be. 

to make the contributions friendlier and easier, I am right now working to transfer the UI to electron which would make it far easier to compiling for any platform seamlessly. 

the core functions will be kept and maintained with the same techniques. it might happen that the rust core will be rather renamed to "core" due of the fact it might happen that other languages for example C and C++ will be integrated.



# MacOS instructions
the following below will explain how you can build it yourself using macOS, a prebuilt app can be found in the [release tab](https://github.com/AkameTheCoder/eventmanager/releases).

### quick introduction of how you can use the prebuilt version
- download the .dmg file from the release tab
- open the dmg file move the application into your application folder
- run it!
## How to build the app

1. download the source code
2. open [app.xcodeproj](https://github.com/AkameTheCoder/eventmanager/tree/main/app/app.xcodeproj) with [xcode](https://apps.apple.com/de/app/xcode/id497799835?l=en-GB&mt=12)
3. now we need to combine the x64 and arm library together.

We have now 2 methods to build our source code
- using the `build.sh` script which does anything for you basically
- building it manually (tuto coming later)


### for the first approach
- open a terminal and clone the repo `git clone https://github.com/AkameTheCoder/eventmanager.git`
- navigate into the repo using following command: `cd eventmanager`
- run the following command in a terminal to make the install script executable: `chmod +x install.sh`
- run the install script: `./install.sh`
- follow the instructions in the script
- after installing you can build the source code anytime with `./build.sh`

4. now you can run and compile the code with [xcode](https://apps.apple.com/de/app/xcode/id497799835?l=en-GB&mt=12) by running in the terminal following command

   `cd eventmanager`
   
   `open eventmanager.xcodeproj`

# Windows Instructions

## How to build

- Download and install (QT)[https://www.qt.io/download-open-source] and the C++ [VS Build Tools](https://aka.ms/vs/17/release/vs_BuildTools.exe). 
- clone the repo `git clone https://github.com/AkameTheCoder/eventmanager.git`
- navigate into the repo using following command: `cd eventmanager`
- run the installer script with `install.ps1`
- if you dont get prompted in the shell script to run the build script after you have installed anything successfully run the build script using this command `./build.ps1`
- after that, open QT Creator, and open the project which is located in `qt-eventmanager`, be sure to configure the project with the msvc compiler
- hit the run button and your application will be build



