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
    rm -rf /opt/sui/db/authorities_db /opt/sui/db/consensus_db
fi

# Copy the new binary to the target directory
sudo mv ./target/release/sui-node /opt/sui/bin/

# Start the PUI Node
sudo systemctl start sui-node
