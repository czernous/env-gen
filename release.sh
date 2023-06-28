#!/bin/bash

# x86_64-apple-darwin
BINARY_NAME="$1"
TARGETS="x86_64-unknown-linux-gnu x86_64-pc-windows-gnu"

if [ -z "$BINARY_NAME" ]; then
  echo "Please provide the binary name as an argument."
  exit 1
fi

for target in $TARGETS; do
  cargo build --release --target $target &&
  if [[ $target == "x86_64-unknown-linux-gnu" ]]; then
    strip target/$target/release/$BINARY_NAME
  elif [[ $target == "x86_64-pc-windows-gnu" ]]; then
    strip target/$target/release/$BINARY_NAME.exe
  fi &
done
wait
