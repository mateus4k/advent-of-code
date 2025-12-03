#!/bin/bash

if [ -z "$1" ]; then
    echo "Error: Folder name must be provided as an argument."
    exit 1
fi

cp -r "src/template" "src/$1"

echo "Run with:
poetry run python3 -m src.$1"
