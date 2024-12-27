#!/bin/sh
set -e

if [[ $# == 1 ]]; then
    PATH_TO_BINARY="$1"
    shift

    rm -rf touchHLE.app
    mkdir -p touchHLE.app/Contents/MacOS touchHLE.app/Contents/Resources
    cp $PATH_TO_BINARY touchHLE.app/Contents/MacOS/touchHLE
    cp -r ../touchHLE_dylibs touchHLE.app/Contents/Resources/
    cp -r ../touchHLE_fonts touchHLE.app/Contents/Resources/
    cp -r ../touchHLE_default_options.txt touchHLE.app/Contents/Resources/
else
    echo "Incorrect usage."
    exit 1
fi
