#!/bin/env bash

set -euo pipefail

if [ $# -ne 2 ]; then
    # both input and output must reside under the current directory ( -v .:/app )
    echo -e "Converts a video file into OGV after applying a GRAY8 filter\nUsage: $0 <input> <output>"
    exit 1
fi

echo -e "\nBuilding (this might take a while) ..."
docker run --rm -v .:/app "$(docker build --target gst-dev -q .)" sh -c "cd /app; cargo build"

echo -e "\nRunning ..."
docker run --rm -v .:/app "$(docker build --target gst-base -q .)" sh -c "cd /app; target/debug/into-gray8-ogg --input \"$1\" --output \"$2\""
