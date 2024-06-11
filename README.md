# ledmatrix_widgets

A rust application for creating and displaying widgets on the Framework 16 LED Matrix modules.

### Current Widgets
- Current battery life
- CPU usage per-core

### Future Additions
- RAM usage
- Disk size
- Network traffic
- Overall CPU usage
- Customize position with application parameters
- Customize refresh rate
- JSON Configuration file

### Installation
Head over to the Releases tab and download for either Ubuntu/Debian (.deb), Fedora (.rpm), Arch (.pkg.tar.xz) or Windows (.msi). 
If you want to run locally, clone this repo and follow the build instructions below.

Or, you can download from [the releases tab](https://github.com/superrm11/ledmatrix_widgets/releases)

Note - these installers will only install the executable and add it to your path - you can only run by running the command `ledmatrix_widgets`.
There are plans to package a .desktop / systemd unit file, and add a shortcut to Startup for Windows, but that will have to wait for 
future releases.

### Build Instructions

Prereqs:
```
cargo

# Arch
systemd-libs

# Ubuntu/Debian
libudev-dev
pkg-config

# Fedora
systemd-devel
```
In the root directory, run `cargo build` or `cargo run`. This project is cross platform, and works with both Windows and Linux.
