#!/usr/bin/env

if [[ ${OS:-} = Windows_NT ]]; then
    if [ ! -d "/seek/bin" ]; then
        mkdir /seek
        mkdir /seek/bin
    fi
    curl https://github.com/yxshv/seek/releases/download/v0.1.0/windows.exe -o /seek/bin/seek.exe
    export PATH="$PATH;/seek/bin"
fi