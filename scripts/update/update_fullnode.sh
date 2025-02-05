#!/bin/bash
set -e

VERSION=$1
CLEAN_DB=$2

# Access the Home directory where we have the PUI Folder
cd /home/pui

# Update the PUI Node
sudo git fetch
sudo git checkout $VERSION

# Build the PUI Node
sudo cargo build --release --bin sui-node

# Stop the PUI Node
sudo systemctl stop sui-node

if [ "$CLEAN_DB" = "true" ]; then
    sudo rm -rf /home/sui-db/
fi

# Start the PUI Node
sudo systemctl start sui-node
