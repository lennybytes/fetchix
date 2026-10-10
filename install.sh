#!/bin/bash
set -euo pipefail

sudo install -Dm755 ./target/release/fetchix /usr/local/bin/fetchix
echo "Installed fetchix to /usr/local/bin/fetchix"
