#!/bin/bash

# Function to display help message
show_help() {
    echo "Usage: $0 [-i ICON] [-n NAME] [-h] script"
    echo "Options:"
    echo "  -i ICON    Specify an icon file for the executable."
    echo "  -n NAME    Specify a name for the executable."
    echo "  -h         Display this help message."
    echo "  script     The Python script to compile."
}

# Default values
ICON=""
NAME=""
SCRIPT=""

# Parse command-line options
while getopts ":i:n:h" opt; do
    case ${opt} in
        i )
            ICON=$OPTARG
            ;;
        n )
            NAME=$OPTARG
            ;;
        h )
            show_help
            exit 0
            ;;
        \? )
            echo "Invalid option: $OPTARG" 1>&2
            show_help
            exit 1
            ;;
        : )
            echo "Invalid option: $OPTARG requires an argument" 1>&2
            show_help
            exit 1
            ;;
    esac
done
shift $((OPTIND -1))

# Get the Python script to compile
SCRIPT=$1

if [[ -z "$SCRIPT" ]]; then
    echo "Error: No script specified." 1>&2
    show_help
    exit 1
fi

# Ensure PyInstaller is installed
echo "Installing PyInstaller..."
pip install pyinstaller

# Build the PyInstaller command
PYINSTALLER_CMD="pyinstaller --onefile"

# Add windowed option if the script uses a GUI
if grep -q 'tkinter' "$SCRIPT"; then
    PYINSTALLER_CMD+=" --windowed"
fi

# Add icon if specified
if [[ -n "$ICON" ]]; then
    PYINSTALLER_CMD+=" --icon=$ICON"
fi

# Add name if specified
if [[ -n "$NAME" ]]; then
    PYINSTALLER_CMD+=" --name=$NAME"
fi

# Add the script to the command
PYINSTALLER_CMD+=" $SCRIPT"

# Run PyInstaller
echo "Running PyInstaller..."
$PYINSTALLER_CMD

# Check if the build was successful
if [[ $? -ne 0 ]]; then
    echo "PyInstaller failed to create the executable." 1>&2
    exit 1
fi

echo "Executable created successfully in the dist directory."
