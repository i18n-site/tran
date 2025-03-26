#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

rm -rf ../test/.i18n/hash
exec cargo run -- -w ../test
