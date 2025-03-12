#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! command -v pbc &>/dev/null; then
  cargo install pbc
fi
pbc
rm -rf pb rust/proto__

echo -e '\npub use proto_tran::*;' >>rust/proto_tran/src/lib.rs

cd ./rust/proto_tran
touch Cargo.lock
rm Cargo.toml
ln -s ../../proto_tran.toml Cargo.toml
awk '!seen[$0]++' src/lib.rs | sponge src/lib.rs
