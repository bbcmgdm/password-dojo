#!/usr/bin/env bash

if [ -z "$1" ]; then
    1>&2 echo "Usage: $(basename "$0") PASSWORD_LIST"
    exit 1
fi

while IFS= read -r line
do
    IFS=, read -r username pw_hash <<< "$line"

    pw=$(./target/release/reverse "$pw_hash")

    echo "Password for '$username' with hash '$pw_hash' is '$pw'"
done < <(cat "$1")