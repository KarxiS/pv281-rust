#!/bin/bash

# Usage:
# chmod +x setup.sh
# sudo ./setup.sh

set -e

# Check if running as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo ./setup.sh)"
  exit 1
fi

TARGET_USER=${SUDO_USER:-$USER}
USER_HOME=$(getent passwd "$TARGET_USER" | cut -d: -f6)

echo "----------------------------------------------------------------"
echo "Starting setup for user: $TARGET_USER"
echo "----------------------------------------------------------------"

# ==========================================
# 1. Install System Dependencies for Tauri
# ==========================================
echo "Updating apt repositories..."
apt-get update

echo "Installing ALL system dependencies..."
# KOMPLETNY ZOZNAM (vratane WebKit 4.1 a JavaScriptCore 4.1)
apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pkg-config

# ==========================================
# 2. Install Rust and Tauri CLI (as User)
# ==========================================
echo "Checking Rust installation for user '$TARGET_USER'..."

sudo -u "$TARGET_USER" bash <<EOF
    # Load cargo env if exists
    if [ -f "$USER_HOME/.cargo/env" ]; then
        source "$USER_HOME/.cargo/env"
    fi

    # Install Rust if not present
    if ! command -v rustc &> /dev/null; then
        echo "Rust not found. Installing Rust (stable)..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$USER_HOME/.cargo/env"
    else
        echo "Rust is already installed."
    fi

    # Install Tauri CLI
    if ! command -v cargo-tauri &> /dev/null; then
        echo "Installing Tauri CLI (this may take a while)..."
        cargo install tauri-cli
    else
        echo "Tauri CLI is already installed."
    fi

    # GNOME Mouse Tweaks (No acceleration)
    echo "Applying GNOME mouse tweaks (flat profile, speed 0.2)..."
    gsettings set org.gnome.desktop.peripherals.mouse accel-profile 'flat'
    gsettings set org.gnome.desktop.peripherals.mouse speed 0.2
EOF

# ==========================================
# 3. Setup Permissions (Groups)
# ==========================================
echo "Setting up permissions..."

if ! getent group input > /dev/null; then
  groupadd input
fi
usermod -aG input "$TARGET_USER"

if getent group plugdev > /dev/null; then
    usermod -aG plugdev "$TARGET_USER"
fi

# ==========================================
# 4. Setup UDEV rules
# ==========================================
echo "Creating udev rules for input devices..."
cat > /etc/udev/rules.d/99-kmf-input.rules <<EOF
KERNEL=="uinput", SUBSYSTEM=="misc", MODE="0660", GROUP="input"
KERNEL=="event*", SUBSYSTEM=="input", MODE="0660", GROUP="input"
EOF

udevadm control --reload-rules
udevadm trigger

echo "----------------------------------------------------------------"
echo "Setup complete! All WebKitGTK 4.1 dependencies installed."
echo "Please LOG OUT and LOG BACK IN just to be safe."
echo "----------------------------------------------------------------"