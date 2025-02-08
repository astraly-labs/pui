#!/bin/bash

# Enable debug mode if DEBUG environment variable is set
if [ "${DEBUG:-0}" = "1" ]; then
    set -x
fi

if [ $# -ne 2 ]; then
    echo "Usage: $0 <keypair> <alias>"
    exit 1
fi

KEYPAIR=$1
ALIAS=$2

echo "Converting keypair: $KEYPAIR"
CONVERT_CMD="./target/release/sui keytool convert \"$KEYPAIR\""
echo "Executing: $CONVERT_CMD"

# First convert the keypair
CONVERT_OUTPUT=$(./target/release/sui keytool convert "$KEYPAIR" 2>&1)
CONVERT_STATUS=$?

if [ $CONVERT_STATUS -ne 0 ]; then
    echo "Error: Command failed with status $CONVERT_STATUS"
    echo "Command output:"
    echo "$CONVERT_OUTPUT"
    exit 1
fi

# Print full output for debugging
echo "Full convert output:"
echo "$CONVERT_OUTPUT"

# Extract values and trim all whitespace
CONVERTED=$(echo "$CONVERT_OUTPUT" | grep "bech32WithFlag" | sed 's/.*│.*│[[:space:]]*\(.*\)[[:space:]]*│/\1/' | tr -d '[:space:]')
KEY_SCHEME=$(echo "$CONVERT_OUTPUT" | grep "scheme" | sed 's/.*│.*│[[:space:]]*\(.*\)[[:space:]]*│/\1/' | tr -d '[:space:]')

if [ -z "$CONVERTED" ] || [ -z "$KEY_SCHEME" ]; then
    echo "Error: Failed to extract key information"
    echo "Command output:"
    echo "$CONVERT_OUTPUT"
    exit 1
fi

echo "Successfully converted keypair"
echo "Using key: $CONVERTED"
echo "Using scheme: $KEY_SCHEME"

# Now use the converted key to get the address with the provided alias
IMPORT_CMD="./target/release/sui keytool import --alias \"$ALIAS\" \"$CONVERTED\" \"$KEY_SCHEME\""
echo "Executing: $IMPORT_CMD"

IMPORT_OUTPUT=$(./target/release/sui keytool import --alias "$ALIAS" "$CONVERTED" "$KEY_SCHEME" 2>&1)
IMPORT_STATUS=$?

# Print full import output for debugging
echo "Import command output:"
echo "$IMPORT_OUTPUT"

if [ $IMPORT_STATUS -ne 0 ]; then
    echo "Error: Import command failed with status $IMPORT_STATUS"
    echo "Command output:"
    echo "$IMPORT_OUTPUT"
    exit 1
fi

# Extract address from the table output similar to how we extracted the key
ADDRESS=$(echo "$IMPORT_OUTPUT" | grep "suiAddress" | sed 's/.*│.*│[[:space:]]*\(.*\)[[:space:]]*│/\1/' | tr -d '[:space:]')

if [ -n "$ADDRESS" ]; then
    echo "Successfully obtained address:"
    echo "$ADDRESS"
else
    echo "Error: Failed to get address from keypair"
    echo "Command output:"
    echo "$IMPORT_OUTPUT"
    exit 1
fi