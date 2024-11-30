#!/bin/bash

if [ "$1" == "create" ]; then
    read -p "Insert new folder: " folder_name
    cp -r "templates" $folder_name
else
    echo "Unknown command"
    exit 1
fi
