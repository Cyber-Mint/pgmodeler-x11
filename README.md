# pgModeler-X11 (X11 in Docker)
> Forked from fossabot/pgmodeler-x11 2019

## What's this?
Build and run the latest version of [pgModeler](https://pgmodeler.io/) inside a  Docker container on Linux (& macOS).


> This project is also an example of creating a build environment that does
> cross-platform UI compilation from inside Docker, compiling using GTK and
> Rust. i.e. it is a mechanism to build Linux and macOS binaries in Docker.
>   
> Maitainers:
> ===========
> -- https://github.com/mvniekerk <br>
> -- https://github.com/bank-builder

## Quickstart

Use the [docker-compose](./docker-compose.yml) found here to spin up a self contained pgmodeler and database for design purposes.

```
# add exports to ~/.bashrc
export USERID=$(id -u)
export WORKING_DIR=$(pwd)
docker-compose up -d
# and cleanup nicely when done
docker-compose down --remove-orphans
```

## Why?

* Because building pgModeler is harder than it should be.

* Because most distros are behind in their pgModeler version and/or don't have an easy way of getting the latest release - this gives another docker-based option to easily get a modern up-to-date pgModeler version. 

* This allows you to easily switch between different pgModeler versions without clobbering your config directory. 
  
Change the `PGMODELER_VERSION` in the container to build another version to be run, add `PGMODELER_VERSION` as an environment variable when running the launcher image to start a different run image.

* Because having a base Docker pgModeler image to be shared between users on all platforms helps address the security concerns raised by running binaries others have built. A company (such as a bank) can compile its own image and can audit how it was built and how it was stored - banks are funny that way!

* Because having a reproducible way of building GTK applications for the three major desktop platforms using Docker and Rust rocks!

## About pgModeler and commercial support
pgModeler [provides pre-compiled binaries at a small price](https://pgmodeler.io/download?purchase=true).
While we are providing an easy way to build and run pgModeler, we will not be changing the scripts and images provided to build *stable* versions (only *development* versions) - this is so as not to erode their (paying) customer base.

We will also not be providing support. 

Further, as you're essentially running pgModeler on Linux, it will look and feel like Linux. Which is nice if you're a Linux user, not so great if you're a Mac or Windows user - kindly buy the prebuilt binaries from the pgModeler project if this is a problem.

## macOS Packaging
[ Please ensure you have read and understood the Xcode license terms before continuing.](https://www.apple.com/legal/sla/docs/xcode.pdf)

**Building for macOS should be done on Apple hardware!**

The launcher compiles and links inside a container that has [Macports](https://www.macports.org/) installed.  The binary is `target/macos-x86_64/x86_64-apple-darwin/release/launcher`.  In order to be able to run this, you need to install gtk3 using either Brew or Macports. This will change in the future as we aim to make an application launcher to be put in your `/Applications` directory. 

[TODO implement https://wiki.gnome.org/Projects/GTK/OSX/Bundling] 


## Prerequisites
### Linux
* Docker

### macOS
* Docker
* XQuartz

## Building the container

In the `container` directory, run `./build.sh`.

This will create an image: `pgmodeler-docker-x11/run`.

This will take ~15 minutes.

## Build and install the launcher
**\*\* Currently Linux Only \*\***

In the `launcher` directory, run `./build.sh <platform>` where platform is linux, mac.

Only Linux actually installs it now (for now). For macOs the binary is built but then needs to be run from the command line and have gtk3 installed (see "macOS Packaging").

When the Linux build finishes, it will create a .desktop entry in your /usr/share/applications/ directory, which means you can run pgModeler from your application launcher.

## Run without the launcher
The launcher looks at your environment and then runs the command below and lets you know if something goes wrong. 

But nothing is keeping you from doing it yourself.
**NOTE:**  this can/will clobber your config if you're not using 0.9.2-alpha1.
### Linux
`docker run --rm -it --user $(id -u) -e DISPLAY=unix$DISPLAY --workdir=$(pwd) --volume="/home/$USER:/home/$USER" --volume="/etc/group:/etc/group:ro" --volume="/etc/passwd:/etc/passwd:ro" --volume="/etc/shadow:/etc/shadow:ro" --volume="/etc/sudoers.d:/etc/sudoers.d:ro" -v /tmp/.X11-unix:/tmp/.X11-unix cybermint/pgmodeler`

### macOS
You need XQuartz installed, and 'Allow connections from network clients' selected in the X11 preferences.

Run the following two commands:

```bash
xhost + 127.0.0.1 
```

```bash
docker run --rm -it -e DISPLAY=host.docker.internal:0 --workdir=$(pwd) -v /Users/{your_user_folder}:/home/{your_user_folder} --volume="/etc/group:/etc/group:ro" --volume="/etc/passwd:/etc/passwd:ro" --volume="/etc/sudoers.d:/etc/sudoers.d:ro" -v /tmp/.X11-unix:/tmp/.X11-unix -v $(pwd):$(pwd) cybermint/pgmodeler
```

***Note**: Replace {your_user_folder} with the literal name of your home folder. Do not use the $USER variable.*


---
&copy; 2019, Grindrod Bank Limited.
&copy; 2020, Cyber-Mint (Pty) Ltd
