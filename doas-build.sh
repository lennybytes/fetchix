#!/bin/bash
set -euo pipefail

cargo build --release
echo "Built fetchix"
