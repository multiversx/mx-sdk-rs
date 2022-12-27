#!/bin/sh

ROOT_PATH=${1:?"Missing path!"}

find $ROOT_PATH -type d -depth -execdir mv mandos scenarios 2>/dev/null \;
