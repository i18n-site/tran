#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ -n "$1" ]; then
  export PROJECT=$1
else
  echo "USAGE : $0 project_name"
  exit 1
fi

git add -u && git commit -m. || true
cd $PROJECT
rm -f Cargo.lock
touch Cargo.lock
cargo v patch -y
git add -u && git commit -m. || true
cargo publish --registry crates-io --allow-dirty
