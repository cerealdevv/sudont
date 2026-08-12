#!/bin/sh
set -e

MANPAGE="sudont.1"
SYSDIR="/usr/share/man/man1"
USRDIR="$HOME/.local/share/man/man1"

sdinstall() {
    install -Dm644 "$MANPAGE" "$1/sudont.1"
    echo "sudont: installed manpage to $1/sudont.1"
}

if [ "$(id -u)" -eq 0 ]; then
    sdinstall "$SYSDIR"
else
    echo "sudont: are you sure you want to install the sudont manpage to $SYSDIR? (requires sudo)"
    read -p "install manpage? [y/N] " confirm
    case "$confirm" in
        [yY]*)
            if sudo install -Dm644 "$MANPAGE" "$SYSDIR/sudont.1"; then
                echo "installed manpage to $SYSDIR/sudont.1"
            else
                echo "sudont: falling back to local install"
                mkdir -p "$USRDIR"
                sdinstall "$USRDIR"
            fi
            ;;
        *)
            mkdir -p "$USRDIR"
            sdinstall "$USRDIR"
            echo "sudont: add 'export MANPATH=\"\$HOME/.local/share/man:\$MANPATH\"' to your shell rc to use the manpage"
            ;;
    esac
fi
