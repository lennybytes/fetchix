# FETCHIX 🦀

A lightweight command-line system information tool written in **Rust**.

Fetchix displays useful system information in a simple and minimal format. It is a rewrite of the original C++ implementation.

## Features

- 🐧 OS / distribution name detection (read from `/etc/os-release`)
- 🧩 Kernel version detection (read from `/proc/sys/kernel/osrelease`)
- 🌐 Local IP address (via the [`local-ip-address`](https://crates.io/crates/local-ip-address) crate)
- 🐚 Shell detection (via the `SHELL` environment variable)
- ⚡ Fast and lightweight — only one external dependency
- 🦀 Written entirely in Rust

## Requirements

- Rust toolchain with **edition 2024** support (Rust 1.85+)
- Cargo
- Linux (OS and kernel information are read from `/etc/os-release` and `/proc`)

You can check whether Rust and Cargo are installed with:

```bash
rustc --version
cargo --version
```

## Building

Clone the repository:

```bash
git clone https://github.com/lennybytes/fetchix.git
cd fetchix
```

Build Fetchix with Cargo:

```bash
cargo build --release
```

The compiled binary will be located at:

```text
target/release/fetchix
```

You can also run Fetchix directly with:

```bash
cargo run --release
```

## Installing

Build the release binary:

```bash
./build.sh
```

The `doas-build.sh` script is also available for symmetry with `build.sh`; building itself does not require elevated privileges.

Install the binary to `/usr/local/bin/fetchix`:

```bash
sudo cp target/release/fetchix /usr/local/bin/fetchix
```

## Usage

After building:

```bash
./target/release/fetchix
```

Example:

```text
OS:       Arch Linux
Kernel:   6.18.55-0-lts
Local IP: 192.168.1.100
Shell:    /bin/bash
```

The exact output depends on your system. If a value cannot be determined, Fetchix falls back to a placeholder such as `Unknown` or `Not connected`.

## Project Structure

```text
fetchix/
├── src/
│   └── main.rs               # Entry point
├── Cargo.toml                # Package manifest
├── Cargo.lock                # Dependency lock file
├── build.sh                  # Build release binary
├── doas-build.sh             # Build release binary (doas variant)
└── README.md                 # This file
```

## Rust Rewrite

Fetchix originally started as a **C++ project**.

The project has since been completely rewritten in Rust. The goal of the rewrite is to make the codebase easier to maintain while keeping Fetchix small, fast, and simple. The current implementation lives on the `main` branch.

## Supported Systems

Fetchix is primarily developed and tested on Linux.

| Operating System | Status |
|---|---|
| Linux | ✅ Supported |
| macOS | ⚠️ Local IP and shell only — OS/kernel info require `/etc/os-release` and `/proc` |
| BSD | ⚠️ Local IP and shell only — OS/kernel info require `/etc/os-release` and `/proc` |
| Windows | ❌ Not currently supported |

## Development

Clone the repository:

```bash
git clone https://github.com/lennybytes/fetchix.git
cd fetchix
```

Run the development version:

```bash
cargo run
```

Check the project:

```bash
cargo check
```

Build an optimized binary:

```bash
cargo build --release
```

## License

This project is licensed under the **MIT License**.

```text
MIT License

Copyright (c) 2026 Lennybytes

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Author

**Lennybytes**

GitHub: https://github.com/lennybytes

---

**FETCHIX** — simple system information, written in Rust. 🦀🐧
