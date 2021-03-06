# Reverse Beacon Network CLI 
[![Crates.io](https://img.shields.io/crates/v/rbn)](https://crates.io/crates/rbn) ![Build](https://github.com/Ewpratten/rbn/workflows/Build/badge.svg)

`rbn` is a small CLI interface to the [Reverse Beacon Network](https://reversebeacon.net), displaying all global network spots in real time as a formatted feed in a terminal.

## Installation

### From Source

```sh
git clone https://github.com/ewpratten/rbn
cd rbn
cargo install --path .
```

### From Crates.io

```sh
cargo install rbn
```

### Pre-built Binaries

I share a few pre-built binaries for systems I use on the [releases](https://github.com/Ewpratten/rbn/releases/latest) page.

## Usage

```
# rbn --help

Reverse Beacon Network Client 0.1.2
Evan Pratten <ewpratten@gmail.com>

USAGE:
    rbn [OPTIONS] --callsign <callsign>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --band <band>...             Band name to filter by. This can be used multiple times to filter multiple bands
    -c, --callsign <callsign>        Your callsign (used to authenticate with RBN)
    -f, --filtercall <filtercall>    Callsign to filter by

```

## Screenshots

![Screenshot](./screenshot.png)
