#!/usr/bin/env bash

buildLinux() {
    echo Linux
    cd docker
    docker build -t gtk3-rs-linux `pwd` --file=`pwd`/Dockerfile-linux-build-container
    cd ..
    docker run -it --rm -v `pwd`:/project gtk3-rs-linux cargo build --target-dir target/linux-`uname -i`/ --release
    sudo mkdir -p /opt/pgmodeler-docker-x11/bin
    sudo cp target/linux-`uname -i`/release/launcher /opt/pgmodeler-docker-x11/bin/
    sudo ln -s /opt/pgmodeler-docker-x11/bin/launcher /usr/bin/pgmodeler-docker-x11
    sudo cp resources/pgmodeler_logo.png /opt/pgmodeler-docker-x11/
    sudo cp resources/pgmodeler.desktop /usr/share/applications/pgmodeler-docker-x11.desktop
    sudo chmod +x /usr/share/applications/pgmodeler-docker-x11.desktop
}

buildMac() {
    echo Mac
    cd docker
    docker build -t osxcross:10.8 `pwd` --file=`pwd`/Dockerfile-macos-osxcross
    docker build -t gtk3-rs-macos `pwd` --file=`pwd`/Dockerfile-macos-build-container
    cd ..
    docker run -it --rm -v `pwd`:/project gtk3-rs-macos cargo build --target-dir target/macos-x86_64/ --release --target=x86_64-apple-darwin
}

#buildWindows() {
#    echo Windows
#    echo "Nothing here Jim"
#}

unknown() {
    echo "Unsupported platform ${1}"
    echo "Must be one of the following: linux or mac (windows to be supported soon)"
}

case "${1}" in
    linux*)     buildLinux;;
    mac*)       buildMac;;
#    windows*)   buildWindows;;
    windows*)   unknown;;
    *)          unknown
esac
