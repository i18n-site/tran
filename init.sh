#!/usr/bin/env bash

set -e
DIR=$(dirname "${BASH_SOURCE[0]}")
set -x

if [ -d "sh" ]; then
  cd sh
  git pull
  cd ..
else
  git clone -b dev --depth=1 git@github.com:i18n-site/cargo_sh.git sh
fi
rm -f mise.toml .mise.toml
ln -s sh/_mise.toml .mise.toml
mise trust || true
rm $0
git add .
