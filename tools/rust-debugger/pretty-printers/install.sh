#!/bin/bash

# Installs the MultiversX LLDB pretty-printer to the VS Code extensions directory.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PRETTY_PRINTER="$SCRIPT_DIR/multiversx_sc_lldb_pretty_printers.py"
DEST="$HOME/.vscode/extensions/multiversx_sc_lldb_pretty_printers.py"

cp "$PRETTY_PRINTER" "$DEST"
echo "Installed: $DEST"
