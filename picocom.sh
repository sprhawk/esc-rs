#!/bin/sh

PORT="0"

if [ ! -z $1 ]; then
    PORT=$1
fi
        
picocom -b 115200 /dev/ttyACM$PORT
