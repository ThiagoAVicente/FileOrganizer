#!bin/sh
set -e

# build project
cargo build --release

BINARY_NAME="FileOrganizer"
RELEASE_PATH="target/release/$BINARY_NAME"
DEST="/usr/local/bin"

# copy the binary to the destination
sudo cp "$RELEASE_PATH" "$DEST/"

# add alias to .bashrc or .zshrc
message="FileOrganizer installed. Run [FileOrganizer -h] to see the help menu."
if [ -f "$HOME/.bashrc" ]; then
    echo "alias organize='$BINARY_NAME'" >> "$HOME/.bashrc"
    message="FileOrganizer installed. Alias organize=FileOrganizer was created. Run [organize -h] to see the help menu."
fi
if [ -f "$HOME/.zshrc" ]; then
    echo "alias organize='$BINARY_NAME'" >> "$HOME/.zshrc"
    message="FileOrganizer installed. Alias organize=FileOrganizer was created. Run [organize -h] to see the help menu."
fi
echo "$message"