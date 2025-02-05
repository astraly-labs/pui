#!/bin/bash
set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Version parameter required"
    exit 1
fi

# Access the Home directory where we have the PUI Folder
cd /home/pui

# Update the PUI Node
sudo git fetch
sudo git checkout $VERSION

# Build the PUI Node
sudo cargo build --release --bin sui-node

# Stop the PUI Node
sudo systemctl stop sui-node

# Start the PUI Node
sudo systemctl start sui-node
