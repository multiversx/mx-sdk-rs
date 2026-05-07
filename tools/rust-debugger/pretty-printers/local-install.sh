#!/bin/bash
set -e

# Installs the MultiversX LLDB pretty-printer to the VS Code extensions directory.
#
# The pretty-printer is picked up automatically by the CodeLLDB extension via the
# "sourceMap" / init-commands mechanism configured in .vscode/launch.json.  Running
# this script is the quickest way to test local edits to the .py file without going
# through `cargo run install debugger` or publishing a new crate version.
#
# Usage:
#   ./tools/rust-debugger/pretty-printers/local-install.sh
#
# After running, restart any active debug session so LLDB reloads the script.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PRETTY_PRINTER="$SCRIPT_DIR/multiversx_sc_lldb_pretty_printers.py"
DEST_DIR="$HOME/.vscode/extensions"
DEST="$DEST_DIR/multiversx_sc_lldb_pretty_printers.py"

mkdir -p "$DEST_DIR"
cp "$PRETTY_PRINTER" "$DEST"
echo "Installed: $DEST"
