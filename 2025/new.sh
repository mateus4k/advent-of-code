#!/bin/bash

if [ -z "$1" ]; then
    echo "Error: Folder name must be provided as an argument."
    exit 1
fi

mkdir -p "src/$1"

touch "src/$1/__init__.py"

echo "Run with:
poetry run python src/$1/__init__.py"