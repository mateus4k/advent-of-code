#!/bin/bash

if [ -z "$1" ]; then
    echo "Error: Folder name must be provided as an argument."
    exit 1
fi

cp -r "template" "$1"
