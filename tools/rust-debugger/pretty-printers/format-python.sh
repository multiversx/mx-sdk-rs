#!/bin/bash

# Python code formatting script for MultiversX SDK pretty-printers
echo "🐍 Formatting Python code..."

# Get the SDK root directory (3 levels up from this script)
SDK_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PYTHON_CMD="$SDK_ROOT/.venv/bin/python"

# Find Python files in this directory
PYTHON_FILES=$(find . -name "*.py" -maxdepth 1)

if [ -z "$PYTHON_FILES" ]; then
    echo "No Python files found to format"
    exit 0
fi

echo "Found $(echo "$PYTHON_FILES" | wc -l) Python files to format"

# Check if virtual environment exists
if [ ! -f "$PYTHON_CMD" ]; then
    echo "❌ Python virtual environment not found at $PYTHON_CMD"
    echo "Please run 'configure_python_environment' from the SDK root first"
    exit 1
fi

# 1. Sort imports with isort
echo "📦 Sorting imports with isort..."
$PYTHON_CMD -m isort $PYTHON_FILES

# 2. Format code with black
echo "🖤 Formatting code with black..."
$PYTHON_CMD -m black $PYTHON_FILES

# 3. Check style with flake8 (optional)
if [ "$1" = "--check" ]; then
    echo "🔍 Checking style with flake8..."
    $PYTHON_CMD -m flake8 $PYTHON_FILES --max-line-length=88 --extend-ignore=E203,W503
fi

echo "✅ Python formatting complete!"