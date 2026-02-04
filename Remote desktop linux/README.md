# KMF Driver

Mouse and keyboard sharing between multiple Linux PCs over network.
### Main description
Design a custom protocol and Linux driver to allow mouse and keyboard use between multiple PCs. The keyboard is active on the PC where the mouse cursor is currently. At the same time, when dragging a file, it allows to move/copy that file to another PC.

## Team

- **Jakub K.**
- **Karol Š.**
- **Patrik H.**
- **Filip P.**


## Features

- Share mouse and keyboard across multiple PCs
- File drag & drop between computers
- Network communication between devices

## Requirements

- Linux (tested on NixOS), or VM ubuntu,neoos..etc
- Rust 1.70+
- Root privileges

## NixOS Setup

This project includes Nix flake configuration for easy setup on NixOS:

```bash
# Using flake.nix and flake.lock
nix develop
```

The flake provides all necessary dependencies and development environment.
# DRIVER documentation 
## Finding Input Devices - NeoOS, Linux
If you use VM, scroll down to "Finding Input Devices in virtual machines"

Your mouse and keyboard are located in `/dev/input/by-id/`.

**To list available devices:**
```bash
ls -la /dev/input/by-id/
```

You see multiple virtual devices per one psychical device (e.g., one mouse multiple devices, you need to look at:
- `<device-name>-event-mouse` - for mouse
- `<device-name>-event-kbd` - for keyboard

**Example output:**
```
lrwxrwxrwx 1 root root ... usb-Razer_Razer_DeathAdder_V2-event-mouse -> ../event5
lrwxrwxrwx 1 root root ... usb-Logitech_USB_Keyboard-event-kbd -> ../event3
``` 
## Finding Input Devices - Virtual Machine

in virtual machine its different, in VMware you have it grouped up and keyboard is missing.

to get keyboard and mouse in one command, you need to use library **sudo evtest**
```
ksugar@ksugar-virtual-machine:~$ sudo evtest
No device specified, trying to scan all of /dev/input/event*
Available devices:
/dev/input/event0:	Power Button
/dev/input/event1:	AT Translated Set 2 keyboard
/dev/input/event2:	VirtualPS/2 VMware VMMouse
/dev/input/event3:	VirtualPS/2 VMware VMMouse
/dev/input/event4:	VMware VMware Virtual USB Mouse
/dev/input/event5:	VMware DnD UInput pointer
Select the device event number [0-5]: 6
```
Now we see that we have 6 devices, the VMWare Virtual USB Mouse(**event4**)) is the one we want 

and AT Translated Set 2 keyboard is the one we want for keyboard(**event4**).

also because of how VMware works, we need to set gaming policy in VMWare settings.
```
# 1. Turnoff VM
# 2. search .vmx file (napr. ~/vmware/Ubuntu/Ubuntu.vmx)
# 3. open and put at the end mks.gamingMouse.policy = "gaming"
```




**Press CTRL-C or invalid number to cancel scanning.**
## Build

```bash
cargo build --release
```

## Usage

### Basic run:

```bash
cargo build --release && sudo ./target/release/kmf-driver --mouse /dev/input/by-id/<mouse-device> --keyboard /dev/input/by-id/<keyboard-device>
```

### Example run:

```bash
cargo build --release -p kmf-master && sudo ./target/release/kmf-master --mouse /dev/input/event4 --keyboard /dev/input/event1
```
### Alternative without parameters:

```bash
cargo build --release &&
sudo ./target/release/kmf-cli
```

This will start the program without capturing any devices (useful for testing network/cursor features).

## Development

### Git Hooks Setup (Automatic)

This project uses **cargo-husky** for automatic git hooks management.

**Initial setup:**
```bash
# Build project (automatically installs git hooks)
cargo build
```

Git hooks will automatically:
- ✅ Validate commit message format (conventional commits)
- ✅ Run `cargo fmt --check` before commits
- ✅ Run `cargo clippy` before commits

**Windows/linux users:** If hooks aren't working, run:
```powershell
git update-index --chmod=+x .cargo-husky/hooks/commit-msg
git update-index --chmod=+x .cargo-husky/hooks/pre-commit
```

### Manual Development Workflow

If you want to check manually before committing:

### Check code

```bash
cargo check
```
### Format code
```bash
cargo fmt
```
### Run tests:

```bash
cargo test
```

## Git Workflow

```bash
# Create feature branch
git checkout -b feature/your-feature

# Commit changes with conventional commit format
git add .
git commit -m "feat(Scripts): Add mouse cursor tracking"
# or
git commit -m "fix: Handle keyboard event buffer overflow"
# or  
git commit -m "chore: Update dependencies"


# Push
git push -u origin feature/your-feature

# Create Merge Request on GitLab
```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types:

* **feat:** New feature
* **fix:** Bug fix
* **chore:** Maintenance (dependencies, configs)
* **docs:** Documentation changes
* **refactor:** Code refactoring
* **test**: Adding or updating tests
* **ci:** CI/CD changes


## Dependencies #TODO


## Troubleshooting

### Permission denied

```bash
# Make sure you run with sudo
sudo cargo run -- --mouse /dev/input/... --keyboard /dev/input/...
```

### Device not found in other distros

```bash
# List all input devices - this Linux way
ls -la /dev/input/by-id/

# Make sure device is connected
# Look for -event-mouse and -event-kbd suffixes
```

### Previous troubleshoot isn't working / Can't find correct device

```bash
# Use evtest to identify devices - personally recommend this K.
sudo evtest
# then just cancel menu with CTRL-C or invalid number
```
### Automatic script to find devices

```bash
sudo ./scripts/detect-peripherals.sh
```

---
"Your mouse crosses borders. Your files follow. Your sanity stays behind." - Made up

## Usage

### Commands

Available commands in server mode:

- `move <x> <y>` - Send mouse move event
- `click <left|right|middle> <down|up>` - Send mouse click event
- `key <key> <down|up>` - Send keyboard event
- `file <path>` - Transfer file to all clients
- `quit` - Disconnect all clients


### Handshake Flow

1. Client connects to server
2. Client sends `ServerHello(config)` with screen info
3. Server stores client info and waits for commands
4. Server broadcasts `Action` or `File` messages
5. Clients respond with `Ok` or `Err`