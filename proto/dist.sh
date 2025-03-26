#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

bun x mdt .
. ./gen.sh
cp ../../README.md .
git add -u
cargo v patch -y
git add -u && git commit -m. && git push || true
cargo publish --registry crates-io --allow-dirty
