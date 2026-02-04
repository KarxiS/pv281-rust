#!/bin/bash

# Ensure the script is run with root privileges
if [ "$EUID" -ne 0 ]; then
    echo "❌ This script must be run as root. Please use sudo."
    exit 1
fi

# Check if libinput-tools is installed
if ! command -v libinput &> /dev/null; then
    echo "❌ 'libinput-tools' is not installed."
    echo "➡ Please install it using: sudo apt install libinput-tools"
    exit 1
fi

echo "Scanning for mice and keyboards..."
echo "------------------------------------"

# Use awk to parse the output of libinput list-devices
libinput list-devices | awk '
    # When a line starts with "Device:", store the device name
    /^Device:/ {
        # Remove "Device:" prefix and leading/trailing whitespace
        gsub(/^Device:\s*|\s*$/, "");
        device_name = $0;
    }
    # When a line starts with "Kernel:", store the device path
    /^Kernel:/ {
        # Remove "Kernel:" prefix and leading/trailing whitespace
        gsub(/^Kernel:\s*|\s*$/, "");
        device_path = $0;
    }
    # When a line starts with "Capabilities:", check for "pointer" or "keyboard"
    /^Capabilities:/ {
        if ($0 ~ /pointer/) {
            printf "✔ Found Mouse:\n";
            printf "  Name: %s\n", device_name;
            printf "  Path: %s\n\n", device_path;
        }
        if ($0 ~ /keyboard/) {
            printf "✔ Found Keyboard:\n";
            printf "  Name: %s\n", device_name;
            printf "  Path: %s\n\n", device_path;
        }
    }
'
  